use std::collections::{BTreeMap, BTreeSet, HashMap};

use gear_mesh_core::{GearMeshType, TypeKind, TypeRef, is_builtin_type, is_internal_type};

/// Output organization strategy for generated TypeScript files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleStrategy {
    /// Emit every generated type into a single file.
    SingleFile,
    /// Emit one file per type under `models/`.
    PerType,
    /// Group types by namespace segments in their names.
    ByNamespace { separator: String },
}

impl Default for ModuleStrategy {
    fn default() -> Self {
        Self::SingleFile
    }
}

/// Computes module groupings and inter-module imports.
#[derive(Debug, Default)]
pub struct ModuleOrganizer {
    dependency_graph: HashMap<String, BTreeSet<String>>,
}

impl ModuleOrganizer {
    pub fn new(types: &[GearMeshType]) -> Self {
        let dependency_graph = types
            .iter()
            .map(|ty| (ty.name.clone(), extract_type_dependencies(ty)))
            .collect();

        Self { dependency_graph }
    }

    pub fn organize(
        &self,
        types: &[GearMeshType],
        strategy: &ModuleStrategy,
    ) -> BTreeMap<String, Vec<GearMeshType>> {
        let mut modules = BTreeMap::new();

        match strategy {
            ModuleStrategy::SingleFile => {
                modules.insert("index.ts".to_string(), types.to_vec());
            }
            ModuleStrategy::PerType => {
                for ty in types {
                    modules.insert(
                        format!("models/{}.ts", file_stem(&ty.name)),
                        vec![ty.clone()],
                    );
                }
            }
            ModuleStrategy::ByNamespace { separator } => {
                for ty in types {
                    let path = namespace_path(&ty.name, separator);
                    modules
                        .entry(path)
                        .or_insert_with(Vec::new)
                        .push(ty.clone());
                }
            }
        }

        modules
    }

    pub fn build_type_index(
        &self,
        modules: &BTreeMap<String, Vec<GearMeshType>>,
    ) -> HashMap<String, String> {
        let mut index = HashMap::new();

        for (module, types) in modules {
            for ty in types {
                index.insert(ty.name.clone(), module.clone());
            }
        }

        index
    }

    pub fn generate_imports(
        &self,
        module: &str,
        types: &[GearMeshType],
        type_index: &HashMap<String, String>,
        include_zod: bool,
    ) -> Vec<String> {
        let local_names: BTreeSet<_> = types.iter().map(|ty| ty.name.as_str()).collect();
        let mut imports = BTreeMap::<String, BTreeSet<String>>::new();

        for ty in types {
            let Some(deps) = self.dependency_graph.get(&ty.name) else {
                continue;
            };

            for dep in deps {
                if local_names.contains(dep.as_str()) {
                    continue;
                }

                let Some(dep_module) = type_index.get(dep) else {
                    continue;
                };

                if dep_module == module {
                    continue;
                }

                imports
                    .entry(relative_import(module, dep_module))
                    .or_default()
                    .insert(dep.clone());
            }
        }

        let mut rendered = Vec::new();
        for (path, names) in imports {
            let joined = names.into_iter().collect::<Vec<_>>().join(", ");
            rendered.push(format!("import type {{ {} }} from '{}';", joined, path));
            if include_zod {
                rendered.push(format!("import {{ {}Schema }} from '{}';", joined, path));
            }
        }

        rendered
    }
}

fn extract_type_dependencies(ty: &GearMeshType) -> BTreeSet<String> {
    let mut deps = BTreeSet::new();

    match &ty.kind {
        TypeKind::Struct(s) => {
            for field in &s.fields {
                collect_type_refs(&field.ty, &mut deps);
            }
        }
        TypeKind::Enum(e) => {
            for variant in &e.variants {
                match &variant.content {
                    gear_mesh_core::VariantContent::Tuple(types) => {
                        for ty_ref in types {
                            collect_type_refs(ty_ref, &mut deps);
                        }
                    }
                    gear_mesh_core::VariantContent::Struct(fields) => {
                        for field in fields {
                            collect_type_refs(&field.ty, &mut deps);
                        }
                    }
                    gear_mesh_core::VariantContent::Unit => {}
                }
            }
        }
        TypeKind::Newtype(n) => collect_type_refs(&n.inner, &mut deps),
        _ => {}
    }

    deps
}

