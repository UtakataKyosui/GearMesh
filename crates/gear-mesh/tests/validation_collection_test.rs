//! Test validation rule collection

use gear_mesh::GearMesh;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, GearMesh)]
struct TestUser {
    #[validate(length(min = 1, max = 20))]
    name: String,
    #[validate(range(min = 1, max = 100))]
    age: Option<i32>,
}

#[test]
fn test_validation_collection() {
    use gear_mesh::GearMeshExport;

    let ty = TestUser::gear_mesh_type();

    if let gear_mesh::TypeKind::Struct(s) = &ty.kind {
        // Check name field
        let name_field = s.fields.iter().find(|f| f.name == "name").unwrap();
        assert_eq!(name_field.validations.len(), 1);

        // Check age field
        let age_field = s.fields.iter().find(|f| f.name == "age").unwrap();
        println!("Age field validations: {:?}", age_field.validations);
        assert_eq!(
            age_field.validations.len(),
            1,
            "Age field should have 1 validation rule"
        );
    } else {
        panic!("Expected struct type");
    }
}
