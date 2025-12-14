//! gear-mesh: Next-generation Rust to TypeScript type definition sharing
//!
//! This crate provides tools for sharing type definitions between Rust and TypeScript,
//! with features like Branded Types, validation, and documentation preservation.
//!
//! # Features
//!
//! - **Branded Type Generation**: Convert Rust newtype patterns to TypeScript Branded Types
//! - **Documentation Conversion**: Rust doc comments → JSDoc
//! - **Validation Embedding**: Generate runtime validation functions
//! - **BigInt Auto-conversion**: Automatically use BigInt for u64/i64
//! - **Watch Mode**: Real-time regeneration on file changes
//!
//! # Example
//!
//! ```ignore
//! use gear_mesh::GearMesh;
//!
//! #[derive(GearMesh)]
//! #[gear_mesh(branded)]
//! struct UserId(i32);
//!
//! #[derive(GearMesh)]
//! struct User {
//!     id: UserId,
//!     name: String,
//! }
//! ```
//!
//! Generated TypeScript:
//!
//! ```typescript
//! type Brand<T, B> = T & { readonly __brand: B };
//!
//! export type UserId = Brand<number, "UserId">;
//! export const UserId = (value: number): UserId => value as UserId;
//!
//! export interface User {
//!     id: UserId;
//!     name: string;
//! }
//! ```

// Re-export core types
pub use gear_mesh_core::*;

// Re-export derive macro
pub use gear_mesh_derive::GearMesh;

// Re-export generator
pub use gear_mesh_generator::{GeneratorConfig, TypeScriptGenerator};

/// Trait implemented by types that can be exported to TypeScript
pub trait GearMeshExport {
    /// Get the GearMesh type representation
    fn gear_mesh_type() -> GearMeshType;

    /// Get the type name
    fn type_name() -> &'static str;
}

/// Configuration for gear-mesh
#[derive(Debug, Clone)]
pub struct Config {
    /// Output directory for TypeScript files
    pub output_dir: String,
    /// Generator configuration
    pub generator: GeneratorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            output_dir: "bindings".to_string(),
            generator: GeneratorConfig::new(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output_dir(mut self, dir: impl Into<String>) -> Self {
        self.output_dir = dir.into();
        self
    }

    pub fn with_bigint(mut self, use_bigint: bool) -> Self {
        self.generator = self.generator.with_bigint(use_bigint);
        self
    }

    pub fn with_branded(mut self, generate: bool) -> Self {
        self.generator = self.generator.with_branded(generate);
        self
    }

    pub fn with_validation(mut self, generate: bool) -> Self {
        self.generator = self.generator.with_validation(generate);
        self
    }
}

/// Generate TypeScript definitions for the given types
pub fn generate<T: GearMeshExport>() -> String {
    let ty = T::gear_mesh_type();
    let mut generator = TypeScriptGenerator::new(GeneratorConfig::new());
    generator.generate(&[ty])
}

/// Generate TypeScript definitions for multiple types
pub fn generate_all(types: &[GearMeshType], config: GeneratorConfig) -> String {
    let mut generator = TypeScriptGenerator::new(config);
    generator.generate(types)
}
