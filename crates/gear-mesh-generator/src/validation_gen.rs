use crate::GeneratorConfig;
use gear_mesh_core::{FieldInfo, GearMeshType, TypeKind};

/// Zodバリデーションスキーマ生成器
pub struct ValidationGenerator {
    config: GeneratorConfig,
}

impl ValidationGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Zodスキーマを生成
    pub fn generate_zod_schema(&self, ty: &GearMeshType) -> Option<String> {
        match &ty.kind {
            TypeKind::Struct(s) => {
                let mut schema = format!("export const {}Schema = z.object({{\n", ty.name);

                for field in &s.fields {
                    let field_schema = self.field_to_zod(field);
                    schema.push_str(&format!("    {}: {},\n", field.name, field_schema));
                }

                schema.push_str("});\n");
                Some(schema)
            }
            _ => None,
        }
    }

    fn field_to_zod(&self, field: &FieldInfo) -> String {
        let is_option = field.optional;

        // 対象となる型を決定（Optionの場合は中身を取り出す）
        let target_type = if field.ty.name == "Option" && !field.ty.generics.is_empty() {
            &field.ty.generics[0]
        } else {
            &field.ty
        };

        // 配列型かどうかの判定
        let is_array = target_type.name == "Vec" || target_type.name == "Array";

        let base_schema = if is_array {
            if !target_type.generics.is_empty() {
                let inner_type = &target_type.generics[0];
                if is_primitive_type(&inner_type.name) {
                    format!("z.array({})", self.get_zod_type(&inner_type.name))
                } else {
                    format!("z.array({}Schema)", inner_type.name)
                }
            } else {
                "z.array(z.unknown())".to_string()
            }
        } else if is_primitive_type(&target_type.name) {
            self.get_zod_type(&target_type.name)
        } else {
            // カスタム型の場合はスキーマを参照
            format!("{}Schema", target_type.name)
        };

        // バリデーションルールの適用のための型判定（BigIntかどうか）
        // 配列自体のバリデーションは未対応（lengthなど）、ここでは要素の型に対するバリデーションは考慮しづらい
        // 現状のgear-mesh-coreの仕様では、validationsはフィールドにかかるもの。
        // なので、配列の場合は配列自体へのルール（min_itemsなど）か、要素へのルールか曖昧。
        // しかし、BigIntに対するバリデーション(min/max)は要素に対してかかるべきか？
        // ここでは単純化して、ターゲットタイプがBigIntかどうかで判定。
        let is_bigint = self.config.use_bigint
            && matches!(
                target_type.name.as_str(),
                "i64" | "i128" | "u64" | "u128" | "isize" | "usize"
            );

        let mut result = base_schema;

        // IMPORTANT: Add validation rules BEFORE nullable
        for rule in &field.validations {
            result.push_str(&rule.to_zod_schema(is_bigint));
        }

        // Add nullable for Option types AFTER validations
        if is_option {
            result.push_str(".nullable()");
        }

        result
    }

    fn get_zod_type(&self, type_name: &str) -> String {
        match type_name {
            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "z.number()".to_string(),
            "i64" | "i128" | "u64" | "u128" | "isize" | "usize" => {
                if self.config.use_bigint {
                    "z.bigint()".to_string()
                } else {
                    "z.number()".to_string()
                }
            }
            "String" | "str" | "char" => "z.string()".to_string(),
            "bool" => "z.boolean()".to_string(),
            _ => "z.unknown()".to_string(),
        }
    }
}

impl Default for ValidationGenerator {
    fn default() -> Self {
        Self::new(GeneratorConfig::default())
    }
}

fn is_primitive_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "String"
            | "str"
            | "bool"
            | "char"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "isize"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "usize"
            | "f32"
            | "f64"
            | "Vec"
            | "Option"
            | "Result"
            | "HashMap"
            | "HashSet"
            | "Box"
            | "Arc"
            | "Rc"
            | "Cow"
    )
}
