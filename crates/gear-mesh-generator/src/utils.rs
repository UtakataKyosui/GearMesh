/// Checks if a type is a primitive type
pub fn is_primitive_type(type_name: &str) -> bool {
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

/// Checks if the specified type should be treated as a BigInt
pub fn is_bigint_type(type_name: &str) -> bool {
    matches!(
        type_name,
        "i64" | "i128" | "u64" | "u128" | "isize" | "usize"
    )
}
