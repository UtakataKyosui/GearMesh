use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

/// Global registry of output paths that have been processed
static OUTPUT_REGISTRY: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Register an output path and generate types if this is the first time
pub fn register_output(output_path: &str) {
    let mut registry = OUTPUT_REGISTRY.lock().unwrap();

    // Only generate if this is the first type with this output path
    if registry.insert(output_path.to_string()) {
        // Generate types for this output path
        if let Err(e) = crate::generate_types(output_path) {
            eprintln!("⚠️  Failed to generate types to {}: {}", output_path, e);
        }
    }
}