fn collect_type_refs(ty_ref: &TypeRef, deps: &mut BTreeSet<String>) {
    if !is_builtin_type(&ty_ref.name) && !is_internal_type(&ty_ref.name) {
        deps.insert(ty_ref.name.clone());
    }

    for generic in &ty_ref.generics {
        collect_type_refs(generic, deps);
    }
}

fn namespace_path(type_name: &str, separator: &str) -> String {
    let parts = type_name.split(separator).collect::<Vec<_>>();
    if parts.len() <= 1 {
        return format!("models/{}.ts", file_stem(type_name));
    }

    let mut path = parts[..parts.len() - 1]
        .iter()
        .map(|part| file_stem(part))
        .collect::<Vec<_>>();
    path.push(format!("{}.ts", file_stem(parts.last().unwrap())));
    path.join("/")
}

fn file_stem(type_name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<_> = type_name.chars().filter(|&ch| ch != ':').collect();

    for (index, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let prev_char_is_lowercase = if index > 0 {
                chars[index - 1].is_lowercase()
            } else {
                false
            };
            let next_char_is_lowercase =
                chars.get(index + 1).is_some_and(|next| next.is_lowercase());
            let prev_char_is_uppercase = if index > 0 {
                chars[index - 1].is_uppercase()
            } else {
                false
            };

            if index > 0
                && (prev_char_is_lowercase || (prev_char_is_uppercase && next_char_is_lowercase))
            {
                out.push('-');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }

    out
}

fn relative_import(from_module: &str, to_module: &str) -> String {
    let from_parts = from_module.split('/').collect::<Vec<_>>();
    let to_parts = to_module.split('/').collect::<Vec<_>>();

    let mut common = 0;
    while common < from_parts.len().saturating_sub(1)
        && common < to_parts.len().saturating_sub(1)
        && from_parts[common] == to_parts[common]
    {
        common += 1;
    }

    let mut path = Vec::new();
    path.extend(std::iter::repeat_n(
        "..",
        from_parts.len().saturating_sub(1) - common,
    ));
    for segment in &to_parts[common..] {
        path.push(segment);
    }

    let path = path.join("/");
    let path = path.strip_suffix(".ts").unwrap_or(&path);
    if path.starts_with('.') {
        path.to_string()
    } else {
        format!("./{}", path)
    }
}

#[cfg(test)]
mod tests {
    use gear_mesh_core::{FieldInfo, StructType, TypeAttributes};

    use super::*;

    #[test]
    fn per_type_organization_uses_kebab_case_paths() {
        let user = GearMeshType {
            name: "UserId".to_string(),
            kind: TypeKind::Struct(StructType { fields: vec![] }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        };

        let organizer = ModuleOrganizer::new(std::slice::from_ref(&user));
        let modules = organizer.organize(&[user], &ModuleStrategy::PerType);
        assert!(modules.contains_key("models/user-id.ts"));
    }

    #[test]
    fn generate_imports_links_cross_module_dependencies() {
        let user_id = GearMeshType {
            name: "UserId".to_string(),
            kind: TypeKind::Struct(StructType { fields: vec![] }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        };
        let user = GearMeshType {
            name: "User".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("UserId"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                }],
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        };

        let organizer = ModuleOrganizer::new(&[user_id.clone(), user.clone()]);
        let modules = organizer.organize(&[user_id, user.clone()], &ModuleStrategy::PerType);
        let type_index = organizer.build_type_index(&modules);
        let imports = organizer.generate_imports("models/user.ts", &[user], &type_index, true);

        assert!(imports.contains(&"import type { UserId } from './user-id';".to_string()));
        assert!(imports.contains(&"import { UserIdSchema } from './user-id';".to_string()));
    }

    #[test]
    fn file_stem_preserves_acronyms() {
        assert_eq!(file_stem("MyID"), "my-id");
        assert_eq!(file_stem("HTTPClient"), "http-client");
    }
}
