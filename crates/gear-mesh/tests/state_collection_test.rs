//! 状態スロットの宣言から、収集・生成までを通しで確認する

use gear_mesh::{
    GearMesh, GearMeshState, GearMeshStateExport, GeneratorConfig, Revision, StateGenerator,
    collect_registered_slots,
};
use serde::{Deserialize, Serialize};

/// ユーザーが選択したテーマ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, GearMesh)]
enum Theme {
    Light,
    Dark,
    System,
}

/// 実際に描画されるテーマ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, GearMesh)]
enum ResolvedTheme {
    Light,
    Dark,
}

// フィールドはスロット宣言のためだけに存在し、テストからは読まれない。
#[allow(dead_code)]
#[derive(GearMeshState)]
struct AppState {
    /// ユーザーが選択したテーマ
    #[state]
    theme: Theme,

    /// OS設定を解決した実効テーマ
    #[state(readonly)]
    effective_theme: ResolvedTheme,

    /// 投影しない内部状態
    listeners: usize,
}

#[test]
fn a_container_declares_one_slot_per_annotated_field() {
    let slots = AppState::gear_mesh_state_slots();

    assert_eq!(AppState::state_container_name(), "AppState");
    assert_eq!(slots.len(), 2, "`listeners` is not projected");

    let theme = slots.iter().find(|s| s.key == "theme").unwrap();
    assert_eq!(theme.ty.name, "Theme");
    assert_eq!(theme.get_command, "state://theme/get");
    assert_eq!(theme.changed_event, "state://theme/changed");
    assert!(theme.is_read_only());
    assert!(theme.set_command.is_none());
}

#[test]
fn slots_are_collected_by_key() {
    let slots = collect_registered_slots().expect("slot keys should be unique");
    let keys: Vec<_> = slots.iter().map(|slot| slot.key.as_str()).collect();

    assert_eq!(keys, vec!["effective_theme", "theme"]);
}

#[test]
fn collected_slots_generate_a_read_only_projection() {
    let slots = collect_registered_slots().unwrap();
    let output = StateGenerator::new(GeneratorConfig::new())
        .generate_with_types_module(&slots, Some("./index"));

    assert!(output.contains("import type { ResolvedTheme, Theme } from './index';"));
    assert!(output.contains("export const theme: ReadonlyStateSlot<Theme> = readonlySlot<Theme>("));
    assert!(output.contains(
        "export const effectiveTheme: ReadonlyStateSlot<ResolvedTheme> = readonlySlot<ResolvedTheme>("
    ));
    assert!(!output.contains("set("), "phase 1 generates no writers");
}

#[test]
fn revisions_order_a_read_against_a_change() {
    let revision = Revision::new();

    // 購読側が受け取る変更通知
    let change = revision.bump(Theme::Dark);
    // 変更より後に解決した初回読み取り。値は古いが、リビジョンで判別できる。
    let stale_read = gear_mesh::Stamped::new(0, Theme::Light);

    assert!(stale_read.rev < change.rev);
    assert_eq!(revision.stamp(Theme::Dark).rev, change.rev);
}
