//! Plugin support for custom Rust to TypeScript transformations.

use crate::TypeRef;

/// A pluggable transformer for custom Rust types.
pub trait TypeTransformer: Send + Sync {
    /// Returns true when this transformer can handle the provided type name.
    fn can_handle(&self, type_name: &str) -> bool;

    /// Returns a TypeScript type override for the provided type reference.
    fn transform_type(&self, type_ref: &TypeRef) -> Option<String>;

    /// Returns a Zod schema override for the provided type reference.
    fn transform_zod(&self, type_ref: &TypeRef) -> Option<String>;

    /// Raw import statements required by this transformer.
    fn required_imports(&self) -> Vec<String> {
        Vec::new()
    }
}
