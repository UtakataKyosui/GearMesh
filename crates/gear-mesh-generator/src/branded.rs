//! Helpers for generating Branded Types.

use crate::GeneratorConfig;
use gear_mesh_core::{GearMeshType, TypeKind};

/// Generator for Branded Types
pub struct BrandedTypeGenerator {
    config: GeneratorConfig,
}

impl BrandedTypeGenerator {
    /// Creates a new generator.
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Generates helper code for Branded Types.
    pub fn generate_helpers() -> String {
        r#"// Branded Type utilities
type Brand<T, B> = T & { readonly __brand: B };

// Type guard helper
export function isBranded<T, B extends string>(
    value: unknown,
    brand: B,
    typeCheck: (v: unknown) => v is T
): value is Brand<T, B> {
    return typeCheck(value);
}
"#
        .to_string()
    }

    /// Generates code used for Branded Types from a Type.
    pub fn generate(ty: &GearMeshType, inner_ts_type: &str) -> Option<String> {
        if !ty.attributes.branded {
            return None;
        }

        if let TypeKind::Newtype(_) = &ty.kind {
            let name = &ty.name;
            Some(format!(
                r#"export type {name} = Brand<{inner_ts_type}, "{name}">;

export const {name} = (value: {inner_ts_type}): {name} => value as {name};

export function is{name}(value: unknown): value is {name} {{
    return typeof value === '{ts_typeof}';
}}
"#,
                name = name,
                inner_ts_type = inner_ts_type,
                ts_typeof = typescript_typeof(inner_ts_type),
            ))
        } else {
            None
        }
    }

    /// Generates a Zod schema for a Branded Type.
    pub fn generate_zod_schema(&self, ty: &GearMeshType) -> Option<String> {
        if !ty.attributes.branded {
            return None;
        }

        if let TypeKind::Newtype(newtype) = &ty.kind {
            let name = &ty.name;
            let inner_type = &newtype.inner;

            // 内部型に応じたZodスキーマを生成
            let base_schema = match inner_type.name.as_str() {
                "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "z.number()",
                "i64" | "i128" | "u64" | "u128" | "isize" | "usize" => {
                    if self.config.use_bigint {
                        "z.bigint()"
                    } else {
                        "z.number()"
                    }
                }
                "String" | "str" => "z.string()",
                "bool" => "z.boolean()",
                _ => "z.unknown()",
            };

            Some(format!(
                r#"export const {}Schema = {}.brand<"{}">();
"#,
                name, base_schema, name
            ))
        } else {
            None
        }
    }
}

fn typescript_typeof(ts_type: &str) -> &'static str {
    match ts_type {
        "number" | "bigint" => "number",
        "string" => "string",
        "boolean" => "boolean",
        _ => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::{NewtypeType, TypeAttributes, TypeRef};

    #[test]
    fn test_generate_branded() {
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

        let output = BrandedTypeGenerator::generate(&ty, "number");
        assert!(output.is_some());
        let code = output.unwrap();
        assert!(code.contains("export type UserId = Brand<number, \"UserId\">"));
    }

    #[test]
    fn test_generate_branded_zod_bigint() {
        let ty = GearMeshType {
            name: "BigId".to_string(),
            kind: TypeKind::Newtype(NewtypeType {
                inner: TypeRef::new("i64"),
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes {
                branded: true,
                ..Default::default()
            },
        };

        let config = GeneratorConfig::new().with_bigint(true);
        let generator = BrandedTypeGenerator::new(config);
        let output = generator.generate_zod_schema(&ty);

        assert!(output.is_some());
        let code = output.unwrap();
        assert!(code.contains("z.bigint().brand<\"BigId\">"));
    }
}
