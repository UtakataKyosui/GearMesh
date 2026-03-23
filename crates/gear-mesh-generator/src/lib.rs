//! gear-mesh-generator: TypeScript code generator and main API
//!
//! This crate provides TypeScript code generation from Rust types
//! and serves as the main entry point for the gear-mesh library.

mod branded;
mod module_organizer;
mod typescript;
pub mod utils;
mod validation_gen;

use std::fmt;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub use branded::BrandedTypeGenerator;
pub use module_organizer::{ModuleOrganizer, ModuleStrategy};
pub use typescript::TypeScriptGenerator;
pub use validation_gen::ValidationGenerator;

// ============================================================================
// Facade: Re-export core types for convenient access
// ============================================================================

pub use gear_mesh_core::{
    DocComment, EnumRepresentation, EnumType, EnumVariant, FieldInfo, GearMeshType, GenericParam,
    NewtypeType, PrimitiveType, SerdeFieldAttrs, StructType, TypeAttributes, TypeKind, TypeRef,
    TypeTransformer, ValidationRule, VariantContent,
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

/// TypeScript mapping strategy for `Option<T>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionStyle {
    /// Generate `T | null`.
    Nullable,
    /// Generate an optional property key with type `T`, and `T | undefined` in nested contexts.
    Optional,
    /// Generate an optional property key with type `T | null`, and `T | null | undefined` in nested contexts.
    Both,
}

impl Default for OptionStyle {
    fn default() -> Self {
        Self::Nullable
    }
}

/// TypeScript mapping strategy for `Result<T, E>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultStyle {
    /// Generate only the success type `T`.
    OkOnly,
    /// Generate `{ ok: T } | { err: E }`.
    TaggedUnion,
    /// Generate `{ success: true; data: T } | { success: false; error: E }`.
    SuccessError,
}

impl Default for ResultStyle {
    fn default() -> Self {
        Self::OkOnly
    }
}

/// 生成設定
#[derive(Clone)]
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
    /// `Option<T>` の出力スタイル
    pub option_style: OptionStyle,
    /// `Result<T, E>` の出力スタイル
    pub result_style: ResultStyle,
    /// 出力モジュールの構成
    pub module_strategy: ModuleStrategy,
    /// カスタム型変換プラグイン
    pub transformers: Vec<Arc<dyn TypeTransformer>>,
    /// インデント文字列
    pub indent: String,
}

impl fmt::Debug for GeneratorConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeneratorConfig")
            .field("use_bigint", &self.use_bigint)
            .field("generate_branded", &self.generate_branded)
            .field("generate_validation", &self.generate_validation)
            .field("generate_zod", &self.generate_zod)
            .field("generate_jsdoc", &self.generate_jsdoc)
            .field("option_style", &self.option_style)
            .field("result_style", &self.result_style)
            .field("module_strategy", &self.module_strategy)
            .field("transformers", &self.transformers.len())
            .field("indent", &self.indent)
            .finish()
    }
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorConfig {
    pub fn new() -> Self {
        Self {
            use_bigint: true,
            generate_branded: true,
            generate_validation: false,
            generate_zod: false,
            generate_jsdoc: true,
            option_style: OptionStyle::Nullable,
            result_style: ResultStyle::OkOnly,
            module_strategy: ModuleStrategy::SingleFile,
            transformers: Vec::new(),
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

    pub fn with_option_style(mut self, option_style: OptionStyle) -> Self {
        self.option_style = option_style;
        self
    }

    pub fn with_result_style(mut self, result_style: ResultStyle) -> Self {
        self.result_style = result_style;
        self
    }

    pub fn with_module_strategy(mut self, module_strategy: ModuleStrategy) -> Self {
        self.module_strategy = module_strategy;
        self
    }

    pub fn with_transformer<T>(mut self, transformer: T) -> Self
    where
        T: TypeTransformer + 'static,
    {
        self.transformers.push(Arc::new(transformer));
        self
    }

    pub fn with_transformer_arc(mut self, transformer: Arc<dyn TypeTransformer>) -> Self {
        self.transformers.push(transformer);
        self
    }
}
