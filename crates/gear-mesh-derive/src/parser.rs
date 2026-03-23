//! 型パーサー
//!
//! syn::DeriveInputからGearMeshTypeへ変換します。

use syn::{Data, DeriveInput, Fields, Result, Type};

use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType, GenericParam,
    NewtypeType, PrimitiveType, SerdeFieldAttrs, StructType, TypeAttributes, TypeKind, TypeRef,
    VariantContent,
};

use crate::attributes::{
    extract_doc_comments, parse_gear_mesh_attrs, parse_serde_rename, parse_validate_attrs,
};
use crate::error::{branded_requires_newtype, unsupported_generic_argument, unsupported_type};

/// DeriveInputからGearMeshTypeを生成
pub fn parse_type(input: &DeriveInput) -> Result<GearMeshType> {
    let name = input.ident.to_string();
    let attrs = parse_gear_mesh_attrs(&input.attrs)?;
    let docs = extract_doc_comments(&input.attrs);
    let doc_comment = if docs.is_empty() {
        None
    } else {
        Some(DocComment::parse(&docs))
    };

    let generics = input
        .generics
        .type_params()
        .map(|tp| GenericParam {
            name: tp.ident.to_string(),
            bounds: tp
                .bounds
                .iter()
                .map(|b| quote::quote!(#b).to_string())
                .collect(),
        })
        .collect();

    let kind = match &input.data {
        Data::Struct(data) => parse_struct(&data.fields, &attrs)?,
        Data::Enum(data) => {
            let variants = data
                .variants
                .iter()
                .map(parse_variant)
                .collect::<Result<Vec<_>>>()?;
            TypeKind::Enum(EnumType {
                variants,
                representation: EnumRepresentation::External, // デフォルト
            })
        }
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "GearMesh does not support unions",
            ));
        }
    };

    Ok(GearMeshType {
        name,
        kind,
        docs: doc_comment,
        generics,
        attributes: attrs,
    })
}

