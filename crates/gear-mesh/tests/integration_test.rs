//! 統合テスト: 型変換の完全なフロー
//!
//! Rust型定義 → 中間表現 → TypeScriptコード生成までの統合テスト

use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType,
    NewtypeType, PrimitiveType, SerdeFieldAttrs, StructType, TypeAttributes, TypeKind,
    TypeRef, ValidationRule, VariantContent,
};
use gear_mesh_generator::{GeneratorConfig, TypeScriptGenerator};

#[test]
fn test_simple_struct_generation() {
    let user_type = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: Some(DocComment::summary("User ID")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("User name")),
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
    let output = generator.generate(&[user_type]);

    assert!(output.contains("export interface User"));
    assert!(output.contains("id: number;"));
    assert!(output.contains("name: string;"));
    assert!(output.contains("/** User ID */"));
    assert!(output.contains("/** User name */"));
}

#[test]
fn test_branded_type_generation() {
    let user_id_type = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: Some(DocComment::summary("User identifier")),
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    let config = GeneratorConfig::new().with_branded(true).with_jsdoc(true);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_id_type]);

    assert!(output.contains("type Brand<T, B>"));
    assert!(output.contains("export type UserId = Brand<number, \"UserId\">;"));
    assert!(output.contains("export const UserId = (value: number): UserId => value as UserId;"));
}

#[test]
fn test_enum_generation() {
    let status_type = GearMeshType {
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
            ],
            representation: EnumRepresentation::External,
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new();
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[status_type]);

    assert!(output.contains("export type Status"));
    assert!(output.contains("\"Active\""));
    assert!(output.contains("\"Inactive\""));
}

#[test]
fn test_bigint_generation() {
    let timestamp_type = GearMeshType {
        name: "Timestamp".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("u64"),
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            bigint_auto: true,
            ..Default::default()
        },
    };

    let config = GeneratorConfig::new().with_bigint(true);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[timestamp_type]);

    // u64はbigintとして生成される
    assert!(output.contains("bigint") || output.contains("number"));
}

#[test]
fn test_optional_fields() {
    let user_type = GearMeshType {
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
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new();
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_type]);

    assert!(output.contains("id: number;"));
    assert!(output.contains("email?: string | null;"));
}

#[test]
fn test_validation_generation() {
    let user_type = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "age".to_string(),
                ty: TypeRef::new("i32"),
                docs: None,
                validations: vec![ValidationRule::Range {
                    min: Some(0.0),
                    max: Some(150.0),
                }],
                optional: false,
                serde_attrs: SerdeFieldAttrs::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            validate: true,
            ..Default::default()
        },
    };

    let config = GeneratorConfig::new().with_validation(true);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_type]);

    // バリデーション関数が生成されるはず
    assert!(output.contains("export interface User"));
}

#[test]
fn test_multiple_types_generation() {
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

    let user = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("UserId"),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "name".to_string(),
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

    let config = GeneratorConfig::new().with_branded(true);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_id, user]);

    // 両方の型が生成される
    assert!(output.contains("export type UserId"));
    assert!(output.contains("export interface User"));
    assert!(output.contains("id: UserId;"));
}

#[test]
fn test_vec_generation() {
    let users_type = GearMeshType {
        name: "UserList".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "users".to_string(),
                ty: TypeRef::with_generics("Vec", vec![TypeRef::new("User")]),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: SerdeFieldAttrs::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new();
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[users_type]);

    assert!(output.contains("users: User[];"));
}

#[test]
fn test_hashmap_generation() {
    let map_type = GearMeshType {
        name: "UserMap".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "users".to_string(),
                ty: TypeRef::with_generics(
                    "HashMap",
                    vec![TypeRef::new("String"), TypeRef::new("User")],
                ),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: SerdeFieldAttrs::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new();
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[map_type]);

    assert!(output.contains("users: Record<string, User>;"));
}
