pub use gear_mesh_core::{
    is_bigint_type, is_builtin_type, is_internal_type, to_typescript_primitive,
};

use gear_mesh_core::{FieldInfo, RenameRule};

pub fn format_property_name(name: &str) -> String {
    if is_plain_javascript_identifier(name) {
        name.to_string()
    } else {
        format!("{name:?}")
    }
}

pub fn resolve_field_name(field: &FieldInfo, rename_all: Option<RenameRule>) -> String {
    if let Some(rename) = &field.serde_attrs.rename {
        rename.clone()
    } else {
        apply_rename_all(&field.name, rename_all)
    }
}

pub fn apply_rename_all(name: &str, rename_all: Option<RenameRule>) -> String {
    rename_all
        .map(|rule| rule.apply(name))
        .unwrap_or_else(|| name.to_string())
}

pub fn is_plain_javascript_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
        return false;
    }

    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::{SerdeFieldAttrs, TypeRef};

    #[test]
    fn property_names_quote_non_identifiers() {
        assert_eq!(format_property_name("displayName"), "displayName");
        assert_eq!(format_property_name("display-name"), "\"display-name\"");
    }

    #[test]
    fn field_name_prefers_explicit_rename() {
        let field = FieldInfo {
            name: "display_name".to_string(),
            ty: TypeRef::new("String"),
            docs: None,
            validations: vec![],
            optional: false,
            serde_attrs: SerdeFieldAttrs {
                rename: Some("display-name".to_string()),
                ..Default::default()
            },
        };

        assert_eq!(
            resolve_field_name(&field, Some(RenameRule::CamelCase)),
            "display-name"
        );
    }
}
