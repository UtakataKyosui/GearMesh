//! gear-mesh-derive: proc-macro for GearMesh derive
//!
//! このクレートは `#[derive(GearMesh)]` マクロを提供します。

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod parser;

use parser::parse_type;

/// GearMesh derive macro
///
/// Rust型をTypeScriptに変換可能な中間表現に変換します。
///
/// # 属性
///
/// - `#[gear_mesh(branded)]`: Branded Typeとして生成
/// - `#[gear_mesh(validate)]`: バリデーション関数を生成
/// - `#[gear_mesh(bigint = "auto")]`: BigInt自動変換を有効化
///
/// # Example
///
/// ```ignore
/// use gear_mesh::GearMesh;
///
/// #[derive(GearMesh)]
/// #[gear_mesh(branded)]
/// struct UserId(i32);
///
/// #[derive(GearMesh)]
/// struct User {
///     id: UserId,
///     name: String,
/// }
/// ```
#[proc_macro_derive(GearMesh, attributes(gear_mesh, validate))]
pub fn derive_gear_mesh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parse_type(&input) {
        Ok(gear_mesh_type) => {
            let name = &input.ident;
            let type_json = serde_json::to_string(&gear_mesh_type).unwrap_or_default();

            let expanded = quote! {
                impl ::gear_mesh::GearMeshExport for #name {
                    fn gear_mesh_type() -> ::gear_mesh::GearMeshType {
                        let json = #type_json;
                        ::serde_json::from_str(json).expect("Failed to deserialize GearMeshType")
                    }

                    fn type_name() -> &'static str {
                        stringify!(#name)
                    }
                }
            };

            TokenStream::from(expanded)
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
