# gear-mesh v0.1.0 リリースノート

## 🎉 初回リリース完了

**gear-mesh** v0.1.0が完成しました！

## 📦 実装済み機能（フェーズ1 MVP）

### ✅ コア機能
- **基本型変換**: struct, enum, newtype, primitives
- **Branded Type生成**: Rustのnewtypeパターン → TypeScript Branded Type
- **JSDocコメント変換**: Rust doc comments → JSDoc
- **BigInt自動対応**: u64/i64 → bigint
- **バリデーション**: 基本的なバリデーションルール定義

### ✅ 開発ツール
- **CLI**: generate, watch, init コマンド
- **ウォッチモード**: ファイル変更の自動検知
- **設定管理**: TOML形式の設定ファイル

### ✅ 品質保証
- **41のテスト**: ユニット + 統合テスト
- **E2Eテスト**: Dockerベースの完全テスト
- **100%テスト成功率**

## 📊 プロジェクト統計

| 項目 | 数値 |
|------|------|
| クレート数 | 5 |
| テスト数 | 41 + 1 E2E |
| ドキュメントページ | 6 |
| コード行数 | ~3,000行 |

## 🚀 使用方法

```bash
# インストール
cargo add gear-mesh

# 型定義生成
gear-mesh generate

# ウォッチモード
gear-mesh watch
```

## 📝 ドキュメント

- [README.md](../README.md) - クイックスタート
- [TESTING.md](TESTING.md) - テスト詳細
- [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) - プロジェクト構造
- [CONTRIBUTING.md](../CONTRIBUTING.md) - 開発ガイド

## 🔮 今後の予定（フェーズ2以降）

- [ ] プラグインシステム
- [ ] マイグレーション支援
- [ ] 双方向同期（実験的）
- [ ] IDE統合
- [ ] マルチ言語対応

## 🙏 謝辞

このプロジェクトは、既存の優れたライブラリ（ts-rs、typeshare、specta）からインスピレーションを得ています。

---

**gear-mesh** - Next-generation Rust to TypeScript type sharing library
