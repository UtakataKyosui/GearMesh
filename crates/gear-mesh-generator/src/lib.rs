//! gear-mesh-generator: TypeScriptコード生成器
//!
//! GearMeshType（中間表現）からTypeScriptコードを生成します。

mod branded;
mod typescript;
mod validation_gen;

#[cfg(test)]
mod tests;

pub use branded::BrandedTypeGenerator;
pub use typescript::TypeScriptGenerator;
pub use validation_gen::ValidationGenerator;

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
