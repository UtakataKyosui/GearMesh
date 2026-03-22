/// Returns true when the type name is treated as a built-in Rust or standard container type.
pub fn is_builtin_type(type_name: &str) -> bool {
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
            | "BTreeMap"
            | "HashSet"
            | "BTreeSet"
            | "Box"
            | "Arc"
            | "Rc"
            | "Cow"
    )
}

/// Returns true when the type name is represented internally by GearMesh.
pub fn is_internal_type(type_name: &str) -> bool {
    matches!(type_name, "__array__" | "__slice__" | "__tuple__" | "()")
}

/// Determines if the given type name should be treated as a `bigint` in TypeScript.
pub fn is_bigint_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "i64" | "i128" | "u64" | "u128" | "isize" | "usize"
    )
}

/// Maps a primitive Rust type name to its TypeScript primitive counterpart.
pub fn to_typescript_primitive(type_name: &str, use_bigint: bool) -> Option<&'static str> {
    match type_name {
        "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => Some("number"),
        "i64" | "i128" | "u64" | "u128" | "isize" | "usize" => {
            if use_bigint {
                Some("bigint")
            } else {
                Some("number")
            }
        }
        "bool" => Some("boolean"),
        "char" | "String" | "str" => Some("string"),
        "()" => Some("null"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_type_detection_includes_standard_containers() {
        assert!(is_builtin_type("Option"));
        assert!(is_builtin_type("BTreeMap"));
        assert!(is_builtin_type("BTreeSet"));
        assert!(!is_builtin_type("UserId"));
    }

    #[test]
    fn test_internal_type_detection() {
        assert!(is_internal_type("__array__"));
        assert!(is_internal_type("__tuple__"));
        assert!(!is_internal_type("Vec"));
    }

    #[test]
    fn test_typescript_primitive_mapping() {
        assert_eq!(to_typescript_primitive("String", true), Some("string"));
        assert_eq!(to_typescript_primitive("u64", true), Some("bigint"));
        assert_eq!(to_typescript_primitive("u64", false), Some("number"));
        assert_eq!(to_typescript_primitive("CustomType", true), None);
    }
}
