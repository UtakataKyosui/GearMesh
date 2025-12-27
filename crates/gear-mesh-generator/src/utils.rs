/// Checks if a type is a built-in Rust type that has special handling in the generator.
///
/// This is used to differentiate between standard library types (including primitives,
/// collections, and smart pointers) and user-defined structs/enums which are expected
/// to have a corresponding `...Schema` generated.
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
            | "HashSet"
            | "Box"
            | "Arc"
            | "Rc"
            | "Cow"
    )
}

/// Determines if the given type name should be treated as a `bigint` in TypeScript.
pub fn is_bigint_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "i64" | "i128" | "u64" | "u128" | "isize" | "usize"
    )
}
