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
    generate_with_config(
        output_path,
        crate::GeneratorConfig::new()
            .with_zod(true)
            .with_validation(true),
    )
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
    generate_types_to_dir_with_config(
        output_dir,
        crate::GeneratorConfig::new()
            .with_zod(true)
            .with_validation(true)
            .with_module_strategy(crate::ModuleStrategy::PerType),
    )
}

pub fn generate_with_config(
    output_path: impl AsRef<std::path::Path>,
    config: crate::GeneratorConfig,
) -> std::io::Result<()> {
    use std::fs;

    let types = collect_registered_types();
    if types.is_empty() {
        eprintln!(
            "⚠️  Warning: No types found. Make sure you have #[derive(GearMesh)] on your types."
        );
    }

    let mut generator = crate::TypeScriptGenerator::new(config.clone());
    let output = generator.generate(&types);
    let output_path = output_path.as_ref();

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let cache_path = crate::cache::cache_file(&config.cache_dir);
    let mut cache = if config.enable_cache {
        crate::cache::OutputCache::load(&cache_path)
    } else {
        crate::cache::OutputCache::default()
    };
    write_output(output_path, &output, config.enable_cache, &mut cache)?;
    if config.enable_cache {
        cache.persist(&cache_path)?;
    }

    println!("✅ Generated TypeScript types: {}", output_path.display());
    println!("   {} types exported", types.len());
    Ok(())
}

pub fn generate_types_to_dir_with_config(
    output_dir: impl AsRef<std::path::Path>,
    config: crate::GeneratorConfig,
) -> std::io::Result<()> {
    use std::fs;

    let output_dir = output_dir.as_ref();
    let types = collect_registered_types();
    if types.is_empty() {
        eprintln!(
            "⚠️  Warning: No types found. Make sure you have #[derive(GearMesh)] on your types."
        );
        return Ok(());
    }

    fs::create_dir_all(output_dir)?;

    let organizer = crate::ModuleOrganizer::new(&types);
    let modules = organizer.organize(&types, &config.module_strategy);
    let type_index = organizer.build_type_index(&modules);
    let cache_path = crate::cache::cache_file(&config.cache_dir);
    let mut cache = if config.enable_cache {
        crate::cache::OutputCache::load(&cache_path)
    } else {
        crate::cache::OutputCache::default()
    };

    for (relative_path, module_types) in &modules {
        let file_path = output_dir.join(relative_path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let imports = organizer.generate_imports(
            relative_path,
            module_types,
            &type_index,
            config.generate_zod,
        );
        let mut generator = crate::TypeScriptGenerator::new(config.clone());
        let content = generator.generate_with_imports(module_types, &imports);
        write_output(&file_path, &content, config.enable_cache, &mut cache)?;
        println!("  ✓ {}", relative_path);
    }

    if !matches!(config.module_strategy, crate::ModuleStrategy::SingleFile) {
        let mut index_content = String::new();
        index_content.push_str("// Auto-generated index file\n");
        index_content.push_str("// Re-exports all generated modules\n\n");

        for relative_path in modules.keys() {
            let export_path = relative_path.strip_suffix(".ts").unwrap_or(relative_path);
            index_content.push_str(&format!("export * from './{}';\n", export_path));
        }

        write_output(
            &output_dir.join("index.ts"),
            &index_content,
            config.enable_cache,
            &mut cache,
        )?;
        println!("   📄 index.ts created");
    }

    if config.enable_cache {
        cache.persist(&cache_path)?;
    }

    println!("✅ Generated TypeScript types to: {}", output_dir.display());
    println!("   {} types exported", types.len());
    Ok(())
}

fn collect_registered_types() -> Vec<crate::GearMeshType> {
    inventory::iter::<TypeInfo>()
        .map(|info| (info.get_type)())
        .collect()
}

fn write_output(
    path: &std::path::Path,
    content: &str,
    use_cache: bool,
    cache: &mut crate::cache::OutputCache,
) -> std::io::Result<()> {
    use std::fs;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !use_cache || cache.has_changed(path, content) || !path.exists() {
        fs::write(path, content)?;
        cache.update(path, content);
    }

    Ok(())
}
