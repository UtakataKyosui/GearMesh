use gear_mesh_generator::{
    FieldInfo, GearMeshType, GeneratorConfig, StructType, TypeAttributes, TypeKind, TypeRef,
    TypeScriptGenerator,
};
use proptest::prelude::*;

fn arb_identifier() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[A-Z][A-Za-z0-9]{0,7}").unwrap()
}

fn arb_field_name() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-z][a-z0-9_]{0,7}").unwrap()
}

fn arb_type_ref() -> impl Strategy<Value = TypeRef> {
    prop_oneof![
        Just(TypeRef::new("String")),
        Just(TypeRef::new("bool")),
        Just(TypeRef::new("i32")),
        Just(TypeRef::new("u64")),
        Just(TypeRef::with_generics("Vec", vec![TypeRef::new("String")])),
        Just(TypeRef::with_generics(
            "Option",
            vec![TypeRef::new("String")]
        )),
    ]
}

fn arb_gear_mesh_type() -> impl Strategy<Value = GearMeshType> {
    (
        arb_identifier(),
        prop::collection::vec((arb_field_name(), arb_type_ref()), 1..4),
    )
        .prop_map(|(name, fields)| GearMeshType {
            name,
            kind: TypeKind::Struct(StructType {
                fields: fields
                    .into_iter()
                    .map(|(field_name, ty)| FieldInfo {
                        name: field_name,
                        optional: ty.name == "Option",
                        ty,
                        docs: None,
                        validations: vec![],
                        serde_attrs: Default::default(),
                    })
                    .collect(),
            }),
            docs: None,
            generics: vec![],
            attributes: TypeAttributes::default(),
        })
}

fn has_balanced_pairs(source: &str, open: char, close: char) -> bool {
    let mut depth = 0isize;

    for ch in source.chars() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
        }

        if depth < 0 {
            return false;
        }
    }

    depth == 0
}

proptest! {
    #[test]
    fn generated_typescript_keeps_basic_syntax_invariants(ty in arb_gear_mesh_type()) {
        let mut generator = TypeScriptGenerator::new(
            GeneratorConfig::new().with_zod(true)
        );
        let output = generator.generate(&[ty]);

        prop_assert_eq!(output.contains("export interface"), true);
        prop_assert_eq!(has_balanced_pairs(&output, '{', '}'), true);
        prop_assert_eq!(has_balanced_pairs(&output, '(', ')'), true);
    }

    #[test]
    fn gear_mesh_type_roundtrips_through_json(ty in arb_gear_mesh_type()) {
        let json = serde_json::to_string(&ty).unwrap();
        let deserialized: GearMeshType = serde_json::from_str(&json).unwrap();
        let reparsed = serde_json::to_value(deserialized).unwrap();
        let original = serde_json::to_value(ty).unwrap();

        prop_assert_eq!(original, reparsed);
    }
}
