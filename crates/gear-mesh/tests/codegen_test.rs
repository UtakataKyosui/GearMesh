//! コード生成テスト: Rust型から期待されるTypeScript型が生成されることを検証
//!
//! このテストスイートは、様々なRust型パターンに対して、
//! 正しいTypeScriptコードが生成されることを保証します。

use gear_mesh_core::*;
use gear_mesh_generator::{GeneratorConfig, TypeScriptGenerator};

/// テストヘルパー: 生成されたコードに期待される文字列が含まれることを検証
fn assert_contains(output: &str, expected: &str) {
    assert!(
        output.contains(expected),
        "Expected output to contain:\n{}\n\nBut got:\n{}",
        expected,
        output
    );
}

/// テストヘルパー: 生成されたコードに期待される文字列が含まれないことを検証
fn assert_not_contains(output: &str, unexpected: &str) {
    assert!(
        !output.contains(unexpected),
        "Expected output NOT to contain:\n{}\n\nBut got:\n{}",
        unexpected,
        output
    );
}

#[test]
fn test_simple_struct_with_primitives() {
    let ty = GearMeshType {
        name: "Point".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "x".to_string(),
                    ty: TypeRef::new("f64"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "y".to_string(),
                    ty: TypeRef::new("f64"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "export interface Point {");
    assert_contains(&output, "x: number;");
    assert_contains(&output, "y: number;");
    assert_contains(&output, "}");
}

#[test]
fn test_struct_with_string_types() {
    let ty = GearMeshType {
        name: "Person".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "email".to_string(),
                    ty: TypeRef::new("String"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "export interface Person {");
    assert_contains(&output, "name: string;");
    assert_contains(&output, "email: string;");
}

#[test]
fn test_struct_with_boolean() {
    let ty = GearMeshType {
        name: "Config".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "enabled".to_string(),
                    ty: TypeRef::new("bool"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "debug".to_string(),
                    ty: TypeRef::new("bool"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "export interface Config {");
    assert_contains(&output, "enabled: boolean;");
    assert_contains(&output, "debug: boolean;");
}

#[test]
fn test_struct_with_integer_types() {
    let ty = GearMeshType {
        name: "Stats".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "count_i8".to_string(),
                    ty: TypeRef::new("i8"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_i16".to_string(),
                    ty: TypeRef::new("i16"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_i32".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_u8".to_string(),
                    ty: TypeRef::new("u8"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_u16".to_string(),
                    ty: TypeRef::new("u16"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_u32".to_string(),
                    ty: TypeRef::new("u32"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // すべてnumberになる
    assert_contains(&output, "count_i8: number;");
    assert_contains(&output, "count_i16: number;");
    assert_contains(&output, "count_i32: number;");
    assert_contains(&output, "count_u8: number;");
    assert_contains(&output, "count_u16: number;");
    assert_contains(&output, "count_u32: number;");
}

#[test]
fn test_struct_with_bigint_types() {
    let ty = GearMeshType {
        name: "BigNumbers".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "count_i64".to_string(),
                    ty: TypeRef::new("i64"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_u64".to_string(),
                    ty: TypeRef::new("u64"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_i128".to_string(),
                    ty: TypeRef::new("i128"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "count_u128".to_string(),
                    ty: TypeRef::new("u128"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    // BigInt有効
    let config = GeneratorConfig::new().with_bigint(true).with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(std::slice::from_ref(&ty));

    assert_contains(&output, "count_i64: bigint;");
    assert_contains(&output, "count_u64: bigint;");
    assert_contains(&output, "count_i128: bigint;");
    assert_contains(&output, "count_u128: bigint;");

    // BigInt無効（numberになる）
    let config = GeneratorConfig::new().with_bigint(false).with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "count_i64: number;");
    assert_contains(&output, "count_u64: number;");
    assert_contains(&output, "count_i128: number;");
    assert_contains(&output, "count_u128: number;");
}

#[test]
fn test_branded_type_generation() {
    let ty = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    let config = GeneratorConfig::new().with_branded(true).with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // Branded Typeヘルパーが生成される
    assert_contains(&output, "type Brand<T, B> = T & { readonly __brand: B };");

    // Branded Type定義
    assert_contains(&output, "export type UserId = Brand<number, \"UserId\">;");

    // ヘルパー関数
    assert_contains(
        &output,
        "export const UserId = (value: number): UserId => value as UserId;",
    );
}

#[test]
fn test_multiple_branded_types() {
    let user_id = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    let product_id = GearMeshType {
        name: "ProductId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    let config = GeneratorConfig::new().with_branded(true).with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_id, product_id]);

    // Branded Typeヘルパーは1回だけ生成される
    let brand_count = output.matches("type Brand<T, B>").count();
    assert_eq!(brand_count, 1, "Brand helper should appear only once");

    // 両方の型が生成される
    assert_contains(&output, "export type UserId = Brand<number, \"UserId\">;");
    assert_contains(
        &output,
        "export type ProductId = Brand<number, \"ProductId\">;",
    );
}

#[test]
fn test_optional_fields() {
    let ty = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "email".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("String")]),
                    docs: None,
                    validations: vec![],
                    optional: true,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "age".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("u8")]),
                    docs: None,
                    validations: vec![],
                    optional: true,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "id: number;");
    assert_contains(&output, "email?: string | null;");
    assert_contains(&output, "age?: number | null;");
}

#[test]
fn test_vec_array_generation() {
    let ty = GearMeshType {
        name: "UserList".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "users".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("User")]),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "tags".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("String")]),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "users: User[];");
    assert_contains(&output, "tags: string[];");
}

#[test]
fn test_hashmap_generation() {
    let ty = GearMeshType {
        name: "UserMap".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "users_by_id".to_string(),
                    ty: TypeRef::with_generics(
                        "HashMap",
                        vec![TypeRef::new("String"), TypeRef::new("User")],
                    ),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "scores".to_string(),
                    ty: TypeRef::with_generics(
                        "HashMap",
                        vec![TypeRef::new("String"), TypeRef::new("i32")],
                    ),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // StringキーのHashMapはRecordになる
    assert_contains(&output, "users_by_id: Record<string, User>;");
    assert_contains(&output, "scores: Record<string, number>;");
}

#[test]
fn test_simple_enum_generation() {
    let ty = GearMeshType {
        name: "Status".to_string(),
        kind: TypeKind::Enum(EnumType {
            variants: vec![
                EnumVariant {
                    name: "Active".to_string(),
                    content: VariantContent::Unit,
                    docs: None,
                },
                EnumVariant {
                    name: "Inactive".to_string(),
                    content: VariantContent::Unit,
                    docs: None,
                },
                EnumVariant {
                    name: "Pending".to_string(),
                    content: VariantContent::Unit,
                    docs: None,
                },
            ],
            representation: EnumRepresentation::External,
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    assert_contains(&output, "export type Status =");
    assert_contains(&output, "\"Active\"");
    assert_contains(&output, "\"Inactive\"");
    assert_contains(&output, "\"Pending\"");
}

#[test]
fn test_jsdoc_generation() {
    let ty = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: Some(DocComment::summary("User's unique identifier")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("User's display name")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: Some(DocComment::summary("User information")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(true);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // 型のJSDoc
    assert_contains(&output, "/**");
    assert_contains(&output, " * User information");
    assert_contains(&output, " */");

    // フィールドのJSDoc
    assert_contains(&output, "/** User's unique identifier */");
    assert_contains(&output, "/** User's display name */");
}

#[test]
fn test_jsdoc_disabled() {
    let ty = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "id".to_string(),
                ty: TypeRef::new("i32"),
                docs: Some(DocComment::summary("User ID")),
                validations: vec![],
                optional: false,
                serde_attrs: SerdeFieldAttrs::default(),
            }],
        }),
        docs: Some(DocComment::summary("User information")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // JSDocが生成されない
    assert_not_contains(&output, "/**");
    assert_not_contains(&output, "User information");
    assert_not_contains(&output, "User ID");
}

#[test]
fn test_nested_types() {
    let ty = GearMeshType {
        name: "Response".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "data".to_string(),
                    ty: TypeRef::with_generics(
                        "Option",
                        vec![TypeRef::with_generics("Vec", vec![TypeRef::new("User")])],
                    ),
                    docs: None,
                    validations: vec![],
                    optional: true,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "metadata".to_string(),
                    ty: TypeRef::with_generics(
                        "HashMap",
                        vec![TypeRef::new("String"), TypeRef::new("String")],
                    ),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[ty]);

    // Option<Vec<User>> → User[] | null
    assert_contains(&output, "data?: User[] | null;");
    // HashMap<String, String> → Record<string, string>
    assert_contains(&output, "metadata: Record<string, string>;");
}

#[test]
fn test_complex_real_world_example() {
    // UserId (Branded)
    let user_id = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: Some(DocComment::summary("Unique user identifier")),
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    // User struct
    let user = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("UserId"),
                    docs: Some(DocComment::summary("User ID")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("Full name")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "email".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("Email address")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "age".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("u8")]),
                    docs: Some(DocComment::summary("Age (optional)")),
                    validations: vec![],
                    optional: true,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "tags".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("String")]),
                    docs: Some(DocComment::summary("User tags")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: Some(DocComment::summary("User information")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new()
        .with_branded(true)
        .with_jsdoc(true)
        .with_bigint(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_id, user]);

    // Branded Type
    assert_contains(&output, "type Brand<T, B>");
    assert_contains(&output, "export type UserId = Brand<number, \"UserId\">;");
    assert_contains(
        &output,
        "export const UserId = (value: number): UserId => value as UserId;",
    );

    // User interface
    assert_contains(&output, "export interface User {");
    assert_contains(&output, "id: UserId;");
    assert_contains(&output, "name: string;");
    assert_contains(&output, "email: string;");
    assert_contains(&output, "age?: number | null;");
    assert_contains(&output, "tags: string[];");

    // JSDoc (複数行形式で生成される)
    assert_contains(&output, "/**");
    assert_contains(&output, " * Unique user identifier");
    assert_contains(&output, " */");
    assert_contains(&output, " * User information");
    assert_contains(&output, "/** User ID */");
    assert_contains(&output, "/** Full name */");
    assert_contains(&output, "/** Email address */");
    assert_contains(&output, "/** Age (optional) */");
    assert_contains(&output, "/** User tags */");
}
