//! 属性パーサー
//!
//! `#[gear_mesh(...)]` と `#[validate(...)]` 属性を解析します。

use syn::{Attribute, Expr, Lit, Meta, Result};

use gear_mesh_core::{CrossFieldRule, RenameRule, SerdeTypeAttrs, TypeAttributes, ValidationRule};

/// gear_mesh属性を解析
pub fn parse_gear_mesh_attrs(attrs: &[Attribute]) -> Result<TypeAttributes> {
    let mut result = TypeAttributes::default();

    for attr in attrs {
        if attr.path().is_ident("gear_mesh") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("branded") {
                    result.branded = true;
                } else if meta.path.is_ident("validate") {
                    result.validate = true;
                } else if meta.path.is_ident("bigint") && meta.input.peek(syn::Token![=]) {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let value: syn::LitStr = meta.input.parse()?;
                    if value.value() == "auto" {
                        result.bigint_auto = true;
                    } else {
                        return Err(meta.error(
                            "invalid value for `bigint`\nhelp: use `#[gear_mesh(bigint = \"auto\")]`",
                        ));
                    }
                } else if meta.path.is_ident("output") && meta.input.peek(syn::Token![=]) {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let value: syn::LitStr = meta.input.parse()?;
                    result.output_path = Some(value.value());
                } else {
                    return Err(meta.error(
                        "unsupported #[gear_mesh(...)] option\nhelp: supported options are `branded`, `validate`, `bigint = \"auto\"`, and `output = \"path\"`",
                    ));
                }
                Ok(())
            })?;
        }
    }

    result.serde = parse_serde_type_attrs(attrs)?;

    Ok(result)
}

