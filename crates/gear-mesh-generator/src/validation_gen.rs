use crate::{GeneratorConfig, OptionStyle, ResultStyle};
use gear_mesh_core::{FieldInfo, GearMeshType, TypeKind, is_bigint_type, is_builtin_type};

/// Generator for Zod validation schemas
pub struct ValidationGenerator {
    config: GeneratorConfig,
}

impl ValidationGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Generates a Zod schema
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

        // Extract the target type for validation and schema generation.
        // For Option<T> fields, we need to unwrap to get T here because:
        // 1. Validation rules apply to the inner type T, not the Option wrapper
        // 2. We need to append .nullable() AFTER validation rules (e.g., .min().max().nullable())
        // 3. The recursive type_to_zod handles nested Options in complex types (e.g., Vec<Option<String>>)
        let target_type = if field.ty.name == "Option" && !field.ty.generics.is_empty() {
            &field.ty.generics[0]
        } else {
            &field.ty
        };

        // ベースとなるスキーマを生成
        let base_schema = self.type_to_zod(target_type);

        // バリデーションルールの適用のための型判定（BigIntかどうか）
        // NOTE: ここでの判定は最上位の型に対してのみ有効
        let is_bigint = self.config.use_bigint && is_bigint_type(&target_type.name);

        let mut result = base_schema;

        // IMPORTANT: Add validation rules BEFORE nullable
        for rule in &field.validations {
            result.push_str(&rule.to_zod_schema(is_bigint));
        }

        // Add the configured Option wrapper AFTER validations
        if is_option {
            result = self.wrap_option_schema(result);
        }

        result
    }

    /// Recursively generates a Zod schema from a TypeRef
    fn type_to_zod(&self, type_ref: &gear_mesh_core::TypeRef) -> String {
        match type_ref.name.as_str() {
            // プリミティブ型
            name if is_builtin_type(name) => {
                // コレクション型は個別に処理
                match name {
                    "Vec" | "Array" => {
                        if !type_ref.generics.is_empty() {
                            let inner_schema = self.type_to_zod(&type_ref.generics[0]);
                            format!("z.array({})", inner_schema)
                        } else {
                            "z.array(z.unknown())".to_string()
                        }
                    }
                    "Option" => {
                        if !type_ref.generics.is_empty() {
                            let inner_schema = self.type_to_zod(&type_ref.generics[0]);
                            self.wrap_option_schema(inner_schema)
                        } else {
                            self.wrap_option_schema("z.unknown()".to_string())
                        }
                    }
                    "Result" => self.result_to_zod(type_ref),
                    "HashMap" | "BTreeMap" => {
                        let value_schema = if type_ref.generics.len() >= 2 {
                            self.type_to_zod(&type_ref.generics[1])
                        } else {
                            "z.unknown()".to_string()
                        };
                        // HashMap<K, V> -> z.record(V) (Key is always string in JS objects usually, but Zod supports record)
                        format!("z.record({})", value_schema)
                    }
                    "HashSet" | "BTreeSet" => {
                        if !type_ref.generics.is_empty() {
                            format!("z.set({})", self.type_to_zod(&type_ref.generics[0]))
                        } else {
                            "z.set(z.unknown())".to_string()
                        }
                    }
                    _ => self.get_zod_primitive_type(name),
                }
            }
            // カスタム型
            name => format!("{}Schema", name),
        }
    }

    fn get_zod_primitive_type(&self, type_name: &str) -> String {
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

    fn wrap_option_schema(&self, schema: String) -> String {
        match self.config.option_style {
            OptionStyle::Nullable => {
                if schema.ends_with(".nullable()") || schema.ends_with(".nullish()") {
                    schema
                } else {
                    format!("{}.nullable()", schema)
                }
            }
            OptionStyle::Optional => {
                if schema.ends_with(".optional()") || schema.ends_with(".nullish()") {
                    schema
                } else {
                    format!("{}.optional()", schema)
                }
            }
            OptionStyle::Both => {
                if schema.ends_with(".nullish()") {
                    schema
                } else {
                    format!("{}.nullish()", schema)
                }
            }
        }
    }

    fn result_to_zod(&self, type_ref: &gear_mesh_core::TypeRef) -> String {
        let ok = type_ref
            .generics
            .first()
            .map(|ty| self.type_to_zod(ty))
            .unwrap_or_else(|| "z.unknown()".to_string());
        let err = type_ref
            .generics
            .get(1)
            .map(|ty| self.type_to_zod(ty))
            .unwrap_or_else(|| "z.unknown()".to_string());

        match self.config.result_style {
            ResultStyle::OkOnly => ok,
            ResultStyle::TaggedUnion => format!(
                "z.union([z.object({{ ok: {} }}), z.object({{ err: {} }})])",
                ok, err
            ),
            ResultStyle::SuccessError => format!(
                "z.union([z.object({{ success: z.literal(true), data: {} }}), z.object({{ success: z.literal(false), error: {} }})])",
                ok, err
            ),
        }
    }
}

impl Default for ValidationGenerator {
    fn default() -> Self {
        Self::new(GeneratorConfig::default())
    }
}
