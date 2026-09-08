//! gear-mesh-derive: proc-macro for GearMesh derive
//!
//! このクレートは `#[derive(GearMesh)]` マクロを提供します。

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod attributes;
mod error;
mod parser;
mod state;

use parser::parse_type;
use state::parse_state_container;

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
#[proc_macro_derive(GearMesh, attributes(gear_mesh, validate, serde))]
pub fn derive_gear_mesh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parse_type(&input) {
        Ok(gear_mesh_type) => {
            let name = &input.ident;
            let type_json = match serde_json::to_string(&gear_mesh_type) {
                Ok(json) => json,
                Err(err) => {
                    let err = syn::Error::new_spanned(
                        name,
                        format!("failed to serialize GearMeshType for derive output: {err}"),
                    );
                    return TokenStream::from(err.to_compile_error());
                }
            };

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

                // Register type with inventory for automatic collection
                ::gear_mesh::inventory::submit! {
                    ::gear_mesh::TypeInfo {
                        get_type: || <#name as ::gear_mesh::GearMeshExport>::gear_mesh_type(),
                        type_name: stringify!(#name),
                    }
                }
            };

            // If output path is specified, trigger automatic generation
            let output_trigger = if let Some(output_path) = &gear_mesh_type.attributes.output_path {
                quote! {
                    // Trigger automatic type generation on first use
                    const _: () = {
                        ::gear_mesh::register_output(#output_path);
                    };
                }
            } else {
                quote! {}
            };

            let final_output = quote! {
                #expanded
                #output_trigger
            };

            TokenStream::from(final_output)
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

/// GearMeshState derive macro
///
/// 状態コンテナのフィールドを状態スロットとして宣言します。
/// スロットの定義単位は型ではなく、キーで識別されるスロットそのものです。
///
/// # 属性
///
/// - `#[state]`: フィールド名をキーにした読み取り専用スロット
/// - `#[state(key = "...")]`: キーを明示する
/// - `#[state(get = "...")]`: 既存の読み取りコマンド名を採用する
/// - `#[state(event = "...")]`: 既存の変更イベント名を採用する
/// - `#[state(readonly)]`: 読み取り専用であることを明示する
///
/// `get` と `event` を省略した場合、名前はキーから導出されます。
///
/// # Example
///
/// ```ignore
/// use gear_mesh::GearMeshState;
///
/// #[derive(GearMeshState)]
/// struct AppState {
///     /// ユーザーが選択したテーマ
///     #[state]
///     theme: Theme,
/// }
/// ```
#[proc_macro_derive(GearMeshState, attributes(state))]
pub fn derive_gear_mesh_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let slots = match parse_state_container(&input) {
        Ok(slots) => slots,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };

    let name = &input.ident;
    let slots_json = match serde_json::to_string(&slots) {
        Ok(json) => json,
        Err(err) => {
            let err = syn::Error::new_spanned(
                name,
                format!("failed to serialize state slots for derive output: {err}"),
            );
            return TokenStream::from(err.to_compile_error());
        }
    };

    TokenStream::from(quote! {
        impl ::gear_mesh::GearMeshStateExport for #name {
            fn gear_mesh_state_slots() -> ::std::vec::Vec<::gear_mesh::StateSlot> {
                let json = #slots_json;
                ::serde_json::from_str(json).expect("Failed to deserialize state slots")
            }

            fn state_container_name() -> &'static str {
                stringify!(#name)
            }
        }

        // Register the slots with inventory for automatic collection
        ::gear_mesh::inventory::submit! {
            ::gear_mesh::StateSlotInfo {
                get_slots: || <#name as ::gear_mesh::GearMeshStateExport>::gear_mesh_state_slots(),
                container_name: stringify!(#name),
            }
        }
    })
}