/// validate属性を解析
pub fn parse_validate_attrs(
    attrs: &[Attribute],
    field_name: Option<&str>,
) -> Result<Vec<ValidationRule>> {
    let mut rules = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("validate") {
            let mut last_message_target = None;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("range") {
                    let mut min = None;
                    let mut max = None;
                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("min") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::Lit = inner.input.parse()?;
                            match lit {
                                syn::Lit::Int(i) => min = Some(i.base10_parse::<i64>()? as f64),
                                syn::Lit::Float(f) => min = Some(f.base10_parse::<f64>()?),
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        lit,
                                        "invalid `range(min = ...)` value\nhelp: use an integer or float literal, e.g. `min = 1`",
                                    ))
                                }
                            }
                        } else if inner.path.is_ident("max") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::Lit = inner.input.parse()?;
                            match lit {
                                syn::Lit::Int(i) => max = Some(i.base10_parse::<i64>()? as f64),
                                syn::Lit::Float(f) => max = Some(f.base10_parse::<f64>()?),
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        lit,
                                        "invalid `range(max = ...)` value\nhelp: use an integer or float literal, e.g. `max = 10`",
                                    ))
                                }
                            }
                        } else {
                            return Err(inner.error(
                                "unsupported `range(...)` option\nhelp: supported options are `min = ...` and `max = ...`",
                            ));
                        }
                        Ok(())
                    })?;
                    rules.push(ValidationRule::Range { min, max });
                    last_message_target = None;
                } else if meta.path.is_ident("length") {
                    let mut min = None;
                    let mut max = None;
                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("min") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitInt = inner.input.parse()?;
                            min = Some(lit.base10_parse()?);
                        } else if inner.path.is_ident("max") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitInt = inner.input.parse()?;
                            max = Some(lit.base10_parse()?);
                        } else {
                            return Err(inner.error(
                                "unsupported `length(...)` option\nhelp: supported options are `min = ...` and `max = ...`",
                            ));
                        }
                        Ok(())
                    })?;
                    rules.push(ValidationRule::Length { min, max });
                    last_message_target = None;
                } else if meta.path.is_ident("email") {
                    rules.push(ValidationRule::Email);
                    last_message_target = None;
                } else if meta.path.is_ident("url") {
                    rules.push(ValidationRule::Url);
                    last_message_target = None;
                } else if meta.path.is_ident("pattern") {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let lit: syn::LitStr = meta.input.parse()?;
                    rules.push(ValidationRule::Pattern(lit.value()));
                    last_message_target = None;
                } else if meta.path.is_ident("custom") {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let lit: syn::LitStr = meta.input.parse()?;
                    rules.push(ValidationRule::Custom {
                        name: lit.value(),
                        message: None,
                    });
                    last_message_target = Some(rules.len() - 1);
                } else if meta.path.is_ident("message") {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let lit: syn::LitStr = meta.input.parse()?;
                    if let Some(index) = last_message_target
                        && let Some(ValidationRule::Custom { message, .. }) =
                            rules.get_mut(index)
                    {
                        *message = Some(lit.value());
                    }
                } else if meta.path.is_ident("cross_field") {
                    let mut fields = field_name
                        .into_iter()
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>();
                    let mut rule = None;
                    let mut message = None;

                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("match") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitStr = inner.input.parse()?;
                            let other = lit.value().trim().to_string();
                            if other.is_empty() {
                                return Err(inner.error(
                                    "`cross_field(match = ...)` requires a non-empty field name",
                                ));
                            }
                            fields.push(other);
                            rule = Some(CrossFieldRule::Match);
                        } else if inner.path.is_ident("at_least_one") {
                            fields.extend(parse_csv_field_list(&inner)?);
                            rule = Some(CrossFieldRule::AtLeastOne);
                        } else if inner.path.is_ident("mutually_exclusive") {
                            fields.extend(parse_csv_field_list(&inner)?);
                            rule = Some(CrossFieldRule::MutuallyExclusive);
                        } else if inner.path.is_ident("message") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitStr = inner.input.parse()?;
                            message = Some(lit.value());
                        } else {
                            return Err(inner.error(
                                "unsupported `cross_field(...)` option\nhelp: supported options are `match = \"field\"`, `at_least_one = \"field1,field2\"`, `mutually_exclusive = \"field1,field2\"`, and `message = \"...\"`",
                            ));
                        }
                        Ok(())
                    })?;

                    let Some(rule) = rule else {
                        return Err(meta.error(
                            "missing `cross_field(...)` rule\nhelp: use `match = \"field\"`, `at_least_one = \"field1,field2\"`, or `mutually_exclusive = \"field1,field2\"`",
                        ));
                    };
                    if matches!(rule, CrossFieldRule::Match) && fields.len() < 2 {
                        return Err(meta.error(
                            "`cross_field(match = ...)` requires at least two fields",
                        ));
                    }

                    rules.push(ValidationRule::CrossField {
                        fields,
                        rule,
                        message,
                        path: field_name.map(ToOwned::to_owned),
                    });
                    last_message_target = None;
                } else if meta.path.is_ident("conditional") {
                    let mut condition = None;
                    let mut nested_rule = None;
                    let mut message = None;

                    meta.parse_nested_meta(|inner| {
                        if inner.path.is_ident("condition") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitStr = inner.input.parse()?;
                            condition = Some(lit.value());
                        } else if inner.path.is_ident("custom") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitStr = inner.input.parse()?;
                            nested_rule = Some(ValidationRule::Custom {
                                name: lit.value(),
                                message: None,
                            });
                        } else if inner.path.is_ident("message") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            let lit: syn::LitStr = inner.input.parse()?;
                            message = Some(lit.value());
                        } else {
                            return Err(inner.error(
                                "unsupported `conditional(...)` option\nhelp: supported options are `condition = \"...\"`, `custom = \"validator\"`, and `message = \"...\"`",
                            ));
                        }
                        Ok(())
                    })?;

                    let Some(condition) = condition else {
                        return Err(meta.error(
                            "missing `conditional(condition = ...)` expression",
                        ));
                    };
                    let Some(mut nested_rule) = nested_rule else {
                        return Err(meta.error(
                            "missing nested conditional rule\nhelp: use `conditional(condition = \"...\", custom = \"validator\")`",
                        ));
                    };

                    if let ValidationRule::Custom {
                        message: nested_message,
                        ..
                    } = &mut nested_rule
                    {
                        *nested_message = message;
                    }

                    rules.push(ValidationRule::Conditional {
                        condition,
                        rule: Box::new(nested_rule),
                    });
                    last_message_target = None;
                } else {
                    return Err(meta.error(
                        "unsupported #[validate(...)] rule\nhelp: supported rules are `range`, `length`, `email`, `url`, `pattern = \"...\"`, `custom = \"validator\"`, `cross_field(...)`, and `conditional(...)`",
                    ));
                }
                Ok(())
            })?;
        }
    }

    Ok(rules)
}

/// serde属性を解析してリネーム情報を取得
pub fn parse_serde_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("serde")
            && let Meta::List(list) = &attr.meta
        {
            // シンプルな実装：rename = "..." を探す
            let tokens = list.tokens.to_string();
            if let Some(start) = tokens.find("rename")
                && let Some(eq_pos) = tokens[start..].find('=')
            {
                let after_eq = &tokens[start + eq_pos + 1..];
                if let Some(quote_start) = after_eq.find('"')
                    && let Some(quote_end) = after_eq[quote_start + 1..].find('"')
                {
                    return Some(
                        after_eq[quote_start + 1..quote_start + 1 + quote_end].to_string(),
                    );
                }
            }
        }
    }
    None
}

/// serde型属性を解析
pub fn parse_serde_type_attrs(attrs: &[Attribute]) -> Result<SerdeTypeAttrs> {
    let mut result = SerdeTypeAttrs::default();

    for attr in attrs {
        if attr.path().is_ident("serde") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename_all") {
                    let value = parse_string_value(&meta)?;
                    result.rename_all = Some(value.parse::<RenameRule>().map_err(|_| {
                        meta.error(
                            "invalid `serde(rename_all = ...)` value\nhelp: supported values are `lowercase`, `UPPERCASE`, `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `kebab-case`, and `SCREAMING-KEBAB-CASE`",
                        )
                    })?);
                }
                Ok(())
            })?;
        }
    }

    Ok(result)
}

