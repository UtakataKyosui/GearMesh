//! Real derive expansion used by tests/e2e's TypeScript consumer regression test.
#![allow(dead_code)]

use gear_mesh::{GearMesh, GearMeshExport, GeneratorConfig, TypeScriptGenerator};

#[derive(GearMesh)]
#[gear_mesh(branded)]
struct UserId(i32);

#[derive(GearMesh)]
#[gear_mesh(branded)]
struct ProductId(i32);

#[derive(GearMesh)]
enum Status {
    Pending,
    Complete,
}

/// A record shared with a TypeScript consumer.
#[derive(GearMesh)]
#[serde(rename_all = "camelCase")]
struct User {
    id: UserId,
    display_name: String,
    /// Missing values are represented by null, not an absent key.
    nickname: Option<String>,
    tags: Vec<String>,
    balance: u64,
    active: bool,
    status: Status,
}

fn main() -> std::io::Result<()> {
    let output = std::env::args_os().nth(1).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "expected output file path",
        )
    })?;
    // Explicit order keeps the fixture stable across inventory/linker ordering.
    let types = [
        UserId::gear_mesh_type(),
        ProductId::gear_mesh_type(),
        Status::gear_mesh_type(),
        User::gear_mesh_type(),
    ];
    let generated = TypeScriptGenerator::new(GeneratorConfig::new()).generate(&types);
    std::fs::write(output, generated)
}
