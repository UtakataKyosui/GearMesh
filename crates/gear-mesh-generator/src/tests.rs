//! TypeScriptコード生成の追加テスト

use std::{fs, path::PathBuf};

use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType, NewtypeType,
    StructType, TypeAttributes, TypeKind, TypeRef, ValidationRule, VariantContent,
};
use pretty_assertions::assert_eq;

use crate::{GeneratorConfig, OptionStyle, ResultStyle, TypeScriptGenerator};

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
fn test_option_type_generation_with_optional_style() {
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

    let mut generator =
        TypeScriptGenerator::new(GeneratorConfig::new().with_option_style(OptionStyle::Optional));
    let output = generator.generate(&[optional_type]);

    assert!(output.contains("value?: string | undefined;"));
}

#[test]
fn test_result_type_generation_with_tagged_union_style() {
    let result_holder = GearMeshType {
        name: "ApiResponse".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "result".to_string(),
                ty: TypeRef::with_generics(
                    "Result",
                    vec![TypeRef::new("String"), TypeRef::new("ApiError")],
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

    let mut generator = TypeScriptGenerator::new(
        GeneratorConfig::new().with_result_style(ResultStyle::TaggedUnion),
    );
    let output = generator.generate(&[result_holder]);

    assert!(output.contains("result: { ok: string } | { err: ApiError };"));
}

#[test]
fn test_result_type_generation_with_success_error_style() {
    let result_holder = GearMeshType {
        name: "ApiResponse".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "result".to_string(),
                ty: TypeRef::with_generics(
                    "Result",
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

    let mut generator = TypeScriptGenerator::new(
        GeneratorConfig::new().with_result_style(ResultStyle::SuccessError),
    );
    let output = generator.generate(&[result_holder]);

    assert!(
        output.contains(
            "result: { success: true; data: number } | { success: false; error: string };"
        )
    );
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

#[test]
fn test_zod_schema_generation_for_usize() {
    let ty = GearMeshType {
        name: "SizeInfo".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "size".to_string(),
                ty: TypeRef::new("usize"),
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

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_zod(true));
    let output = generator.generate(&[ty]);

    assert!(output.contains("export const SizeInfoSchema = z.object({"));
    // usize should be bigint by default
    assert!(
        output.contains("size: z.bigint()"),
        "Default usize should be z.bigint(), generated: {}",
        output
    );
}

#[test]
fn test_zod_schema_generation_no_bigint() {
    let ty = GearMeshType {
        name: "BigNum".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "val".to_string(),
                ty: TypeRef::new("i64"),
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

    // Explicitly disable bigint
    let mut generator =
        TypeScriptGenerator::new(GeneratorConfig::new().with_zod(true).with_bigint(false));
    let output = generator.generate(&[ty]);

    assert!(output.contains("export const BigNumSchema = z.object({"));
    // i64 should be number when use_bigint is false
    assert!(
        output.contains("val: z.number()"),
        "i64 should be z.number() when use_bigint=false, generated: {}",
        output
    );
}

#[test]
fn test_zod_schema_generation_with_validation_on_bigint() {
    let ty = GearMeshType {
        name: "ValidatedBigInt".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "val".to_string(),
                ty: TypeRef::new("u64"),
                docs: None,
                validations: vec![gear_mesh_core::ValidationRule::Range {
                    min: Some(10.0),
                    max: Some(100.0),
                }],
                optional: false,
                serde_attrs: Default::default(),
            }],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_zod(true));
    let output = generator.generate(&[ty]);

    assert!(output.contains("export const ValidatedBigIntSchema = z.object({"));
    // Expect z.bigint().min(10).max(100)
    assert!(
        output.contains("val: z.bigint().min(10n).max(100n)"),
        "Validation rules failed for bigint, generated: {}",
        output
    );
}

#[test]
fn test_zod_vec_generation() {
    let ty = GearMeshType {
        name: "StringList".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "tags".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("String")]),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "scores".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("i32")]),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_zod(true));
    let output = generator.generate(&[ty]);

    assert!(output.contains("export const StringListSchema = z.object({"));

    // Vec<String> -> z.array(z.string())
    assert!(
        output.contains("tags: z.array(z.string())"),
        "Vec<String> should be z.array(z.string()), generated: {}",
        output
    );

    // Vec<i32> -> z.array(z.number())
    assert!(
        output.contains("scores: z.array(z.number())"),
        "Vec<i32> should be z.array(z.number()), generated: {}",
        output
    );
}

