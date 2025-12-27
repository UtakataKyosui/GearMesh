/// Export types macro for compile-time TypeScript generation
///
/// # Example
///
/// ```ignore
/// use gear_mesh::export_types;
///
/// export_types! {
///     output = "../frontend/src/types/generated.ts",
///     types = [User, UserId, CreateUserRequest]
/// }
/// ```
#[macro_export]
macro_rules! export_types {
    (output = $output:expr, types = [$($ty:ty),* $(,)?]) => {
        {
            use std::fs;
            use std::path::Path;
            use $crate::{GearMeshExport, TypeScriptGenerator, GeneratorConfig};

            // Collect all types
            let mut types = Vec::new();
            $(
                types.push(<$ty as GearMeshExport>::gear_mesh_type());
            )*

            // Generate TypeScript
            let config = GeneratorConfig::new();
            let mut generator = TypeScriptGenerator::new(config);
            let output_content = generator.generate(&types);

            // Create output directory
            let output_path = Path::new($output);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).ok();
            }

            // Write to file
            fs::write(output_path, output_content)
                .expect(&format!("Failed to write TypeScript definitions to {}", $output));

            println!("✅ Generated TypeScript types: {}", $output);
        }
    };
}
