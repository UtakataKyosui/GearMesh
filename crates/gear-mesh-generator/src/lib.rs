//! gear-mesh-generator: TypeScript code generator and main API
//!
//! This crate provides TypeScript code generation from Rust types
//! and serves as the main entry point for the gear-mesh library.

mod branded;
mod typescript;
mod validation_gen;

#[cfg(test)]
mod tests;

pub use branded::BrandedTypeGenerator;
pub use typescript::TypeScriptGenerator;
pub use validation_gen::ValidationGenerator;

// ============================================================================
// Facade: Re-export core types for convenient access
// ============================================================================

pub use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType, GenericParam,
    NewtypeType, PrimitiveType, SerdeFieldAttrs, StructType, TypeAttributes, TypeKind, TypeRef,
    ValidationRule, VariantContent,
};

// Re-export derive macro
pub use gear_mesh_derive::GearMesh;

/// Trait for types that can be exported to TypeScript
///
/// This trait is automatically implemented by the `#[derive(GearMesh)]` macro.
pub trait GearMeshExport {
    /// Get the intermediate representation of this type
    fn gear_mesh_type() -> GearMeshType;

    /// Get the name of this type
    fn type_name() -> &'static str;
}

// ============================================================================
// Generator Configuration
// ============================================================================

/// 生成設定
#[derive(Debug, Clone, Default)]
pub struct GeneratorConfig {
    /// BigIntを自動的に使用するか
    pub use_bigint: bool,
    /// Branded Typeを生成するか
    pub generate_branded: bool,
    /// バリデーション関数を生成するか
    pub generate_validation: bool,
    /// Zodスキーマを生成するか
    pub generate_zod: bool,
    /// JSDocを生成するか
    pub generate_jsdoc: bool,
    /// インデント文字列
    pub indent: String,
}

impl GeneratorConfig {
    pub fn new() -> Self {
        Self {
            use_bigint: true,
            generate_branded: true,
            generate_validation: false,
            generate_zod: false,
            generate_jsdoc: true,
            indent: "    ".to_string(),
        }
    }

    pub fn with_bigint(mut self, use_bigint: bool) -> Self {
        self.use_bigint = use_bigint;
        self
    }

    pub fn with_branded(mut self, generate: bool) -> Self {
        self.generate_branded = generate;
        self
    }

    pub fn with_validation(mut self, generate: bool) -> Self {
        self.generate_validation = generate;
        self
    }

    pub fn with_zod(mut self, generate: bool) -> Self {
        self.generate_zod = generate;
        self
    }

    pub fn with_jsdoc(mut self, generate: bool) -> Self {
        self.generate_jsdoc = generate;
        self
    }
}
