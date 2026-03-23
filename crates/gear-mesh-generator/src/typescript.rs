//! TypeScriptコード生成の主要ロジック

use gear_mesh_core::{
    EnumRepresentation, EnumType, FieldInfo, GearMeshType, NewtypeType, RenameRule, StructType,
    TypeAttributes, TypeKind, TypeRef, VariantContent, to_typescript_primitive,
};

use crate::{GeneratorConfig, OptionStyle, ResultStyle};

/// TypeScript生成器
pub struct TypeScriptGenerator {
    config: GeneratorConfig,
    pub output: String,
}

impl TypeScriptGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self {
            config,
            output: String::new(),
        }
    }

    /// 複数の型からTypeScriptコードを生成
    pub fn generate(&mut self, types: &[GearMeshType]) -> String {
        self.output.clear();

        // Zod import
        if self.config.generate_zod {
            self.output.push_str("import { z } from 'zod';\n\n");
        }

        // Branded Type用のヘルパーを追加
        if self.config.generate_branded && types.iter().any(|t| t.attributes.branded) {
            self.output.push_str("// Branded Type helper\n");
            self.output
                .push_str("type Brand<T, B> = T & { readonly __brand: B };\n\n");
        }

        // 各型を生成
        for ty in types {
            self.generate_type(ty);
            self.output.push('\n');
        }

        // Zodスキーマを生成
        if self.config.generate_zod {
            self.output.push_str("// Zod Schemas\n\n");
            let validator = crate::ValidationGenerator::new(self.config.clone());
            for ty in types {
                if let Some(schema) = validator.generate_zod_schema(ty) {
                    self.output.push_str(&schema);
                    self.output.push('\n');
                }
            }
        }

        self.output.clone()
    }

    /// 単一の型を生成
    pub fn generate_type(&mut self, ty: &GearMeshType) {
        // JSDoc生成
        if self.config.generate_jsdoc
            && let Some(ref docs) = ty.docs
        {
            self.output.push_str(&docs.to_jsdoc());
            self.output.push('\n');
        }

        match &ty.kind {
            TypeKind::Struct(s) => self.generate_struct(&ty.name, s, &ty.generics, &ty.attributes),
            TypeKind::Enum(e) => self.generate_enum(&ty.name, e, &ty.generics, &ty.attributes),
            TypeKind::Newtype(n) => {
                if ty.attributes.branded {
                    self.generate_branded_type(&ty.name, n);
                } else {
                    self.generate_type_alias(&ty.name, n);
                }
            }
            TypeKind::Primitive(_) | TypeKind::Tuple(_) | TypeKind::Array(_) => {
                // プリミティブ型は通常エクスポートしない
            }
            _ => {}
        }
    }

    /// 構造体を生成
    fn generate_struct(
        &mut self,
        name: &str,
        struct_type: &StructType,
        generics: &[gear_mesh_core::GenericParam],
        attrs: &TypeAttributes,
    ) {
        let generic_str = if generics.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                generics
                    .iter()
                    .map(|g| g.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        self.output
            .push_str(&format!("export interface {}{} {{\n", name, generic_str));

        for field in &struct_type.fields {
            self.generate_field(field, attrs.serde.rename_all);
        }

        self.output.push_str("}\n");
    }

    /// フィールドを生成
    fn generate_field(&mut self, field: &FieldInfo, rename_all: Option<RenameRule>) {
        let indent = &self.config.indent;

        // フィールドのJSDoc
        if self.config.generate_jsdoc
            && let Some(ref docs) = field.docs
        {
            self.output
                .push_str(&format!("{}{}\n", indent, docs.to_inline_jsdoc()));
        }

        let field_name = format_property_name(&resolve_field_name(field, rename_all));
        let optional = if self.is_optional_field(field) {
            "?"
        } else {
            ""
        };
        let ts_type = self.field_type_to_typescript(field);

        self.output.push_str(&format!(
            "{}{}{}: {};\n",
            indent, field_name, optional, ts_type
        ));
    }

    /// 列挙型を生成
    fn generate_enum(
        &mut self,
        name: &str,
        enum_type: &EnumType,
        generics: &[gear_mesh_core::GenericParam],
        attrs: &TypeAttributes,
    ) {
        let generic_str = if generics.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                generics
                    .iter()
                    .map(|g| g.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        // ユニオン型として生成
        let variants: Vec<String> = enum_type
            .variants
            .iter()
            .map(|v| self.generate_variant(name, v, &enum_type.representation, attrs))
            .collect();

        self.output.push_str(&format!(
            "export type {}{} = {};\n",
            name,
            generic_str,
            variants.join(" | ")
        ));
    }

    /// 列挙型バリアントを生成
    fn generate_variant(
        &self,
        _enum_name: &str,
        variant: &gear_mesh_core::EnumVariant,
        repr: &EnumRepresentation,
        attrs: &TypeAttributes,
    ) -> String {
        let variant_name = apply_rename_all(&variant.name, attrs.serde.rename_all);
        match (&variant.content, repr) {
            (VariantContent::Unit, EnumRepresentation::External) => {
                format!("\"{}\"", variant_name)
            }
            (VariantContent::Unit, EnumRepresentation::Internal { tag }) => {
                format!("{{ {}: \"{}\" }}", tag, variant_name)
            }
            (VariantContent::Tuple(types), EnumRepresentation::External) => {
                if types.len() == 1 {
                    let inner = self.type_ref_to_typescript(&types[0]);
                    format!("{{ \"{}\": {} }}", variant_name, inner)
                } else {
                    let inner: Vec<_> = types
                        .iter()
                        .map(|t| self.type_ref_to_typescript(t))
                        .collect();
                    format!("{{ \"{}\": [{}] }}", variant_name, inner.join(", "))
                }
            }
            (VariantContent::Struct(fields), EnumRepresentation::External) => {
                let field_strs: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        let ts_type = self.type_ref_to_typescript(&f.ty);
                        let field_name = format_property_name(&resolve_field_name(f, None));
                        format!("{}: {}", field_name, ts_type)
                    })
                    .collect();
                format!(
                    "{{ \"{}\": {{ {} }} }}",
                    variant_name,
                    field_strs.join("; ")
                )
            }
            (VariantContent::Struct(fields), EnumRepresentation::Internal { tag }) => {
                let field_strs: Vec<_> = fields
                    .iter()
                    .map(|f| {
                        let ts_type = self.type_ref_to_typescript(&f.ty);
                        let field_name = format_property_name(&resolve_field_name(f, None));
                        format!("{}: {}", field_name, ts_type)
                    })
                    .collect();
                format!(
                    "{{ {}: \"{}\"; {} }}",
                    tag,
                    variant_name,
                    field_strs.join("; ")
                )
            }
            _ => format!("\"{}\"", variant_name),
        }
    }

    /// Branded Typeを生成
    fn generate_branded_type(&mut self, name: &str, newtype: &NewtypeType) {
        let inner_type = self.type_ref_to_typescript(&newtype.inner);
        self.output.push_str(&format!(
            "export type {} = Brand<{}, \"{}\">;\n",
            name, inner_type, name
        ));

        // ヘルパー関数を生成
        self.output.push_str(&format!(
            "export const {} = (value: {}): {} => value as {};\n",
            name, inner_type, name, name
        ));
    }

    /// 通常のtype aliasを生成
    fn generate_type_alias(&mut self, name: &str, newtype: &NewtypeType) {
        let inner_type = self.type_ref_to_typescript(&newtype.inner);
        self.output
            .push_str(&format!("export type {} = {};\n", name, inner_type));
    }

    /// TypeRefからTypeScript型文字列へ変換
    fn type_ref_to_typescript(&self, type_ref: &TypeRef) -> String {
        if let Some(primitive) =
            to_typescript_primitive(type_ref.name.as_str(), self.config.use_bigint)
        {
            return primitive.to_string();
        }

        match type_ref.name.as_str() {
            "Vec" | "__array__" | "__slice__" => {
                if let Some(inner) = type_ref.generics.first() {
                    format!("{}[]", self.type_ref_to_typescript(inner))
                } else {
                    "unknown[]".to_string()
                }
            }
            "Option" => {
                if let Some(inner) = type_ref.generics.first() {
                    self.wrap_option_type(self.type_ref_to_typescript(inner))
                } else {
                    self.wrap_option_type("unknown".to_string())
                }
            }
            "Result" => self.result_to_typescript(type_ref),
            "Box" | "Arc" | "Rc" | "Cow" => {
                if let Some(inner) = type_ref.generics.last() {
                    self.type_ref_to_typescript(inner)
                } else {
                    "unknown".to_string()
                }
            }
            "HashMap" | "BTreeMap" => {
                let key = type_ref
                    .generics
                    .first()
                    .map(|t| self.type_ref_to_typescript(t))
                    .unwrap_or_else(|| "string".to_string());
                let value = type_ref
                    .generics
                    .get(1)
                    .map(|t| self.type_ref_to_typescript(t))
                    .unwrap_or_else(|| "unknown".to_string());

                // キーがstringの場合はRecord、それ以外はMap
                if key == "string" {
                    format!("Record<string, {}>", value)
                } else {
                    format!("Map<{}, {}>", key, value)
                }
            }
            "HashSet" | "BTreeSet" => {
                if let Some(inner) = type_ref.generics.first() {
                    format!("Set<{}>", self.type_ref_to_typescript(inner))
                } else {
                    "Set<unknown>".to_string()
                }
            }
            "__tuple__" => {
                let types: Vec<_> = type_ref
                    .generics
                    .iter()
                    .map(|t| self.type_ref_to_typescript(t))
                    .collect();
                format!("[{}]", types.join(", "))
            }

            // カスタム型（そのまま使用）
            _ => {
                if type_ref.generics.is_empty() {
                    type_ref.name.clone()
                } else {
                    let generics: Vec<_> = type_ref
                        .generics
                        .iter()
                        .map(|t| self.type_ref_to_typescript(t))
                        .collect();
                    format!("{}<{}>", type_ref.name, generics.join(", "))
                }
            }
        }
    }

    fn wrap_option_type(&self, inner: String) -> String {
        match self.config.option_style {
            OptionStyle::Nullable => format!("{} | null", inner),
            OptionStyle::Optional => inner,
            OptionStyle::Both => format!("{} | null", inner),
        }
    }

    fn is_optional_field(&self, field: &FieldInfo) -> bool {
        if field.ty.name != "Option" || !field.optional {
            return field.optional;
        }

        match self.config.option_style {
            OptionStyle::Nullable => false,
            OptionStyle::Optional | OptionStyle::Both => true,
        }
    }

    fn field_type_to_typescript(&self, field: &FieldInfo) -> String {
        if field.ty.name != "Option" || field.ty.generics.is_empty() {
            return self.type_ref_to_typescript(&field.ty);
        }

        let inner = self.type_ref_to_typescript(&field.ty.generics[0]);
        match self.config.option_style {
            OptionStyle::Nullable => format!("{} | null", inner),
            OptionStyle::Optional => inner,
            OptionStyle::Both => format!("{} | null", inner),
        }
    }

    fn result_to_typescript(&self, type_ref: &TypeRef) -> String {
        let ok = type_ref
            .generics
            .first()
            .map(|ty| self.type_ref_to_typescript(ty))
            .unwrap_or_else(|| "unknown".to_string());
        let err = type_ref
            .generics
            .get(1)
            .map(|ty| self.type_ref_to_typescript(ty))
            .unwrap_or_else(|| "unknown".to_string());

        match self.config.result_style {
            ResultStyle::OkOnly => ok,
            ResultStyle::TaggedUnion => format!("{{ ok: {} }} | {{ err: {} }}", ok, err),
            ResultStyle::SuccessError => format!(
                "{{ success: true; data: {} }} | {{ success: false; error: {} }}",
                ok, err
            ),
        }
    }
}

