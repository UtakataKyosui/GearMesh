use gear_mesh_core::{FieldInfo, GearMeshType, TypeKind};

/// Zodバリデーションスキーマ生成器
pub struct ValidationGenerator;

impl ValidationGenerator {
    pub fn new() -> Self {
        Self
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
        // Check if this is an Option type
        let is_option = field.optional;

        // Get base type name, handling generics
        let base_type_name = if !field.ty.generics.is_empty() {
            // For generic types like Vec<T> or Option<T>, use the first generic argument
            &field.ty.generics[0].name
        } else {
            &field.ty.name
        };

        let base_type = match base_type_name.as_str() {
            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "z.number()",
            "i64" | "i128" | "u64" | "u128" => "z.bigint()",
            "String" | "str" | "char" => "z.string()",
            "bool" => "z.boolean()",
            _ => {
                // Check if it's a Vec/Array
                if field.ty.name == "Vec" || field.ty.name == "Array" {
                    // For arrays of custom types, use schema reference
                    if !field.ty.generics.is_empty() {
                        let inner_type = &field.ty.generics[0].name;
                        if !is_primitive_type(inner_type) {
                            let schema_ref = format!("{}Schema", inner_type);
                            return if is_option {
                                format!("z.array({}).nullable()", schema_ref)
                            } else {
                                format!("z.array({})", schema_ref)
                            };
                        }
                    }
                    return if is_option {
                        "z.array(z.unknown()).nullable()".to_string()
                    } else {
                        "z.array(z.unknown())".to_string()
                    };
                }
                // For custom types, use schema reference
                if !is_primitive_type(base_type_name) {
                    return if is_option {
                        format!("{}Schema.nullable()", base_type_name)
                    } else {
                        format!("{}Schema", base_type_name)
                    };
                }
                "z.unknown()"
            }
        };

        let mut result = base_type.to_string();

        // IMPORTANT: Add validation rules BEFORE nullable
        // This ensures validations apply to the inner type, not the nullable wrapper
        for rule in &field.validations {
            result.push_str(&rule.to_zod_schema());
        }

        // Add nullable for Option types AFTER validations
        if is_option {
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
