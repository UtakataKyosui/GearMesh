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
                } else if meta.path.is_ident("bigint") {
                    if meta.input.peek(syn::Token![=]) {
                        let _ = meta.input.parse::<syn::Token![=]>()?;
                        let value: syn::LitStr = meta.input.parse()?;
                        if value.value() == "auto" {
                            result.bigint_auto = true;
                        }
                    }
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
                            // 整数か浮動小数点数のいずれかをパース
                            if let Ok(lit) = inner.input.parse::<syn::LitFloat>() {
                                min = Some(lit.base10_parse::<f64>()?);
                            } else if let Ok(lit) = inner.input.parse::<syn::LitInt>() {
                                min = Some(lit.base10_parse::<i64>()? as f64);
                            }
                        } else if inner.path.is_ident("max") {
                            let _ = inner.input.parse::<syn::Token![=]>()?;
                            if let Ok(lit) = inner.input.parse::<syn::LitFloat>() {
                                max = Some(lit.base10_parse::<f64>()?);
                            } else if let Ok(lit) = inner.input.parse::<syn::LitInt>() {
                                max = Some(lit.base10_parse::<i64>()? as f64);
                            }
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
        if attr.path().is_ident("serde") {
            if let Meta::List(list) = &attr.meta {
                // シンプルな実装：rename = "..." を探す
                let tokens = list.tokens.to_string();
                if let Some(start) = tokens.find("rename") {
                    if let Some(eq_pos) = tokens[start..].find('=') {
                        let after_eq = &tokens[start + eq_pos + 1..];
                        if let Some(quote_start) = after_eq.find('"') {
                            if let Some(quote_end) = after_eq[quote_start + 1..].find('"') {
                                return Some(
                                    after_eq[quote_start + 1..quote_start + 1 + quote_end].to_string(),
                                );
                            }
                        }
                    }
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
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(nv) = &attr.meta {
                    if let Expr::Lit(expr_lit) = &nv.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}
