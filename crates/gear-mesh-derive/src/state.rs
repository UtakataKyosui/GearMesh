//! 状態スロットのパーサー
//!
//! `#[derive(GearMeshState)]` を付けた構造体のフィールドから、
//! `#[state(...)]` が付いたものだけを状態スロットとして取り出します。

use std::collections::BTreeMap;

use syn::{Attribute, Data, DeriveInput, Fields, Result};

use gear_mesh_core::{DocComment, SlotAccess, StateSlot, derive_changed_event, derive_get_command};

use crate::attributes::extract_doc_comments;
use crate::parser::parse_type_ref;

/// `#[state(...)]` で指定できる内容
#[derive(Default)]
struct SlotOptions {
    key: Option<String>,
    get_command: Option<String>,
    changed_event: Option<String>,
}

/// 状態コンテナからスロット一覧を取り出す
pub fn parse_state_container(input: &DeriveInput) -> Result<Vec<StateSlot>> {
    let container = input.ident.to_string();

    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "#[derive(GearMeshState)] is only supported on structs\nhelp: declare state slots on the struct that holds the application state",
        ));
    };

    let Fields::Named(named) = &data.fields else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "#[derive(GearMeshState)] requires named fields\nhelp: a state slot is identified by its field name",
        ));
    };

    let mut slots = Vec::new();
    let mut seen: BTreeMap<String, ()> = BTreeMap::new();

    for field in &named.named {
        let Some(attr) = find_state_attr(&field.attrs) else {
            continue;
        };

        let field_name = field
            .ident
            .as_ref()
            .expect("named fields always have an identifier")
            .to_string();

        let options = parse_slot_options(attr)?;
        let key = options.key.unwrap_or(field_name);

        if let Err(err) = gear_mesh_core::validate_slot_key(&key) {
            return Err(syn::Error::new_spanned(
                attr,
                format!("{err}\nhelp: use a snake_case key such as `window_geometry`"),
            ));
        }

        if seen.insert(key.clone(), ()).is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                format!(
                    "duplicate state slot key `{key}` in `{container}`\nhelp: each state value owns exactly one slot; give this one a different `key = \"...\"`"
                ),
            ));
        }

        let docs = extract_doc_comments(&field.attrs);
        let mut slot = StateSlot {
            get_command: options
                .get_command
                .unwrap_or_else(|| derive_get_command(&key)),
            changed_event: options
                .changed_event
                .unwrap_or_else(|| derive_changed_event(&key)),
            key,
            ty: parse_type_ref(&field.ty)?,
            access: SlotAccess::ReadOnly,
            set_command: None,
            owner: Some(container.clone()),
            docs: None,
        };

        if !docs.is_empty() {
            slot.docs = Some(DocComment::parse(&docs));
        }

        slots.push(slot);
    }

    if slots.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "#[derive(GearMeshState)] found no state slots\nhelp: mark the fields the UI projects with `#[state]`",
        ));
    }

    Ok(slots)
}

fn find_state_attr(attrs: &[Attribute]) -> Option<&Attribute> {
    attrs.iter().find(|attr| attr.path().is_ident("state"))
}

fn parse_slot_options(attr: &Attribute) -> Result<SlotOptions> {
    let mut options = SlotOptions::default();

    // `#[state]` 単体は「フィールド名をキーにした読み取り専用スロット」を意味する。
    if matches!(attr.meta, syn::Meta::Path(_)) {
        return Ok(options);
    }

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("readonly") {
            // フェーズ1では読み取り専用が唯一のアクセス権であり、`readonly` は明示のための指定。
            Ok(())
        } else if meta.path.is_ident("set") {
            Err(meta.error(
                "writable state slots are not supported yet\nhelp: phase 1 generates read-only projections; drop `set = \"...\"` for now (see docs/STATE.md)",
            ))
        } else if meta.path.is_ident("key") {
            options.key = Some(parse_string_value(&meta)?);
            Ok(())
        } else if meta.path.is_ident("get") {
            options.get_command = Some(parse_string_value(&meta)?);
            Ok(())
        } else if meta.path.is_ident("event") {
            options.changed_event = Some(parse_string_value(&meta)?);
            Ok(())
        } else {
            Err(meta.error(
                "unsupported #[state(...)] option\nhelp: supported options are `key = \"...\"`, `get = \"...\"`, `event = \"...\"`, and `readonly`",
            ))
        }
    })?;

    Ok(options)
}

