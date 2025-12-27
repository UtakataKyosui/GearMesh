# Code Generation Tests

このドキュメントでは、`codegen_test.rs` で実装されているコード生成テストについて説明します。

## 概要

`codegen_test.rs` は、Rust型定義から期待されるTypeScriptコードが正しく生成されることを検証する包括的なテストスイートです。TDD（テスト駆動開発）の原則に従い、様々な型パターンに対する期待値を明確に定義しています。

**総テスト数**: 36テスト（28 passing + 8 ignored）

## テスト構成

### Phase 1: 基本型テスト (15テスト)

#### 1. プリミティブ型
- **`test_simple_struct_with_primitives`**: `f64` → `number`
- **`test_struct_with_string_types`**: `String` → `string`
- **`test_struct_with_boolean`**: `bool` → `boolean`
- **`test_struct_with_integer_types`**: `i8`, `i16`, `i32`, `u8`, `u16`, `u32` → `number`

#### 2. BigInt型
- **`test_struct_with_bigint_types`**: `i64`, `u64`, `i128`, `u128` の変換
  - `use_bigint = true`: `bigint`
  - `use_bigint = false`: `number`

### Branded Type テスト

#### 3. Branded Type生成
- **`test_branded_type_generation`**: 単一のBranded Type生成
  - `type Brand<T, B>` ヘルパーの生成
  - `export type UserId = Brand<number, "UserId">`
  - `export const UserId = (value: number): UserId => value as UserId`

- **`test_multiple_branded_types`**: 複数のBranded Type
  - Brandヘルパーが1回だけ生成されることを確認
  - 各型が独立して生成される

### コレクション型テスト

#### 4. Option型
- **`test_optional_fields`**: `Option<T>` → `T | null` + optional marker `?`

#### 5. Vec型
- **`test_vec_array_generation`**: `Vec<T>` → `T[]`

#### 6. HashMap型
- **`test_hashmap_generation`**: `HashMap<String, T>` → `Record<string, T>`

### Enum型テスト

#### 7. 列挙型
- **`test_simple_enum_generation`**: Unit variantsのEnum
  - `"Active" | "Inactive" | "Pending"`

### JSDocテスト

#### 8. ドキュメント生成
- **`test_jsdoc_generation`**: JSDoc有効時の生成
  - 型レベルのJSDoc（複数行形式）
  - フィールドレベルのJSDoc（単一行形式）

- **`test_jsdoc_disabled`**: JSDoc無効時
  - ドキュメントコメントが生成されないことを確認

### 複雑な型テスト

#### 9. ネストされた型
- **`test_nested_types`**: 
  - `Option<Vec<User>>` → `User[] | null`
  - `HashMap<String, String>` → `Record<string, string>`

#### 10. 実世界の例
- **`test_complex_real_world_example`**: 総合的なテスト
  - Branded Type (`UserId`)
  - 構造体 (`User`)
  - Optional fields
  - Vec型
  - JSDoc
  - すべての機能の統合

### Phase 2: Tuple型テスト (3テスト)

#### 11. Tuple型
- **`test_tuple_2_elements`**: 2要素タプル
  - `(f64, f64)` → `[number, number]`

- **`test_tuple_3_elements`**: 3要素以上のタプル
  - `(i32, String, bool)` → `[number, string, boolean]`

- **`test_nested_tuple`**: ネストされたタプル
  - `((i32, String), bool)` → `[[number, string], boolean]`

### Phase 3: Tagged Unions (4テスト)

#### 12. Enum with Data
- **`test_enum_with_tuple_variant`**: Tuple variant
  - `Result { Ok(T), Err(E) }` → `{ "Ok": T } | { "Err": E }`

- **`test_enum_with_struct_variant`**: Struct variant
  - `Message { Text { content: String } }` → `{ "Text": { content: string } }`

- **`test_enum_mixed_variants`**: Mixed variants
  - Unit, Tuple, Struct variantsの混在

- **`test_enum_internal_tagged`**: Internal tagged
  - `#[serde(tag = "type")]` → `{ type: "Create"; name: string }`

### Phase 4: Generic Types (3テスト)

#### 13. ジェネリック型
- **`test_generic_single_param`**: 単一型パラメータ
  - `Container<T>` → `interface Container<T>`

- **`test_generic_multiple_params`**: 複数型パラメータ
  - `Pair<T, U>` → `interface Pair<T, U>`

- **`test_generic_with_constraints`**: 制約付き
  - 制約は無視され、型パラメータのみ生成

### Phase 5: Validation Rules (5テスト - Ignored)

> **Note**: これらのテストは `#[ignore]` でマークされています。Validation関数生成機能の完全な実装後に有効化されます。

#### 14. バリデーションルール
- **`test_validation_range`**: 範囲チェック
- **`test_validation_length`**: 文字列長チェック
- **`test_validation_email`**: メールアドレス形式
- **`test_validation_url`**: URL形式
- **`test_validation_pattern`**: 正規表現パターン

### Phase 6: Zod Schema (3テスト - Ignored)

> **Note**: これらのテストは `#[ignore]` でマークされています。Zodスキーマ生成機能の完全な実装後に有効化されます。

#### 15. Zodスキーマ生成
- **`test_zod_basic_schema`**: 基本スキーマ
- **`test_zod_with_validation`**: Validation付き
- **`test_zod_optional_fields`**: Optional fields

### Phase 7: Serde Attributes (3テスト)

#### 16. Serde rename
- **`test_serde_rename_field`**: フィールドのrename
  - `user_id` → `userId`

- **`test_serde_rename_mixed`**: Mixed (renameあり/なし)
  - 一部のフィールドのみrename

- **`test_serde_rename_with_optional`**: Optionalとの組み合わせ
  - `email_address?` → `emailAddress?`


## テストヘルパー

### `assert_contains(output, expected)`
生成されたコードに期待される文字列が含まれることを検証します。

```rust
assert_contains(&output, "export interface User {");
```

### `assert_not_contains(output, unexpected)`
生成されたコードに期待されない文字列が含まれないことを検証します。

```rust
assert_not_contains(&output, "/**");  // JSDoc無効時
```

## テスト実行

### 全テスト実行
```bash
cargo test --test codegen_test
```

### 特定のテスト実行
```bash
cargo test --test codegen_test test_branded_type_generation
```

### ワークスペース全体のテスト
```bash
cargo test --workspace
```

## カバレッジ

このテストスイートは以下をカバーしています：

- ✅ すべてのプリミティブ型（数値、文字列、真偽値）
- ✅ BigInt型（有効/無効両方）
- ✅ Branded Type（単一/複数）
- ✅ Optional fields (`Option<T>`)
- ✅ コレクション型 (`Vec<T>`, `HashMap<K, V>`)
- ✅ Enum型（Unit variants）
- ✅ JSDoc生成（有効/無効）
- ✅ ネストされた型
- ✅ 実世界の複雑な例

## 期待される出力例

### Branded Type
```typescript
type Brand<T, B> = T & { readonly __brand: B };

export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;
```

### 構造体
```typescript
export interface User {
    id: number;
    name: string;
    email: string;
}
```

### Optional fields
```typescript
export interface User {
    id: number;
    email?: string | null;
    age?: number | null;
}
```

### JSDoc付き
```typescript
/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: number;
    /** User's display name */
    name: string;
}
```

## 今後の拡張

以下の型パターンのテストを追加する予定：

- [ ] Tuple型
- [ ] Enum with data (Tagged unions)
- [ ] Generic types
- [ ] Validation rules
- [ ] Zod schema generation
- [ ] Serde rename attributes
