/// Type information for automatic collection via inventory
pub struct TypeInfo {
    pub get_type: fn() -> crate::GearMeshType,
    pub type_name: &'static str,
}

inventory::collect!(TypeInfo);

/// Generate TypeScript definitions for all registered types
///
/// This function automatically collects all types that have `#[derive(GearMesh)]`
/// and generates TypeScript definitions.
///
/// # Example
///
/// ```no_run
/// use gear_mesh::generate_types;
///
/// generate_types("../frontend/src/types/generated.ts")
///     .expect("Failed to generate types");
/// ```
pub fn generate_types(output_path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    use crate::{GeneratorConfig, TypeScriptGenerator};
    use std::fs;

    // Collect all registered types
    let types: Vec<_> = inventory::iter::<TypeInfo>()
        .map(|info| (info.get_type)())
        .collect();

    if types.is_empty() {
        eprintln!(
            "⚠️  Warning: No types found. Make sure you have #[derive(GearMesh)] on your types."
        );
    }

    // Generate TypeScript
    let config = GeneratorConfig::new();
    let mut generator = TypeScriptGenerator::new(config);
    let output_content = generator.generate(&types);

    // Create output directory
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write to file
    fs::write(output_path, output_content)?;

    println!("✅ Generated TypeScript types: {}", output_path.display());
    println!("   {} types exported", types.len());

    Ok(())
}
