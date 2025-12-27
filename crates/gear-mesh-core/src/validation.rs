//! バリデーションルール定義
//!
//! Rustのバリデーション属性から抽出されるルールを表現します。

use serde::{Deserialize, Serialize};

/// バリデーションルール
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    /// 範囲チェック
    Range { min: Option<f64>, max: Option<f64> },
    /// 文字列長さチェック
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },
    /// 正規表現パターン
    Pattern(String),
    /// メールアドレス形式
    Email,
    /// URL形式
    Url,
    /// 必須フィールド
    Required,
    /// カスタムバリデーション
    Custom {
        name: String,
        message: Option<String>,
    },
}

impl ValidationRule {
    /// TypeScriptのバリデーションコードを生成
    pub fn to_typescript_check(&self, field_name: &str) -> String {
        match self {
            ValidationRule::Range { min, max } => {
                let mut checks = Vec::new();
                if let Some(min) = min {
                    checks.push(format!("obj.{field_name} >= {min}"));
                }
                if let Some(max) = max {
                    checks.push(format!("obj.{field_name} <= {max}"));
                }
                if checks.is_empty() {
                    "true".to_string()
                } else {
                    checks.join(" && ")
                }
            }
            ValidationRule::Length { min, max } => {
                let mut checks = Vec::new();
                if let Some(min) = min {
                    checks.push(format!("obj.{field_name}.length >= {min}"));
                }
                if let Some(max) = max {
                    checks.push(format!("obj.{field_name}.length <= {max}"));
                }
                if checks.is_empty() {
                    "true".to_string()
                } else {
                    checks.join(" && ")
                }
            }
            ValidationRule::Pattern(pattern) => {
                format!("/{pattern}/.test(obj.{field_name})")
            }
            ValidationRule::Email => {
                format!(r#"/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(obj.{field_name})"#)
            }
            ValidationRule::Url => {
                format!(r#"/^https?:\/\/[^\s]+$/.test(obj.{field_name})"#)
            }
            ValidationRule::Required => {
                format!("obj.{field_name} !== undefined && obj.{field_name} !== null")
            }
            ValidationRule::Custom { name, .. } => {
                format!("validate{name}(obj.{field_name})")
            }
        }
    }

    /// Zodスキーマコードを生成
    pub fn to_zod_schema(&self, is_bigint: bool) -> String {
        match self {
            ValidationRule::Range { min, max } => {
                let mut schema = String::new();
                let suffix = if is_bigint { "n" } else { "" };
                if let Some(min) = min {
                    if is_bigint {
                        debug_assert!(
                            min.fract() == 0.0,
                            "Fractional value ({}) provided for integer range validation, this will be truncated to {}",
                            min,
                            *min as i128
                        );
                        schema.push_str(&format!(".min({}{suffix})", *min as i128));
                    } else {
                        schema.push_str(&format!(".min({min})"));
                    }
                }
                if let Some(max) = max {
                    if is_bigint {
                        debug_assert!(
                            max.fract() == 0.0,
                            "Fractional value ({}) provided for integer range validation, this will be truncated to {}",
                            max,
                            *max as i128
                        );
                        schema.push_str(&format!(".max({}{suffix})", *max as i128));
                    } else {
                        schema.push_str(&format!(".max({max})"));
                    }
                }
                schema
            }
            ValidationRule::Length { min, max } => {
                let mut schema = String::new();
                if let Some(min) = min {
                    schema.push_str(&format!(".min({min})"));
                }
                if let Some(max) = max {
                    schema.push_str(&format!(".max({max})"));
                }
                schema
            }
            ValidationRule::Pattern(pattern) => {
                format!(".regex(/{pattern}/)")
            }
            ValidationRule::Email => ".email()".to_string(),
            ValidationRule::Url => ".url()".to_string(),
            ValidationRule::Required => String::new(), // Zodではデフォルトで必須
            ValidationRule::Custom { name, message } => {
                if let Some(msg) = message {
                    format!(".refine(validate{name}, {{ message: \"{msg}\" }})")
                } else {
                    format!(".refine(validate{name})")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_validation() {
        let rule = ValidationRule::Range {
            min: Some(1.0),
            max: Some(100.0),
        };
        assert_eq!(
            rule.to_typescript_check("age"),
            "obj.age >= 1 && obj.age <= 100"
        );
    }

    #[test]
    fn test_email_validation() {
        let rule = ValidationRule::Email;
        assert!(rule.to_typescript_check("email").contains("@"));
    }
}
