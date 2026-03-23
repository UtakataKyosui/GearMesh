use gear_mesh_core::{GearMeshType, TypeRef, is_builtin_type, is_internal_type};
use std::collections::HashSet;

/// Extract all custom type names referenced by a type
pub fn extract_type_dependencies(ty: &GearMeshType) -> HashSet<String> {
    let mut deps = HashSet::new();

    match &ty.kind {
        gear_mesh_core::TypeKind::Struct(s) => {
            for field in &s.fields {
                collect_type_refs(&field.ty, &mut deps);
            }
        }
        gear_mesh_core::TypeKind::Enum(e) => {
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
        gear_mesh_core::TypeKind::Newtype(n) => {
            collect_type_refs(&n.inner, &mut deps);
        }
        _ => {}
    }

    deps
}

fn collect_type_refs(ty_ref: &TypeRef, deps: &mut HashSet<String>) {
    if !is_builtin_type(&ty_ref.name) && !is_internal_type(&ty_ref.name) {
        deps.insert(ty_ref.name.clone());
    }

    // Always recursively collect from generics (even for containers)
    for generic in &ty_ref.generics {
        collect_type_refs(generic, deps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_types_are_not_reported_as_dependencies() {
        let mut deps = HashSet::new();
        collect_type_refs(
            &TypeRef::with_generics("__array__", vec![TypeRef::new("String")]),
            &mut deps,
        );

        assert!(deps.is_empty());
    }

    #[test]
    fn test_standard_containers_only_report_custom_inner_types() {
        let mut deps = HashSet::new();
        collect_type_refs(
            &TypeRef::with_generics(
                "BTreeMap",
                vec![TypeRef::new("String"), TypeRef::new("User")],
            ),
            &mut deps,
        );

        assert_eq!(deps.len(), 1);
        assert!(deps.contains("User"));
    }
}
