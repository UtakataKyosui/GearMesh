//! gear-mesh-core: 中間表現(IR)と型システム
//!
//! このクレートは、RustとTypeScript間の型変換における
//! 言語非依存の中間表現を提供します。

mod types;
mod validation;
mod docs;

pub use types::*;
pub use validation::*;
pub use docs::*;
