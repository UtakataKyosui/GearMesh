//! 属性パーサー
//!
//! `#[gear_mesh(...)]` と `#[validate(...)]` 属性を解析します。

use syn::{Attribute, Expr, Lit, Meta, Result};

use gear_mesh_core::{TypeAttributes, ValidationRule};

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

    Ok(result)
}

/// validate属性を解析
pub fn parse_validate_attrs(attrs: &[Attribute]) -> Result<Vec<ValidationRule>> {
    let mut rules = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("validate") {
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
                } else if meta.path.is_ident("email") {
                    rules.push(ValidationRule::Email);
                } else if meta.path.is_ident("url") {
                    rules.push(ValidationRule::Url);
                } else if meta.path.is_ident("pattern") {
                    let _ = meta.input.parse::<syn::Token![=]>()?;
                    let lit: syn::LitStr = meta.input.parse()?;
                    rules.push(ValidationRule::Pattern(lit.value()));
                } else {
                    return Err(meta.error(
                        "unsupported #[validate(...)] rule\nhelp: supported rules are `range`, `length`, `email`, `url`, and `pattern = \"...\"`",
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
            #[validate(custom = "nope")]
            value: String
        };

        let err = parse_validate_attrs(&field.attrs).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported #[validate(...)] rule"));
        assert!(message.contains("supported rules"));
    }
}
