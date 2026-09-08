//! 状態スロットの自動収集
//!
//! `#[derive(GearMeshState)]` を付けた状態コンテナが宣言したスロットを
//! inventory 経由で集め、TypeScriptの投影を生成します。

use std::collections::BTreeMap;
use std::io;

use crate::StateSlot;

/// inventory に登録される状態スロットの情報
pub struct StateSlotInfo {
    /// 状態コンテナが宣言したスロット
    pub get_slots: fn() -> Vec<StateSlot>,
    /// 状態コンテナの名前
    pub container_name: &'static str,
}

inventory::collect!(StateSlotInfo);

/// 登録済みの状態スロットをキー順に集める
///
/// ひとつの状態値がふたつのスロットを持つことはないため、
/// キーが衝突した場合はエラーになります。
pub fn collect_registered_slots() -> io::Result<Vec<StateSlot>> {
    merge_slots(
        inventory::iter::<StateSlotInfo>().map(|info| (info.container_name, (info.get_slots)())),
    )
}

/// 状態コンテナごとのスロットをキー順にまとめる
///
/// ひとつの状態値がふたつのスロットを持つことはないため、
/// キーが衝突した場合はエラーになります。
fn merge_slots<'a>(
    containers: impl Iterator<Item = (&'a str, Vec<StateSlot>)>,
) -> io::Result<Vec<StateSlot>> {
    let mut by_key: BTreeMap<String, StateSlot> = BTreeMap::new();
    let mut by_binding: BTreeMap<String, String> = BTreeMap::new();

    for (container_name, slots) in containers {
        for slot in slots {
            gear_mesh_core::validate_slot_key(&slot.key)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

            if let Some(existing) = by_key.get(&slot.key) {
                let previous = existing.owner.as_deref().unwrap_or("<unknown>");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "duplicate state slot key `{}`: declared by both `{}` and `{}`",
                        slot.key, previous, container_name
                    ),
                ));
            }

            // Distinct keys can still camel-case to the same TypeScript binding
            // (`a_b` and `a__b` both become `aB`), which would emit two consts
            // of the same name.
            let binding = slot.binding_name();
            if let Some(previous_key) = by_binding.get(&binding) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "state slot keys `{}` and `{}` both project to the TypeScript binding `{}`",
                        previous_key, slot.key, binding
                    ),
                ));
            }

            by_binding.insert(binding, slot.key.clone());
            by_key.insert(slot.key.clone(), slot);
        }
    }

    Ok(by_key.into_values().collect())
}

/// 登録済みの状態スロットからTypeScriptの投影を生成する
///
/// `types_module` を渡すと、値の型をそのモジュールから取り込みます。
///
/// # Example
///
/// ```no_run
/// use gear_mesh::generate_state;
///
/// generate_state("../frontend/src/types/state.ts", Some("./index"))
///     .expect("Failed to generate state slots");
/// ```
pub fn generate_state(
    output_path: impl AsRef<std::path::Path>,
    types_module: Option<&str>,
) -> io::Result<()> {
    generate_state_with_config(output_path, crate::GeneratorConfig::new(), types_module)
}

/// 設定を指定して状態スロットの投影を生成する
pub fn generate_state_with_config(
    output_path: impl AsRef<std::path::Path>,
    config: crate::GeneratorConfig,
    types_module: Option<&str>,
) -> io::Result<()> {
    let slots = collect_registered_slots()?;
    if slots.is_empty() {
        eprintln!(
            "⚠️  Warning: No state slots found. Make sure you have #[derive(GearMeshState)] on your state container."
        );
        return Ok(());
    }

    let output_path = output_path.as_ref();
    let generator = crate::StateGenerator::new(config.clone());
    let output = generator.generate_with_types_module(&slots, types_module);

    let cache_path = crate::cache::cache_file(&config.cache_dir);
    let mut cache = if config.enable_cache {
        crate::cache::OutputCache::load(&cache_path)
    } else {
        crate::cache::OutputCache::default()
    };

    crate::inventory_collect::write_output(output_path, &output, config.enable_cache, &mut cache)?;

    if config.enable_cache {
        cache.persist(&cache_path)?;
    }

    println!(
        "✅ Generated TypeScript state slots: {}",
        output_path.display()
    );
    println!("   {} state slots exported", slots.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TypeRef;

    fn slot(key: &str, owner: &str) -> StateSlot {
        StateSlot::read_only(key, TypeRef::new("Theme")).with_owner(owner)
    }

    #[test]
    fn slots_from_several_containers_are_merged_in_key_order() {
        let merged = merge_slots(
            [
                ("Window", vec![slot("window_mode", "Window")]),
                ("AppState", vec![slot("theme", "AppState")]),
            ]
            .into_iter(),
        )
        .unwrap();

        let keys: Vec<_> = merged.iter().map(|slot| slot.key.as_str()).collect();
        assert_eq!(keys, vec!["theme", "window_mode"]);
    }

    #[test]
    fn one_state_value_cannot_be_claimed_by_two_containers() {
        let error = merge_slots(
            [
                ("AppState", vec![slot("theme", "AppState")]),
                ("Window", vec![slot("theme", "Window")]),
            ]
            .into_iter(),
        )
        .unwrap_err();

        let message = error.to_string();
        assert!(
            message.contains("duplicate state slot key `theme`"),
            "{message}"
        );
        assert!(message.contains("AppState"), "{message}");
        assert!(message.contains("Window"), "{message}");
    }

    #[test]
    fn keys_that_collide_as_typescript_bindings_are_rejected() {
        let error = merge_slots(
            [(
                "AppState",
                vec![slot("a_b", "AppState"), slot("a__b", "AppState")],
            )]
            .into_iter(),
        )
        .unwrap_err();

        let message = error.to_string();
        assert!(
            message.contains("project to the TypeScript binding `aB`"),
            "{message}"
        );
    }

    #[test]
    fn hand_built_slots_still_have_to_carry_a_usable_key() {
        let error = merge_slots(
            [(
                "AppState",
                vec![StateSlot::read_only("Theme-Changed", TypeRef::new("Theme"))],
            )]
            .into_iter(),
        )
        .unwrap_err();

        assert!(error.to_string().contains("invalid state slot key"));
    }
}
