//! gear-mesh example
//!
//! Run with: cargo run --example basic

use gear_mesh_core::{
    DocComment, FieldInfo, GearMeshType, NewtypeType, SerdeFieldAttrs, StructType, TypeAttributes,
    TypeKind, TypeRef,
};
use gear_mesh_generator::{GeneratorConfig, TypeScriptGenerator};

fn main() {
    // Create a UserId branded type
    let user_id_type = GearMeshType {
        name: "UserId".to_string(),
        kind: TypeKind::Newtype(NewtypeType {
            inner: TypeRef::new("i32"),
        }),
        docs: Some(DocComment::summary("A unique user identifier")),
        generics: vec![],
        attributes: TypeAttributes {
            branded: true,
            ..Default::default()
        },
    };

    // Create a User struct
    let user_type = GearMeshType {
        name: "User".to_string(),
        kind: TypeKind::Struct(StructType {
            fields: vec![
                FieldInfo {
                    name: "id".to_string(),
                    ty: TypeRef::new("UserId"),
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
                FieldInfo {
                    name: "email".to_string(),
                    ty: TypeRef::new("String"),
                    docs: Some(DocComment::summary("User's email address")),
                    validations: vec![],
                    optional: false,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
                FieldInfo {
                    name: "age".to_string(),
                    ty: TypeRef::with_generics("Option", vec![TypeRef::new("u8")]),
                    docs: Some(DocComment::summary("User's age (optional)")),
                    validations: vec![],
                    optional: true,
                    serde_attrs: SerdeFieldAttrs::default(),
                },
            ],
        }),
        docs: Some(DocComment::summary("A user in the system")),
        generics: vec![],
        attributes: TypeAttributes::default(),
    };

    // Generate TypeScript
    let config = GeneratorConfig::new()
        .with_bigint(true)
        .with_branded(true)
        .with_jsdoc(true);

    let mut generator = TypeScriptGenerator::new(config);
    let output = generator.generate(&[user_id_type, user_type]);

    println!("Generated TypeScript:\n");
    println!("{}", output);
}
