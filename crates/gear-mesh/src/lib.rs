//! # gear-mesh
//!
//! Next-generation Rust to TypeScript type definition sharing library.
//!
//! ## Features
//!
//! - **Type Conversion**: Automatic Rust → TypeScript type conversion
//! - **Branded Types**: Newtype pattern → TypeScript Branded Types
//! - **Doc Comments**: Rust doc comments → JSDoc
//! - **BigInt Support**: Automatic i64/u64 → bigint conversion
//! - **Validation**: Runtime validation function generation
//! - **Serde Integration**: Full serde attribute support
//!
//! ## Quick Start
//!
//! ```rust
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
//!     email: Option<String>,
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
//!     email?: string | null;
//! }
//! ```
//!
//! ## CLI Usage
//!
//! Install the CLI tool:
//!
//! ```bash
//! cargo install gear-mesh-cli
//! ```
//!
//! Generate TypeScript definitions:
//!
//! ```bash
//! gear-mesh generate --input src --output bindings
//! gear-mesh watch --input src --output bindings
//! ```
//!
//! ## Configuration
//!
//! Create `gear-mesh.toml`:
//!
//! ```toml
//! [generator]
//! use_bigint = true
//! generate_branded = true
//! generate_jsdoc = true
//!
//! [paths]
//! input = "src"
//! output = "bindings"
//! ```

// Re-export everything from gear-mesh-generator (which includes the facade)
pub use gear_mesh_generator::*;

// Explicitly re-export commonly used items for discoverability
pub use gear_mesh_core::{DocComment, GearMeshType, TypeKind, ValidationRule};

pub use gear_mesh_derive::GearMesh;

// Export types macro for build-time type generation
#[macro_use]
mod export_macro;
