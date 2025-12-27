//! Branded Type生成ヘルパー

use gear_mesh_core::{GearMeshType, TypeKind};

/// Branded Type生成器
pub struct BrandedTypeGenerator;

impl BrandedTypeGenerator {
    /// Branded Type用のヘルパーコードを生成
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

    /// 型から Branded Type コードを生成
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
}