fn format_property_name(name: &str) -> String {
    if is_plain_typescript_identifier(name) {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}

fn resolve_field_name(field: &FieldInfo, rename_all: Option<RenameRule>) -> String {
    if let Some(rename) = &field.serde_attrs.rename {
        rename.clone()
    } else {
        apply_rename_all(&field.name, rename_all)
    }
}

fn apply_rename_all(name: &str, rename_all: Option<RenameRule>) -> String {
    rename_all
        .map(|rule| rule.apply(name))
        .unwrap_or_else(|| name.to_string())
}

fn is_plain_typescript_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::TypeAttributes;

    #[test]
    fn test_generate_simple_struct() {
        let ty = GearMeshType {
            name: "User".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![
                    FieldInfo {
                        name: "id".to_string(),
                        ty: TypeRef::new("i32"),
                        docs: None,
                        validations: vec![],
                        optional: false,
                        serde_attrs: Default::default(),
                    },
                    FieldInfo {
                        name: "name".to_string(),
                        ty: TypeRef::new("String"),
                        docs: None,
                        validations: vec![],
                        optional: false,
                        serde_attrs: Default::default(),
                    },
                ],
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        };

        let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_jsdoc(false));
        let output = generator.generate(&[ty]);

        assert!(output.contains("export interface User {"));
        assert!(output.contains("id: number;"));
        assert!(output.contains("name: string;"));
    }

    #[test]
    fn test_generate_branded_type() {
        let ty = GearMeshType {
            name: "UserId".to_string(),
            kind: TypeKind::Newtype(NewtypeType {
                inner: TypeRef::new("i32"),
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes {
                branded: true,
                ..Default::default()
            },
        };

        let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_jsdoc(false));
        let output = generator.generate(&[ty]);

        assert!(output.contains("type Brand<T, B>"));
        assert!(output.contains("export type UserId = Brand<number, \"UserId\">;"));
        assert!(
            output.contains("export const UserId = (value: number): UserId => value as UserId;")
        );
    }

    #[test]
    fn test_format_property_name_quotes_non_identifiers() {
        assert_eq!(format_property_name("displayName"), "displayName");
        assert_eq!(format_property_name("display-name"), "\"display-name\"");
    }
}
