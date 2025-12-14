//! ドキュメントコメント処理
//!
//! Rustのdocコメントを解析し、JSDoc形式に変換します。

use serde::{Deserialize, Serialize};

/// ドキュメントコメント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocComment {
    /// メインの説明文
    pub summary: String,
    /// 詳細な説明
    pub description: Option<String>,
    /// @example セクション
    pub examples: Vec<String>,
    /// その他のセクション
    pub sections: Vec<DocSection>,
}

impl DocComment {
    /// 空のドキュメントコメント
    pub fn empty() -> Self {
        Self {
            summary: String::new(),
            description: None,
            examples: Vec::new(),
            sections: Vec::new(),
        }
    }

    /// 単純なサマリーのみのドキュメント
    pub fn summary(text: impl Into<String>) -> Self {
        Self {
            summary: text.into(),
            description: None,
            examples: Vec::new(),
            sections: Vec::new(),
        }
    }

    /// Rustのdocコメントからパース
    pub fn parse(doc: &str) -> Self {
        let lines: Vec<&str> = doc.lines().collect();
        let mut summary = String::new();
        let mut description = String::new();
        let mut examples = Vec::new();
        let mut sections = Vec::new();
        let mut current_section: Option<(String, String)> = None;
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut parsing_summary = true;

        for line in lines {
            let trimmed = line.trim();

            // コードブロックの処理
            if trimmed.starts_with("```") {
                if in_code_block {
                    // コードブロック終了
                    in_code_block = false;
                    if let Some((ref section_name, _)) = current_section {
                        if section_name == "Examples" || section_name == "Example" {
                            examples.push(code_block_content.clone());
                        }
                    }
                    code_block_content.clear();
                } else {
                    // コードブロック開始
                    in_code_block = true;
                }
                continue;
            }

            if in_code_block {
                if !code_block_content.is_empty() {
                    code_block_content.push('\n');
                }
                code_block_content.push_str(line);
                continue;
            }

            // セクションヘッダの処理
            if trimmed.starts_with("# ") {
                // 前のセクションを保存
                if let Some((name, content)) = current_section.take() {
                    if name != "Examples" && name != "Example" {
                        sections.push(DocSection {
                            name,
                            content: content.trim().to_string(),
                        });
                    }
                }
                current_section = Some((trimmed[2..].to_string(), String::new()));
                parsing_summary = false;
                continue;
            }

            // 空行はサマリーの終わり
            if trimmed.is_empty() && parsing_summary && !summary.is_empty() {
                parsing_summary = false;
                continue;
            }

            // 内容の追加
            if let Some((_, ref mut content)) = current_section {
                if !content.is_empty() {
                    content.push('\n');
                }
                content.push_str(trimmed);
            } else if parsing_summary {
                if !summary.is_empty() {
                    summary.push(' ');
                }
                summary.push_str(trimmed);
            } else {
                if !description.is_empty() {
                    description.push('\n');
                }
                description.push_str(trimmed);
            }
        }

        // 最後のセクションを保存
        if let Some((name, content)) = current_section {
            if name != "Examples" && name != "Example" {
                sections.push(DocSection {
                    name,
                    content: content.trim().to_string(),
                });
            }
        }

        Self {
            summary: summary.trim().to_string(),
            description: if description.is_empty() {
                None
            } else {
                Some(description.trim().to_string())
            },
            examples,
            sections,
        }
    }

    /// JSDoc形式に変換
    pub fn to_jsdoc(&self) -> String {
        let mut lines = Vec::new();
        lines.push("/**".to_string());

        // サマリー
        if !self.summary.is_empty() {
            lines.push(format!(" * {}", self.summary));
        }

        // 説明
        if let Some(ref desc) = self.description {
            lines.push(" *".to_string());
            for line in desc.lines() {
                lines.push(format!(" * {}", line));
            }
        }

        // サンプル
        for example in &self.examples {
            lines.push(" *".to_string());
            lines.push(" * @example".to_string());
            lines.push(" * ```typescript".to_string());
            for line in example.lines() {
                lines.push(format!(" * {}", line));
            }
            lines.push(" * ```".to_string());
        }

        // その他のセクション
        for section in &self.sections {
            lines.push(" *".to_string());
            lines.push(format!(" * @{}", section.name.to_lowercase()));
            for line in section.content.lines() {
                lines.push(format!(" * {}", line));
            }
        }

        lines.push(" */".to_string());
        lines.join("\n")
    }

    /// 単一行JSDocコメント
    pub fn to_inline_jsdoc(&self) -> String {
        if self.summary.is_empty() {
            String::new()
        } else {
            format!("/** {} */", self.summary)
        }
    }
}

/// ドキュメントセクション
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    /// セクション名 (e.g., "Arguments", "Returns")
    pub name: String,
    /// セクション内容
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let doc = "This is a simple description.";
        let parsed = DocComment::parse(doc);
        assert_eq!(parsed.summary, "This is a simple description.");
    }

    #[test]
    fn test_parse_with_example() {
        let doc = r#"User information struct

# Examples

```
let user = User::new("Alice");
```
"#;
        let parsed = DocComment::parse(doc);
        assert_eq!(parsed.summary, "User information struct");
        assert_eq!(parsed.examples.len(), 1);
    }

    #[test]
    fn test_to_jsdoc() {
        let doc = DocComment {
            summary: "A user object".to_string(),
            description: None,
            examples: vec!["const user = new User();".to_string()],
            sections: vec![],
        };
        let jsdoc = doc.to_jsdoc();
        assert!(jsdoc.contains("A user object"));
        assert!(jsdoc.contains("@example"));
    }
}

// 追加テストをインクルード
#[cfg(test)]
#[path = "docs_tests.rs"]
mod docs_additional_tests;
