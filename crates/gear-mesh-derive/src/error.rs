use syn::{Error, Type, spanned::Spanned};

pub fn unsupported_type(ty: &Type) -> Error {
    Error::new_spanned(
        ty,
        format!(
            "unsupported type for #[derive(GearMesh)]: `{}`\nhelp: use a named type, tuple, slice/array, or a supported generic such as Option<T> or Vec<T>",
            quote::quote!(#ty)
        ),
    )
}

pub fn branded_requires_newtype(span: impl Spanned) -> Error {
    Error::new(
        span.span(),
        "#[gear_mesh(branded)] is only supported on single-field tuple structs\nhelp: use `struct UserId(i32);` or remove the `branded` option",
    )
}

pub fn unsupported_generic_argument(span: impl Spanned) -> Error {
    Error::new(
        span.span(),
        "unsupported generic argument in #[derive(GearMesh)]\nhelp: only type arguments like `Option<T>` or `HashMap<String, T>` are supported",
    )
}