fn parse_string_value(meta: &syn::meta::ParseNestedMeta<'_>) -> Result<String> {
    let _ = meta.input.parse::<syn::Token![=]>()?;
    let value: syn::LitStr = meta.input.parse()?;
    Ok(value.value())
}

fn parse_csv_field_list(meta: &syn::meta::ParseNestedMeta<'_>) -> Result<Vec<String>> {
    let _ = meta.input.parse::<syn::Token![=]>()?;
    let lit: syn::LitStr = meta.input.parse()?;
    Ok(lit
        .value()
        .split(',')
        .map(str::trim)
        .filter(|field| !field.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

/// docコメントを抽出
pub fn extract_doc_comments(attrs: &[Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc")
                && let Meta::NameValue(nv) = &attr.meta
                && let Expr::Lit(expr_lit) = &nv.value
                && let Lit::Str(lit_str) = &expr_lit.lit
            {
                return Some(lit_str.value());
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_invalid_gear_mesh_option_reports_supported_values() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(GearMesh)]
            #[gear_mesh(unknown)]
            struct User {
                id: i32,
            }
        };

        let err = parse_gear_mesh_attrs(&input.attrs).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported #[gear_mesh(...)] option"));
        assert!(message.contains("supported options"));
    }

    #[test]
    fn test_invalid_bigint_value_reports_fix() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(GearMesh)]
            #[gear_mesh(bigint = "always")]
            struct User {
                id: i32,
            }
        };

        let err = parse_gear_mesh_attrs(&input.attrs).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("invalid value for `bigint`"));
        assert!(message.contains("bigint = \"auto\""));
    }

    #[test]
    fn test_invalid_validate_rule_reports_supported_rules() {
        let field: syn::Field = parse_quote! {
            #[validate(unknown)]
            value: String
        };

        let err = parse_validate_attrs(&field.attrs, Some("value")).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported #[validate(...)] rule"));
        assert!(message.contains("supported rules"));
    }

    #[test]
    fn test_parse_serde_rename_all() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(GearMesh)]
            #[serde(rename_all = "camelCase")]
            struct User {
                user_name: String,
            }
        };

        let attrs = parse_gear_mesh_attrs(&input.attrs).unwrap();
        assert_eq!(attrs.serde.rename_all, Some(RenameRule::CamelCase));
    }

    #[test]
    fn test_invalid_serde_rename_all_reports_supported_values() {
        let input: syn::DeriveInput = parse_quote! {
            #[derive(GearMesh)]
            #[serde(rename_all = "train-case")]
            struct User {
                user_name: String,
            }
        };

        let err = parse_gear_mesh_attrs(&input.attrs).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("invalid `serde(rename_all = ...)` value"));
        assert!(message.contains("supported values"));
    }

    #[test]
    fn test_invalid_range_option_reports_supported_options() {
        let field: syn::Field = parse_quote! {
            #[validate(range(step = 1))]
            value: i32
        };

        let err = parse_validate_attrs(&field.attrs, Some("value")).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported `range(...)` option"));
        assert!(message.contains("supported options are `min = ...` and `max = ...`"));
    }

    #[test]
    fn test_invalid_length_option_reports_supported_options() {
        let field: syn::Field = parse_quote! {
            #[validate(length(exact = 5))]
            value: String
        };

        let err = parse_validate_attrs(&field.attrs, Some("value")).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported `length(...)` option"));
        assert!(message.contains("supported options are `min = ...` and `max = ...`"));
    }

    #[test]
    fn test_parse_custom_message_and_cross_field_rules() {
        let field: syn::Field = parse_quote! {
            #[validate(custom = "validateUsername", message = "Username taken", cross_field(match = "password", message = "Passwords must match"))]
            password_confirmation: String
        };

        let rules = parse_validate_attrs(&field.attrs, Some("password_confirmation")).unwrap();
        assert!(matches!(
            &rules[0],
            ValidationRule::Custom {
                name,
                message: Some(message),
            } if name == "validateUsername" && message == "Username taken"
        ));
        assert!(matches!(
            &rules[1],
            ValidationRule::CrossField {
                fields,
                rule: CrossFieldRule::Match,
                message: Some(message),
                ..
            } if fields == &vec!["password_confirmation".to_string(), "password".to_string()]
                && message == "Passwords must match"
        ));
    }

    #[test]
    fn test_cross_field_match_requires_other_field() {
        let field: syn::Field = parse_quote! {
            #[validate(cross_field(match = ""))]
            password_confirmation: String
        };

        let err = parse_validate_attrs(&field.attrs, Some("password_confirmation")).unwrap_err();
        assert!(err.to_string().contains("requires a non-empty field name"));
    }
}
