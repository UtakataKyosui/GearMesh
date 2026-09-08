//! リビジョン付きの状態値
//!
//! 状態値を購読するUIは、購読の開始と初回の読み取りを同時に行います。
//! 読み取りが変更イベントより後に解決すると、UIは古い値で新しい値を
//! 上書きしてしまいます。値にリビジョンを添えることで、受け取り側は
//! 「今まで見た中で最大のリビジョンより古いものを捨てる」だけで済み、
//! 状態値ごとに手書きのフラグを置く必要がなくなります。

use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

/// リビジョンを添えた状態値
///
/// `rev` はJSONの数値として送られ、TypeScript側では `number` になります。
/// 単調増加のカウンタなので、実用上 `Number.MAX_SAFE_INTEGER` には届きません。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stamped<T> {
    /// この値が生成された時点のリビジョン
    pub rev: u64,
    /// 状態値そのもの
    pub value: T,
}

impl<T> Stamped<T> {
    /// リビジョンと値から構築する
    pub fn new(rev: u64, value: T) -> Self {
        Self { rev, value }
    }

    /// リビジョンを保ったまま値を変換する
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Stamped<U> {
        Stamped {
            rev: self.rev,
            value: f(self.value),
        }
    }
}

/// 状態スロットのリビジョンカウンタ
///
/// 読み取りには [`Revision::stamp`]、変更通知には [`Revision::bump`] を使います。
/// 変更のたびにリビジョンを進めることが、UI側の順序保証の前提になります。
#[derive(Debug, Default)]
pub struct Revision(AtomicU64);

impl Revision {
    /// リビジョン0のカウンタを作る
    pub const fn new() -> Self {
        Self(AtomicU64::new(0))
    }

    /// 現在のリビジョンを返す
    pub fn current(&self) -> u64 {
        self.0.load(Ordering::Acquire)
    }

    /// リビジョンを進めて、進めた後の値を返す
    pub fn advance(&self) -> u64 {
        self.0.fetch_add(1, Ordering::AcqRel) + 1
    }

    /// 現在のリビジョンで値を包む（読み取り用）
    pub fn stamp<T>(&self, value: T) -> Stamped<T> {
        Stamped::new(self.current(), value)
    }

    /// リビジョンを進めて値を包む（変更通知用）
    pub fn bump<T>(&self, value: T) -> Stamped<T> {
        Stamped::new(self.advance(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stamp_reads_do_not_advance_the_revision() {
        let revision = Revision::new();

        assert_eq!(revision.stamp("light").rev, 0);
        assert_eq!(revision.stamp("light").rev, 0);
        assert_eq!(revision.current(), 0);
    }

    #[test]
    fn bump_advances_once_per_change() {
        let revision = Revision::new();

        assert_eq!(revision.bump("dark").rev, 1);
        assert_eq!(revision.bump("light").rev, 2);
        assert_eq!(revision.stamp("light").rev, 2, "a read follows the change");
    }

    #[test]
    fn map_keeps_the_revision() {
        let stamped = Stamped::new(7, "dark").map(str::to_uppercase);

        assert_eq!(stamped.rev, 7);
        assert_eq!(stamped.value, "DARK");
    }

    #[test]
    fn stamped_values_round_trip_through_json() {
        let json = serde_json::to_string(&Stamped::new(3, "dark")).unwrap();
        assert_eq!(json, r#"{"rev":3,"value":"dark"}"#);

        let restored: Stamped<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.rev, 3);
        assert_eq!(restored.value, "dark");
    }
}
