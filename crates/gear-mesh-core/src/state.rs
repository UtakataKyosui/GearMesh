//! 状態スロットの中間表現
//!
//! 状態値の定義単位は「型」ではなく「スロット」です。型は再利用可能な語彙、
//! スロットはアプリケーションがひとつだけ持つ入れ物であり、両者は別の概念です。
//! 設計の背景は `docs/STATE.md` を参照してください。

use serde::{Deserialize, Serialize};

use crate::{DocComment, RenameRule, TypeRef};

/// スロットに対してUI側が持つ権限
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotAccess {
    /// 読み取りと購読のみ。書き込み手段は生成されない。
    ReadOnly,
    /// 読み取り・購読に加えて書き込みを要求できる（フェーズ2で対応）。
    ReadWrite,
}

/// 状態スロットの中間表現
///
/// 値の型・読み取り手段・変更通知・（将来の）書き込み手段という、
/// ひとつの状態値に必要な知識をまとめて保持します。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSlot {
    /// スロットのキー。コマンド名とイベント名の導出元であり、唯一の正。
    pub key: String,
    /// 値の型
    pub ty: TypeRef,
    /// UI側の権限
    pub access: SlotAccess,
    /// 現在値を読み取るコマンド名
    pub get_command: String,
    /// 変更を通知するイベント名
    pub changed_event: String,
    /// 書き込みを要求するコマンド名（フェーズ2）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_command: Option<String>,
    /// このスロットを宣言している状態コンテナの名前
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// ドキュメントコメント
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<DocComment>,
}

impl StateSlot {
    /// 読み取り専用スロットを、キーから導出した名前で構築する
    pub fn read_only(key: impl Into<String>, ty: TypeRef) -> Self {
        let key = key.into();
        Self {
            get_command: derive_get_command(&key),
            changed_event: derive_changed_event(&key),
            key,
            ty,
            access: SlotAccess::ReadOnly,
            set_command: None,
            owner: None,
            docs: None,
        }
    }

    /// 既存のコマンド名を採用する
    pub fn with_get_command(mut self, command: impl Into<String>) -> Self {
        self.get_command = command.into();
        self
    }

    /// 既存のイベント名を採用する
    pub fn with_changed_event(mut self, event: impl Into<String>) -> Self {
        self.changed_event = event.into();
        self
    }

    /// 宣言元の状態コンテナを記録する
    pub fn with_owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(owner.into());
        self
    }

    /// ドキュメントコメントを付与する
    pub fn with_docs(mut self, docs: DocComment) -> Self {
        self.docs = Some(docs);
        self
    }

    /// UI側から書き込めないスロットかどうか
    pub fn is_read_only(&self) -> bool {
        matches!(self.access, SlotAccess::ReadOnly)
    }

    /// TypeScript側で公開する束縛名
    pub fn binding_name(&self) -> String {
        RenameRule::CamelCase.apply(&self.key)
    }
}

/// キーから読み取りコマンド名を導出する
pub fn derive_get_command(key: &str) -> String {
    format!("state://{key}/get")
}

/// キーから変更イベント名を導出する
pub fn derive_changed_event(key: &str) -> String {
    format!("state://{key}/changed")
}

/// スロットキーとして受け付けられない値
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid state slot key `{key}`: {reason}")]
pub struct InvalidSlotKey {
    /// 与えられたキー
    pub key: String,
    /// 受け付けられない理由
    pub reason: &'static str,
}

/// スロットキーを検証する
///
/// キーはイベント名とTypeScriptの識別子の両方の元になるため、
/// `snake_case` のASCII英数字と `_` に限定します。
pub fn validate_slot_key(key: &str) -> Result<(), InvalidSlotKey> {
    let invalid = |reason| {
        Err(InvalidSlotKey {
            key: key.to_string(),
            reason,
        })
    };

    let Some(first) = key.chars().next() else {
        return invalid("the key is empty");
    };

    if !(first.is_ascii_lowercase() || first == '_') {
        return invalid("the key must start with a lowercase ASCII letter or `_`");
    }

    if !key
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return invalid("the key may only contain lowercase ASCII letters, digits, and `_`");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_slot_derives_names_from_the_key() {
        let slot = StateSlot::read_only("effective_theme", TypeRef::new("ResolvedTheme"));

        assert_eq!(slot.get_command, "state://effective_theme/get");
        assert_eq!(slot.changed_event, "state://effective_theme/changed");
        assert_eq!(slot.binding_name(), "effectiveTheme");
        assert!(slot.is_read_only());
        assert!(slot.set_command.is_none());
    }

    #[test]
    fn explicit_names_override_the_derived_ones() {
        let slot = StateSlot::read_only("theme", TypeRef::new("Theme"))
            .with_get_command("get_theme")
            .with_changed_event("theme-changed");

        assert_eq!(slot.get_command, "get_theme");
        assert_eq!(slot.changed_event, "theme-changed");
        assert_eq!(slot.key, "theme", "the key stays the source of truth");
    }

    #[test]
    fn slot_keys_are_restricted_to_snake_case_ascii() {
        assert!(validate_slot_key("theme").is_ok());
        assert!(validate_slot_key("window_geometry_2").is_ok());
        assert!(validate_slot_key("_internal").is_ok());

        assert!(validate_slot_key("").is_err());
        assert!(validate_slot_key("Theme").is_err());
        assert!(validate_slot_key("2theme").is_err());
        assert!(validate_slot_key("theme-changed").is_err());
        assert!(validate_slot_key("テーマ").is_err());
    }

    #[test]
    fn slots_round_trip_through_json() {
        let slot = StateSlot::read_only("theme", TypeRef::new("Theme")).with_owner("AppState");

        let json = serde_json::to_string(&slot).unwrap();
        let restored: StateSlot = serde_json::from_str(&json).unwrap();

        assert_eq!(restored.key, "theme");
        assert_eq!(restored.ty.name, "Theme");
        assert_eq!(restored.owner.as_deref(), Some("AppState"));
        assert_eq!(restored.access, SlotAccess::ReadOnly);
    }
}
