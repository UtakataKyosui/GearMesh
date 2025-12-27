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

    // Generate TypeScript with Zod schemas
    let config = GeneratorConfig::new().with_zod(true).with_validation(true);
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

/// Generate TypeScript definitions to separate files
///
/// Each type will be generated in its own file, with an index.ts that re-exports all types.
///
/// # Example
///
/// ```no_run
/// use gear_mesh::generate_types_to_dir;
///
/// generate_types_to_dir("../frontend/src/types")
///     .expect("Failed to generate types");
/// ```
pub fn generate_types_to_dir(output_dir: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    use crate::{GeneratorConfig, TypeScriptGenerator};
    use std::fs;

    let output_dir = output_dir.as_ref();

    // Collect all registered types
    let types: Vec<_> = inventory::iter::<TypeInfo>()
        .map(|info| (info.get_type)())
        .collect();

    if types.is_empty() {
        eprintln!(
            "⚠️  Warning: No types found. Make sure you have #[derive(GearMesh)] on your types."
        );
        return Ok(());
    }

    // Create output directory
    fs::create_dir_all(output_dir)?;

    // Generate config
    let config = GeneratorConfig::new().with_zod(true).with_validation(true);

    let validator = crate::ValidationGenerator::new();
    let mut exports = Vec::new();

    // Check if we need Branded Type helper
    let has_branded = types.iter().any(|t| t.attributes.branded);

    for ty in &types {
        let file_name = format!("{}.ts", ty.name);
        let file_path = output_dir.join(&file_name);

        let mut content = String::new();

        // Zod import
        if config.generate_zod {
            content.push_str("import { z } from 'zod';\n");
        }

        // Extract type dependencies and generate imports
        let deps = crate::extract_type_dependencies(ty);
        let mut sorted_deps: Vec<_> = deps.iter().collect();
        sorted_deps.sort();

        for dep in sorted_deps {
            // Skip self-reference
            if dep != &ty.name {
                content.push_str(&format!("import type {{ {} }} from './{}';\n", dep, dep));
            }
        }

        if !deps.is_empty() {
            content.push('\n');
        }

        // Branded Type helper if this type needs it
        if config.generate_branded && ty.attributes.branded {
            content.push_str("\n// Branded Type helper\n");
            content.push_str("type Brand<T, B> = T & { readonly __brand: B };\n");
        }

        content.push('\n');

        // Generate the type
        let mut generator = TypeScriptGenerator::new(config.clone());
        generator.generate_type(ty);
        content.push_str(&generator.output);

        // Generate Zod schema
        if config.generate_zod {
            if let Some(schema) = validator.generate_zod_schema(ty) {
                content.push_str("\n// Zod Schema\n\n");
                content.push_str(&schema);
            }
        }

        fs::write(&file_path, content)?;
        exports.push(ty.name.clone());
        println!("  ✓ {}", file_name);
    }

    // Generate index.ts
    let mut index_content = String::new();
    index_content.push_str("// Auto-generated index file\n");
    index_content.push_str("// Re-exports all types\n\n");

    // Add Branded Type helper to index if needed
    if has_branded {
        index_content.push_str("// Branded Type helper\n");
        index_content.push_str("type Brand<T, B> = T & { readonly __brand: B };\n\n");
    }

    for type_name in &exports {
        index_content.push_str(&format!("export * from './{}';\n", type_name));
    }

    fs::write(output_dir.join("index.ts"), index_content)?;

    println!("✅ Generated TypeScript types to: {}", output_dir.display());
    println!("   {} types exported", types.len());
    println!("   📄 index.ts created");

    Ok(())
}
