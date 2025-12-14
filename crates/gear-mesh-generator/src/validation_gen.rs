//! バリデーション関数生成

use gear_mesh_core::{FieldInfo, GearMeshType, StructType, TypeKind, ValidationRule};

/// バリデーション生成器
pub struct ValidationGenerator {
    indent: String,
}

impl ValidationGenerator {
    pub fn new() -> Self {
        Self {
            indent: "    ".to_string(),
        }
    }

    /// 型のバリデーション関数を生成
    pub fn generate(&self, ty: &GearMeshType) -> Option<String> {
        if !ty.attributes.validate {
            return None;
        }

        match &ty.kind {
            TypeKind::Struct(s) => Some(self.generate_struct_validation(&ty.name, s)),
            _ => None,
        }
    }

    /// 構造体のバリデーション関数を生成
    fn generate_struct_validation(&self, name: &str, struct_type: &StructType) -> String {
        let mut output = String::new();

        // Type guard関数
        output.push_str(&format!(
            "export function validate{}(data: unknown): data is {} {{\n",
            name, name
        ));
        output.push_str(&format!(
            "{}if (typeof data !== 'object' || data === null) return false;\n",
            self.indent
        ));
        output.push_str(&format!(
            "{}const obj = data as Record<string, unknown>;\n\n",
            self.indent
        ));

        // 各フィールドのバリデーション
        for field in &struct_type.fields {
            if !field.validations.is_empty() {
                output.push_str(&self.generate_field_validation(field));
            }
        }

        output.push_str(&format!("{}return true;\n", self.indent));
        output.push_str("}\n");

        output
    }

    /// フィールドのバリデーションコードを生成
    fn generate_field_validation(&self, field: &FieldInfo) -> String {
        let mut output = String::new();
        let field_name = &field.name;

        output.push_str(&format!("{}// {}\n", self.indent, field_name));

        for rule in &field.validations {
            let check = rule.to_typescript_check(field_name);
            let type_check = self.get_type_check(field_name, &field.ty);

            if field.optional {
                output.push_str(&format!(
                    "{}if (obj.{} !== null && obj.{} !== undefined) {{\n",
                    self.indent, field_name, field_name
                ));
                output.push_str(&format!(
                    "{}{}if (!({}) || !({})) return false;\n",
                    self.indent, self.indent, type_check, check
                ));
                output.push_str(&format!("{}}}\n", self.indent));
            } else {
                output.push_str(&format!(
                    "{}if (!({}) || !({})) return false;\n",
                    self.indent, type_check, check
                ));
            }
        }

        output.push('\n');
        output
    }

    /// 型チェックコードを生成
    fn get_type_check(&self, field_name: &str, ty: &gear_mesh_core::TypeRef) -> String {
        let js_type = match ty.name.as_str() {
            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "number",
            "i64" | "i128" | "u64" | "u128" => "number", // or bigint
            "String" | "str" | "char" => "string",
            "bool" => "boolean",
            _ => return "true".to_string(), // カスタム型は型チェックをスキップ
        };

        format!("typeof obj.{} === '{}'", field_name, js_type)
    }

    /// Zodスキーマを生成
    pub fn generate_zod_schema(&self, ty: &GearMeshType) -> Option<String> {
        match &ty.kind {
            TypeKind::Struct(s) => Some(self.generate_struct_zod(&ty.name, s)),
            _ => None,
        }
    }

    fn generate_struct_zod(&self, name: &str, struct_type: &StructType) -> String {
        let mut output = format!("export const {}Schema = z.object({{\n", name);

        for field in &struct_type.fields {
            let zod_type = self.field_to_zod(field);
            output.push_str(&format!("{}{}: {},\n", self.indent, field.name, zod_type));
        }

        output.push_str("});\n");
        output
    }

    fn field_to_zod(&self, field: &FieldInfo) -> String {
        let base_type = match field.ty.name.as_str() {
            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "z.number()",
            "i64" | "i128" | "u64" | "u128" => "z.bigint()",
            "String" | "str" | "char" => "z.string()",
            "bool" => "z.boolean()",
            _ => "z.unknown()",
        };

        let mut result = base_type.to_string();

        // バリデーションルールを追加
        for rule in &field.validations {
            result.push_str(&rule.to_zod_schema());
        }

        // オプショナル
        if field.optional {
            result.push_str(".nullable()");
        }

        result
    }
}

impl Default for ValidationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::{TypeAttributes, TypeRef};

    #[test]
    fn test_generate_validation() {
        let ty = GearMeshType {
            name: "User".to_string(),
            kind: TypeKind::Struct(StructType {
                fields: vec![FieldInfo {
                    name: "age".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: None,
                    validations: vec![ValidationRule::Range {
                        min: Some(0.0),
                        max: Some(150.0),
                    }],
                    optional: false,
                    serde_attrs: Default::default(),
                }],
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes {
                validate: true,
                ..Default::default()
            },
        };

        let gen = ValidationGenerator::new();
        let output = gen.generate(&ty);
        assert!(output.is_some());
        let code = output.unwrap();
        assert!(code.contains("validateUser"));
        assert!(code.contains("obj.age >= 0"));
        assert!(code.contains("obj.age <= 150"));
    }
}
