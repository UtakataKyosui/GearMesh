//! TypeScriptコード生成の主要ロジック

use std::collections::BTreeSet;

use gear_mesh_core::{
    EnumRepresentation, EnumType, FieldInfo, GearMeshType, NewtypeType, RenameRule, StructType,
    TypeAttributes, TypeKind, TypeRef, ValidationRule, VariantContent, to_typescript_primitive,
};

use crate::utils::{apply_rename_all, format_property_name, resolve_field_name};
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
        self.generate_with_imports(types, &[])
    }

    pub fn generate_with_imports(
        &mut self,
        types: &[GearMeshType],
        extra_imports: &[String],
    ) -> String {
        self.output.clear();

        self.render_prelude(types, extra_imports);

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
            if self.config.enhanced_jsdoc {
                self.output
                    .push_str(&render_type_jsdoc(docs, &ty.name, self.config.generate_zod));
            } else {
                self.output.push_str(&docs.to_jsdoc());
            }
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
        let field_name = format_property_name(&resolve_field_name(field, rename_all));
        let optional = if self.is_optional_field(field) {
            "?"
        } else {
            ""
        };
        let ts_type = self.field_type_to_typescript(field);

        if self.config.generate_jsdoc
            && let Some(ref docs) = field.docs
        {
            if self.config.enhanced_jsdoc {
                self.output.push_str(&render_field_jsdoc(
                    indent,
                    docs,
                    &ts_type,
                    optional == "?",
                    &field.validations,
                ));
                self.output.push('\n');
            } else {
                self.output
                    .push_str(&format!("{}{}\n", indent, docs.to_inline_jsdoc()));
            }
        }

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
        if let Some(transformed) = self.transformer_type(type_ref) {
            return transformed;
        }

        if let Some(primitive) =
            to_typescript_primitive(type_ref.name.as_str(), self.config.use_bigint)
        {
            return primitive.to_string();
        }

        match type_ref.name.as_str() {
            "Vec" | "__array__" | "__slice__" => {
                if let Some(inner) = type_ref.generics.first() {
                    format!(
                        "{}[]",
                        wrap_array_element_type(self.type_ref_to_typescript(inner))
                    )
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
            OptionStyle::Optional => format!("{} | undefined", inner),
            OptionStyle::Both => format!("{} | null | undefined", inner),
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

    fn render_prelude(&mut self, types: &[GearMeshType], extra_imports: &[String]) {
        let mut imports = BTreeSet::new();

        if self.config.generate_zod {
            imports.insert("import { z } from 'zod';".to_string());
        }

        for transformer in &self.config.transformers {
            for ty in types {
                if contains_transformer_type(ty, transformer.as_ref()) {
                    for import in transformer.required_imports() {
                        imports.insert(import);
                    }
                }
            }
        }

        for import in extra_imports {
            imports.insert(import.clone());
        }

        if !imports.is_empty() {
            self.output
                .push_str(&imports.into_iter().collect::<Vec<_>>().join("\n"));
            self.output.push_str("\n\n");
        }
    }

    fn transformer_type(&self, type_ref: &TypeRef) -> Option<String> {
        self.config
            .transformers
            .iter()
            .find(|transformer| transformer.can_handle(&type_ref.name))
            .and_then(|transformer| transformer.transform_type(type_ref))
    }
}

fn wrap_array_element_type(inner: String) -> String {
    if inner.contains('|') || inner.contains('&') {
        format!("({inner})")
    } else {
        inner
    }
}

fn contains_transformer_type(
    ty: &GearMeshType,
    transformer: &dyn gear_mesh_core::TypeTransformer,
) -> bool {
    match &ty.kind {
        TypeKind::Struct(s) => s
            .fields
            .iter()
            .any(|field| has_transformer_type(&field.ty, transformer)),
        TypeKind::Enum(e) => e.variants.iter().any(|variant| match &variant.content {
            VariantContent::Tuple(types) => {
                types.iter().any(|ty| has_transformer_type(ty, transformer))
            }
            VariantContent::Struct(fields) => fields
                .iter()
                .any(|field| has_transformer_type(&field.ty, transformer)),
            VariantContent::Unit => false,
        }),
        TypeKind::Newtype(n) => has_transformer_type(&n.inner, transformer),
        _ => false,
    }
}

fn has_transformer_type(
    type_ref: &TypeRef,
    transformer: &dyn gear_mesh_core::TypeTransformer,
) -> bool {
    transformer.can_handle(&type_ref.name)
        || type_ref
            .generics
            .iter()
            .any(|inner| has_transformer_type(inner, transformer))
}

fn render_type_jsdoc(
    docs: &gear_mesh_core::DocComment,
    name: &str,
    include_schema_ref: bool,
) -> String {
    let mut lines = vec!["/**".to_string()];

    if !docs.summary.is_empty() {
        lines.push(format!(" * {}", docs.summary));
    }

    if let Some(description) = &docs.description {
        lines.push(" *".to_string());
        for line in description.lines() {
            lines.push(format!(" * {}", line));
        }
    }

    lines.push(" *".to_string());
    lines.push(" * @remarks".to_string());
    lines.push(" * This type is automatically generated from Rust.".to_string());
    lines.push(" * Do not modify manually.".to_string());

    if include_schema_ref {
        lines.push(format!(
            " * @see {{@link {}Schema}} for runtime validation",
            name
        ));
    }

    lines.push(" */".to_string());
    lines.join("\n")
}

fn render_field_jsdoc(
    indent: &str,
    docs: &gear_mesh_core::DocComment,
    ts_type: &str,
    optional: bool,
    validations: &[ValidationRule],
) -> String {
    let mut lines = vec![format!("{indent}/**")];

    if !docs.summary.is_empty() {
        lines.push(format!("{indent} * {}", docs.summary));
    }

    if let Some(description) = &docs.description {
        lines.push(format!("{indent} *"));
        for line in description.lines() {
            lines.push(format!("{indent} * {}", line));
        }
    }

    lines.push(format!("{indent} * @type {{{}}}", ts_type));
    if optional {
        lines.push(format!("{indent} * @optional"));
    }

    for tag in validation_tags(validations) {
        lines.push(format!("{indent} * {}", tag));
    }

    lines.push(format!("{indent} */"));
    lines.join("\n")
}

fn validation_tags(validations: &[ValidationRule]) -> Vec<String> {
    let mut tags = Vec::new();

    for rule in validations {
        match rule {
            ValidationRule::Range { min, max } => {
                if let Some(min) = min {
                    tags.push(format!("@minimum {}", min));
                }
                if let Some(max) = max {
                    tags.push(format!("@maximum {}", max));
                }
            }
            ValidationRule::Length { min, max } => {
                if let Some(min) = min {
                    tags.push(format!("@minLength {}", min));
                }
                if let Some(max) = max {
                    tags.push(format!("@maxLength {}", max));
                }
            }
            ValidationRule::Email => tags.push("@format email".to_string()),
            ValidationRule::Url => tags.push("@format uri".to_string()),
            ValidationRule::Pattern(pattern) => {
                tags.push(format!("@pattern {}", sanitize_jsdoc_tag_value(pattern)))
            }
            ValidationRule::Required => tags.push("@required".to_string()),
            ValidationRule::Custom { name, message } => {
                tags.push(format!("@validation {}", sanitize_jsdoc_tag_value(name)));
                if let Some(message) = message {
                    tags.push(format!("@message {}", sanitize_jsdoc_tag_value(message)));
                }
            }
            // Cross-field and conditional rules are emitted as object-level Zod refinements,
            // so there is no stable field-level JSDoc tag representation for them here.
            ValidationRule::CrossField { .. } | ValidationRule::Conditional { .. } => {}
        }
    }

    tags
}

fn sanitize_jsdoc_tag_value(value: &str) -> String {
    value.replace("*/", "*\\/").replace('\n', "\\n")
}
#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::TypeAttributes;

    struct DateTimeTransformer;

    impl gear_mesh_core::TypeTransformer for DateTimeTransformer {
        fn can_handle(&self, type_name: &str) -> bool {
            type_name == "DateTime"
        }

        fn transform_type(&self, _: &TypeRef) -> Option<String> {
            Some("Date".to_string())
        }

        fn transform_zod(&self, _: &TypeRef) -> Option<String> {
            Some("z.coerce.date()".to_string())
        }

        fn required_imports(&self) -> Vec<String> {
            vec!["import { DateTime } from 'luxon';".to_string()]
        }
    }

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
    fn test_wrap_array_element_type_parenthesizes_unions() {
        assert_eq!(wrap_array_element_type("string".to_string()), "string");
        assert_eq!(
            wrap_array_element_type("string | undefined".to_string()),
            "(string | undefined)"
        );
    }
    #[test]
    fn test_custom_transformer_overrides_typescript_output() {
        let ty = GearMeshType {
            name: "AuditLog".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![FieldInfo {
                    name: "created_at".to_string(),
                    ty: TypeRef::new("DateTime"),
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

        let mut generator = TypeScriptGenerator::new(
            GeneratorConfig::new()
                .with_jsdoc(false)
                .with_transformer(DateTimeTransformer),
        );
        let output = generator.generate(&[ty]);

        assert!(output.contains("import { DateTime } from 'luxon';"));
        assert!(output.contains("created_at: Date;"));
    }

    #[test]
    fn test_enhanced_jsdoc_includes_type_and_validation_tags() {
        let ty = GearMeshType {
            name: "User".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(gear_mesh_core::DocComment::summary("Display name")),
                    validations: vec![ValidationRule::Length {
                        min: Some(1),
                        max: Some(20),
                    }],
                    optional: false,
                    serde_attrs: Default::default(),
                }],
            }),
            docs: Some(gear_mesh_core::DocComment::summary("User information")),
            generics: vec![],
            attributes: TypeAttributes::default(),
        };

        let mut generator = TypeScriptGenerator::new(
            GeneratorConfig::new()
                .with_zod(true)
                .with_enhanced_jsdoc(true),
        );
        let output = generator.generate(&[ty]);

        assert!(output.contains("@remarks"));
        assert!(output.contains("@see {@link UserSchema}"));
        assert!(output.contains("@type {string}"));
        assert!(output.contains("@minLength 1"));
        assert!(output.contains("@maxLength 20"));
    }

    #[test]
    fn enhanced_jsdoc_sanitizes_comment_terminators_and_newlines() {
        let ty = GearMeshType {
            name: "RuleDoc".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![FieldInfo {
                    name: "value".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(gear_mesh_core::DocComment::summary("Docs")),
                    validations: vec![
                        ValidationRule::Pattern("foo*/bar".to_string()),
                        ValidationRule::Custom {
                            name: "line\nbreak".to_string(),
                            message: Some("bad*/message".to_string()),
                        },
                    ],
                    optional: false,
                    serde_attrs: Default::default(),
                }],
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        };

        let mut generator =
            TypeScriptGenerator::new(GeneratorConfig::new().with_enhanced_jsdoc(true));
        let output = generator.generate(&[ty]);

        assert!(output.contains("@pattern foo*\\/bar"));
        assert!(output.contains("@validation line\\nbreak"));
        assert!(output.contains("@message bad*\\/message"));
    }
}