fn parse_string_value(meta: &syn::meta::ParseNestedMeta) -> Result<String> {
    let value: syn::LitStr = meta.value()?.parse()?;
    let value = value.value();

    if value.is_empty() {
        return Err(meta.error("the value must not be empty"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Result<Vec<StateSlot>> {
        parse_state_container(&syn::parse_str::<DeriveInput>(source).unwrap())
    }

    fn error(source: &str) -> String {
        parse(source).unwrap_err().to_string()
    }

    #[test]
    fn only_annotated_fields_become_slots() {
        let slots = parse(
            r#"
            struct AppState {
                /// The theme the user selected.
                #[state]
                theme: Theme,
                window: WindowHandle,
            }
            "#,
        )
        .unwrap();

        assert_eq!(slots.len(), 1);
        assert_eq!(slots[0].key, "theme");
        assert_eq!(slots[0].ty.name, "Theme");
        assert_eq!(slots[0].owner.as_deref(), Some("AppState"));
        assert_eq!(slots[0].get_command, "state://theme/get");
        assert_eq!(slots[0].changed_event, "state://theme/changed");
        assert!(slots[0].docs.is_some());
    }

    #[test]
    fn explicit_options_override_the_derived_names() {
        let slots = parse(
            r#"
            struct AppState {
                #[state(key = "theme", get = "get_theme", event = "theme-changed", readonly)]
                selected: Theme,
            }
            "#,
        )
        .unwrap();

        assert_eq!(slots[0].key, "theme");
        assert_eq!(slots[0].get_command, "get_theme");
        assert_eq!(slots[0].changed_event, "theme-changed");
        assert!(slots[0].is_read_only());
    }

    #[test]
    fn writable_slots_are_rejected_until_phase_two() {
        let message = error(
            r#"
            struct AppState {
                #[state(set = "set_theme")]
                theme: Theme,
            }
            "#,
        );

        assert!(message.contains("not supported yet"), "{message}");
        assert!(message.contains("docs/STATE.md"), "{message}");
    }

    #[test]
    fn one_state_value_owns_exactly_one_slot() {
        let message = error(
            r#"
            struct AppState {
                #[state(key = "theme")]
                selected: Theme,
                #[state(key = "theme")]
                also_selected: Theme,
            }
            "#,
        );

        assert!(
            message.contains("duplicate state slot key `theme`"),
            "{message}"
        );
    }

    #[test]
    fn keys_must_be_usable_as_event_names_and_identifiers() {
        let message = error(
            r#"
            struct AppState {
                #[state(key = "Theme-Changed")]
                theme: Theme,
            }
            "#,
        );

        assert!(message.contains("invalid state slot key"), "{message}");
    }

    #[test]
    fn a_container_without_slots_is_a_mistake() {
        let message = error(
            r#"
            struct AppState {
                theme: Theme,
            }
            "#,
        );

        assert!(message.contains("no state slots"), "{message}");
    }

    #[test]
    fn slots_are_declared_on_structs_with_named_fields() {
        assert!(error("enum AppState { Theme }").contains("only supported on structs"));
        assert!(error("struct AppState(Theme);").contains("requires named fields"));
    }

    #[test]
    fn unknown_options_are_reported_with_the_supported_set() {
        let message = error(
            r#"
            struct AppState {
                #[state(persist)]
                theme: Theme,
            }
            "#,
        );

        assert!(
            message.contains("unsupported #[state(...)] option"),
            "{message}"
        );
    }
}