/// 構造体フィールドをパース
fn parse_struct(fields: &Fields, attrs: &TypeAttributes) -> Result<TypeKind> {
    match fields {
        Fields::Named(named) => {
            if attrs.branded {
                return Err(branded_requires_newtype(named));
            }

            let field_infos = named
                .named
                .iter()
                .map(|f| {
                    let name = f.ident.as_ref().unwrap().to_string();
                    let ty = parse_type_ref(&f.ty)?;
                    let docs = extract_doc_comments(&f.attrs);
                    let validations = parse_validate_attrs(&f.attrs, Some(&name))?;
                    let rename = parse_serde_rename(&f.attrs);
                    let optional = is_option_type(&f.ty);

                    Ok(FieldInfo {
                        name,
                        ty,
                        docs: if docs.is_empty() {
                            None
                        } else {
                            Some(DocComment::parse(&docs))
                        },
                        validations,
                        optional,
                        serde_attrs: SerdeFieldAttrs {
                            rename,
                            ..Default::default()
                        },
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(TypeKind::Struct(StructType {
                fields: field_infos,
            }))
        }
        Fields::Unnamed(unnamed) => {
            // タプル構造体
            if unnamed.unnamed.len() == 1 && attrs.branded {
                // newtypeパターン（Branded Type）
                let inner = parse_type_ref(&unnamed.unnamed[0].ty)?;
                Ok(TypeKind::Newtype(NewtypeType { inner }))
            } else {
                if attrs.branded {
                    return Err(branded_requires_newtype(unnamed));
                }

                let types = unnamed
                    .unnamed
                    .iter()
                    .map(|f| parse_type_ref(&f.ty))
                    .collect::<Result<Vec<_>>>()?;
                Ok(TypeKind::Tuple(types))
            }
        }
        Fields::Unit => {
            if attrs.branded {
                return Err(branded_requires_newtype(fields));
            }
            Ok(TypeKind::Primitive(PrimitiveType::Unit))
        }
    }
}

/// 列挙型バリアントをパース
fn parse_variant(variant: &syn::Variant) -> Result<EnumVariant> {
    let name = variant.ident.to_string();
    let docs = extract_doc_comments(&variant.attrs);

    let content = match &variant.fields {
        Fields::Unit => VariantContent::Unit,
        Fields::Unnamed(unnamed) => {
            let types = unnamed
                .unnamed
                .iter()
                .map(|f| parse_type_ref(&f.ty))
                .collect::<Result<Vec<_>>>()?;
            VariantContent::Tuple(types)
        }
        Fields::Named(named) => {
            let fields = named
                .named
                .iter()
                .map(|f| -> Result<FieldInfo> {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    let ty = parse_type_ref(&f.ty)?;
                    let field_docs = extract_doc_comments(&f.attrs);
                    let validations = parse_validate_attrs(&f.attrs, Some(&field_name))?;
                    let rename = parse_serde_rename(&f.attrs);

                    Ok(FieldInfo {
                        name: field_name,
                        ty,
                        docs: if field_docs.is_empty() {
                            None
                        } else {
                            Some(DocComment::parse(&field_docs))
                        },
                        validations,
                        optional: is_option_type(&f.ty),
                        serde_attrs: SerdeFieldAttrs {
                            rename,
                            ..Default::default()
                        },
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            VariantContent::Struct(fields)
        }
    };

    Ok(EnumVariant {
        name,
        content,
        docs: if docs.is_empty() {
            None
        } else {
            Some(DocComment::parse(&docs))
        },
    })
}

/// syn::TypeからTypeRefへ変換
fn parse_type_ref(ty: &Type) -> Result<TypeRef> {
    match ty {
        Type::Path(path) => {
            let segments: Vec<_> = path.path.segments.iter().collect();

            if segments.is_empty() {
                return Err(unsupported_type(ty));
            }

            let last = segments.last().unwrap();
            let name = last.ident.to_string();

            // ジェネリクス引数を処理
            let generics = match &last.arguments {
                syn::PathArguments::AngleBracketed(args) => args
                    .args
                    .iter()
                    .map(|arg| {
                        if let syn::GenericArgument::Type(ty) = arg {
                            parse_type_ref(ty)
                        } else {
                            Err(unsupported_generic_argument(arg))
                        }
                    })
                    .collect::<Result<Vec<_>>>()?,
                _ => Vec::new(),
            };

            Ok(TypeRef::with_generics(name, generics))
        }
        Type::Reference(reference) => parse_type_ref(&reference.elem),
        Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                Ok(TypeRef::new("()"))
            } else {
                let generics = tuple
                    .elems
                    .iter()
                    .map(parse_type_ref)
                    .collect::<Result<Vec<_>>>()?;
                Ok(TypeRef::with_generics("__tuple__", generics))
            }
        }
        Type::Array(array) => {
            let elem = parse_type_ref(&array.elem)?;
            Ok(TypeRef::with_generics("__array__", vec![elem]))
        }
        Type::Slice(slice) => {
            let elem = parse_type_ref(&slice.elem)?;
            Ok(TypeRef::with_generics("__slice__", vec![elem]))
        }
        _ => Err(unsupported_type(ty)),
    }
}

/// Option<T>かどうかを判定
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(path) = ty
        && let Some(segment) = path.path.segments.last()
    {
        return segment.ident == "Option";
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::{RenameRule, TypeKind};
    use syn::parse_quote;

    #[test]
    fn test_unsupported_field_type_reports_actionable_error() {
        let input: DeriveInput = parse_quote! {
            struct CallbackHolder {
                callback: fn(i32) -> i32,
            }
        };

        let err = parse_type(&input).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("unsupported type for #[derive(GearMesh)]"));
        assert!(message.contains("supported generic such as Option<T> or Vec<T>"));
    }

    #[test]
    fn test_branded_requires_single_field_tuple_struct() {
        let input: DeriveInput = parse_quote! {
            #[gear_mesh(branded)]
            struct UserId {
                value: i32,
            }
        };

        let err = parse_type(&input).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("single-field tuple structs"));
        assert!(message.contains("struct UserId(i32);"));
    }

    #[test]
    fn test_variant_validation_errors_are_not_swallowed() {
        let input: DeriveInput = parse_quote! {
            enum ApiResponse {
                Invalid {
                    #[validate(range(min = "oops"))]
                    code: i32,
                }
            }
        };

        let err = parse_type(&input).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("invalid `range(min = ...)` value"));
    }

    #[test]
    fn test_parse_type_preserves_serde_rename_all_and_variant_field_rename() {
        let input: DeriveInput = parse_quote! {
            #[derive(GearMesh)]
            #[serde(rename_all = "camelCase")]
            enum ApiResponse {
                UserCreated {
                    #[serde(rename = "user-id")]
                    user_id: i32,
                }
            }
        };

        let ty = parse_type(&input).unwrap();
        assert_eq!(ty.attributes.serde.rename_all, Some(RenameRule::CamelCase));

        let TypeKind::Enum(enum_type) = ty.kind else {
            panic!("expected enum type");
        };
        let VariantContent::Struct(fields) = &enum_type.variants[0].content else {
            panic!("expected struct variant");
        };
        assert_eq!(fields[0].serde_attrs.rename.as_deref(), Some("user-id"));
    }
}
