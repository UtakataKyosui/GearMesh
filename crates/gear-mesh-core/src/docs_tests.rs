//! ドキュメントコメント処理の追加テスト

use super::*;

#[test]
fn test_parse_multiline_description() {
    let doc = r#"This is a summary.

This is a detailed description
that spans multiple lines.
"#;
    let parsed = DocComment::parse(doc);
    assert_eq!(parsed.summary, "This is a summary.");
    assert!(parsed.description.is_some());
    assert!(parsed
        .description
        .as_ref()
        .unwrap()
        .contains("detailed description"));
}

#[test]
fn test_parse_multiple_examples() {
    let doc = r#"Summary

# Examples

```
let x = 1;
```

# Examples

```
let y = 2;
```
"#;
    let parsed = DocComment::parse(doc);
    assert_eq!(parsed.examples.len(), 2);
}

#[test]
fn test_parse_custom_sections() {
    let doc = r#"Summary

# Arguments

* `x` - The x coordinate

# Returns

The result value
"#;
    let parsed = DocComment::parse(doc);
    assert_eq!(parsed.sections.len(), 2);
    assert_eq!(parsed.sections[0].name, "Arguments");
    assert_eq!(parsed.sections[1].name, "Returns");
}

#[test]
fn test_to_jsdoc_with_sections() {
    let doc = DocComment {
        summary: "Calculate sum".to_string(),
        description: Some("This function adds two numbers".to_string()),
        examples: vec!["const result = add(1, 2);".to_string()],
        sections: vec![DocSection {
            name: "Returns".to_string(),
            content: "The sum of the two numbers".to_string(),
        }],
    };

    let jsdoc = doc.to_jsdoc();
    assert!(jsdoc.contains("Calculate sum"));
    assert!(jsdoc.contains("This function adds two numbers"));
    assert!(jsdoc.contains("@example"));
    assert!(jsdoc.contains("@returns"));
}

#[test]
fn test_inline_jsdoc_empty() {
    let doc = DocComment::empty();
    let inline = doc.to_inline_jsdoc();
    assert_eq!(inline, "");
}

#[test]
fn test_inline_jsdoc_with_content() {
    let doc = DocComment::summary("A user ID");
    let inline = doc.to_inline_jsdoc();
    assert_eq!(inline, "/** A user ID */");
}