#[test]
fn test_zod_option_generation_with_optional_style() {
    let ty = GearMeshType {
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

    let mut generator = TypeScriptGenerator::new(
        GeneratorConfig::new()
            .with_zod(true)
            .with_option_style(OptionStyle::Optional),
    );
    let output = generator.generate(&[ty]);

    assert!(output.contains("value: z.string().optional()"));
}

#[test]
fn test_zod_result_generation_with_tagged_union_style() {
    let ty = GearMeshType {
        name: "ApiResponse".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![FieldInfo {
                name: "result".to_string(),
                ty: TypeRef::with_generics(
                    "Result",
                    vec![TypeRef::new("String"), TypeRef::new("ApiError")],
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

    let mut generator = TypeScriptGenerator::new(
        GeneratorConfig::new()
            .with_zod(true)
            .with_result_style(ResultStyle::TaggedUnion),
    );
    let output = generator.generate(&[ty]);

    assert!(output.contains(
        "result: z.union([z.object({ ok: z.string() }), z.object({ err: ApiErrorSchema })])"
    ));
}

#[test]
fn test_snapshot_simple_struct_output() {
    let ty = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("i32"),
                    docs: Some(DocComment::summary("User ID")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("Display name")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
            ],
        }),
        docs: Some(DocComment::summary("User information")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[ty]);

    assert_snapshot("simple_struct.snap", &output);
}

#[test]
fn test_snapshot_enum_output() {
    let ty = GearMeshType {
        name: "ApiResult".to_string(),
        kind: TypeKind::Enum(EnumType {
            variants: vec![
                EnumVariant {
                    name: "Ok".to_string(),
                    content: VariantContent::Tuple(vec![TypeRef::new("String")]),
                    docs: None,
                },
                EnumVariant {
                    name: "Err".to_string(),
                    content: VariantContent::Tuple(vec![TypeRef::new("i32")]),
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
    let output = generator.generate(&[ty]);

    assert_snapshot("enum_with_data.snap", &output);
}

#[test]
fn test_snapshot_branded_type_output() {
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

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[ty]);

    assert_snapshot("branded_type.snap", &output);
}

#[test]
fn test_snapshot_generic_type_output() {
    let ty = GearMeshType {
        name: "Paginated".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "items".to_string(),
                    ty: TypeRef::with_generics("Vec", vec![TypeRef::new("T")]),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "next_cursor".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("String")]),
                    docs: None,
                    validations: vec![],
                    optional: true,
                    serde_attrs: Default::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![gear_mesh_core::GenericParam {
            name: "T".to_string(),
            bounds: vec![],
        }],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    let output = generator.generate(&[ty]);

    assert_snapshot("generic_type.snap", &output);
}

#[test]
fn test_snapshot_validation_and_zod_output() {
    let ty = GearMeshType {
        name: "UserInput".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "name".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("Display name")),
                    validations: vec![ValidationRule::Length {
                        min: Some(1),
                        max: Some(20),
                    }],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "email".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("Email address")),
                    validations: vec![ValidationRule::Email],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "website".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("String")]),
                    docs: Some(DocComment::summary("Optional website")),
                    validations: vec![ValidationRule::Url],
                    optional: true,
                    serde_attrs: Default::default(),
                },
            ],
        }),
        docs: Some(DocComment::summary("User input payload")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new().with_zod(true));
    let output = generator.generate(&[ty]);

    assert_snapshot("validation_and_zod.snap", &output);
}

#[test]
fn test_snapshot_result_tagged_union_output() {
    let ty = GearMeshType {
        name: "ApiResponse".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "result".to_string(),
                    ty: TypeRef::with_generics(
                        "Result",
                        vec![TypeRef::new("String"), TypeRef::new("ApiError")],
                    ),
                    docs: None,
                    validations: vec![],
                    optional: false,
                    serde_attrs: Default::default(),
                },
                FieldInfo {
                    name: "retry_after".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("i32")]),
                    docs: None,
                    validations: vec![],
                    optional: true,
                    serde_attrs: Default::default(),
                },
            ],
        }),
        docs: None,
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    let mut generator = TypeScriptGenerator::new(
        GeneratorConfig::new().with_result_style(ResultStyle::TaggedUnion),
    );
    let output = generator.generate(&[ty]);

    assert_snapshot("result_tagged_union.snap", &output);
}

fn assert_snapshot(name: &str, actual: &str) {
    let expected = fs::read_to_string(snapshot_path(name))
        .unwrap_or_else(|err| panic!("failed to read snapshot `{name}`: {err}"));
    assert_eq!(normalize_snapshot(&expected), normalize_snapshot(actual));
}

fn snapshot_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(name)
}

fn normalize_snapshot(input: &str) -> String {
    input.replace("\r\n", "\n")
}
