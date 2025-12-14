//! TypeScriptコード生成の追加テスト

use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType,
    NewtypeType, StructType, TypeAttributes, TypeKind, TypeRef, VariantContent,
};

use crate::{GeneratorConfig, TypeScriptGenerator};

#[test]
fn test_generate_enum_with_data() {
    let result_type = GearMeshType {
        name: "Result".to_string(),
        kind: TypeKind::Enum(EnumType {
            variants: vec![
                EnumVariant {
                    name: "Ok".to_string(),
                    content: VariantContent::Tuple(vec![TypeRef::new("String")]),
                    docs: None,
                },
                EnumVariant {
                    name: "Err".to_string(),
                    content: VariantContent::Tuple(vec![TypeRef::new("String")]),
                    docs: None,
                },
            ],
            representation: EnumRepresentation::External,
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[result_type]);

    assert!(output.contains("export type Result"));
    assert!(output.contains("Ok"));
    assert!(output.contains("Err"));
}

#[test]
fn test_generate_nested_types() {
    let inner_type = GearMeshType {
        name: "Inner".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "value".to_string(),
                ty: TypeRef::new("i32"),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: Default::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let outer_type = GearMeshType {
        name: "Outer".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "inner".to_string(),
                ty: TypeRef::new("Inner"),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: Default::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[inner_type, outer_type]);

    assert!(output.contains("export interface Inner"));
    assert!(output.contains("export interface Outer"));
    assert!(output.contains("inner: Inner;"));
}

#[test]
fn test_generate_with_jsdoc_disabled() {
    let user_type = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "id".to_string(),
                ty: TypeRef::new("i32"),
                docs: Some(DocComment::summary("User ID")),
                validations: vec![],
                optional: false,
                serde_attrs: Default::default(),
            }],
        }),
        docs: Some(DocComment::summary("User information")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let config = GeneratorConfig::new().with_jsdoc(false);
    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_type]);

    // JSDocコメントが含まれないことを確認
    assert!(!output.contains("/**"));
    assert!(output.contains("export interface User"));
}

#[test]
fn test_tuple_type_generation() {
    let tuple_type = GearMeshType {
        name: "Pair".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "data".to_string(),
                ty: TypeRef::with_generics(
                    "__tuple__",
                    vec![TypeRef::new("i32"), TypeRef::new("String")],
                ),
                docs: None,
                validations: vec![],
                optional: false,
                serde_attrs: Default::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[tuple_type]);

    assert!(output.contains("[number, string]"));
}

#[test]
fn test_option_type_generation() {
    let optional_type = GearMeshType {
        name: "OptionalData".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "value".to_string(),
                ty: TypeRef::with_generics("Option", vec![TypeRef::new("String")]),
                docs: None,
                validations: vec![],
                optional: true,
                serde_attrs: Default::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[optional_type]);

    assert!(output.contains("value?: string | null;"));
}

#[test]
fn test_empty_struct_generation() {
    let empty_type = GearMeshType {
        name: "Empty".to_string(),
        kind: TypeKind::Struct(StructType { fields: vec![] }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[empty_type]);

    assert!(output.contains("export interface Empty"));
}

#[test]
fn test_branded_type_without_flag() {
    let newtype = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes {
            branded: false,
            ..Default::default()
        },
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_branded(false));
    let output = generator.generate(&[newtype]);

    // Branded Typeではなく、通常のtype aliasとして生成される
    assert!(output.contains("export type UserId = number;"));
    assert!(!output.contains("Brand<"));
}
