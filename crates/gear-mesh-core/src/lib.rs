//! gear-mesh-core: 中間表現(IR)と型システム
//!
//! このクレートは、RustとTypeScript間の型変換における
//! 言語非依存の中間表現を提供します。

mod docs;
mod types;
mod validation;

pub use docs::*;
pub use types::*;
pub use validation::*;
