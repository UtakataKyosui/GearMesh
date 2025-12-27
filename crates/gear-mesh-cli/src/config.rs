//! 設定ファイル処理

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// gear-mesh設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 入力ディレクトリ
    #[serde(default = "default_input")]
    pub input: PathBuf,

    /// 出力ディレクトリ
    #[serde(default = "default_output")]
    pub output: PathBuf,

    /// BigInt自動変換
    #[serde(default = "default_true")]
    pub use_bigint: bool,

    /// Branded Type生成
    #[serde(default = "default_true")]
    pub generate_branded: bool,

    /// バリデーション関数生成
    #[serde(default)]
    pub generate_validation: bool,

    /// Zodスキーマ生成
    #[serde(default)]
    pub generate_zod: bool,

    /// JSDoc生成
    #[serde(default = "default_true")]
    pub generate_jsdoc: bool,

    /// 出力ファイル名
    #[serde(default = "default_output_file")]
    pub output_file: String,
}

fn default_input() -> PathBuf {
    PathBuf::from("src")
}

fn default_output() -> PathBuf {
    PathBuf::from("bindings")
}

fn default_true() -> bool {
    true
}

fn default_output_file() -> String {
    "types.ts".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input: default_input(),
            output: default_output(),
            use_bigint: true,
            generate_branded: true,
            generate_validation: false,
            generate_zod: false,
            generate_jsdoc: true,
            output_file: default_output_file(),
        }
    }
}

impl Config {
    /// 設定ファイルを読み込む
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 設定をGeneratorConfigに変換
    pub fn to_generator_config(&self) -> gear_mesh_generator::GeneratorConfig {
        gear_mesh_generator::GeneratorConfig::new()
            .with_bigint(self.use_bigint)
            .with_branded(self.generate_branded)
            .with_validation(self.generate_validation)
            .with_zod(self.generate_zod)
            .with_jsdoc(self.generate_jsdoc)
    }
}

/// 設定ファイルを初期化
pub fn init_config() -> Result<()> {
    let config = Config::default();
    let _content = toml::to_string_pretty(&config)?;

    let config_content = r#"# gear-mesh configuration file

# Input directory containing Rust source files
input = "src"

# Output directory for TypeScript files
output = "bindings"

# Output file name
output_file = "types.ts"

# Enable BigInt for u64/i64 types
use_bigint = true

# Generate Branded Types for newtype patterns
generate_branded = true

# Generate validation functions
generate_validation = false

# Generate Zod schemas
generate_zod = false

# Generate JSDoc comments
generate_jsdoc = true
"#;

    fs::write("gear-mesh.toml", config_content)?;
    println!("Created gear-mesh.toml");

    Ok(())
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
