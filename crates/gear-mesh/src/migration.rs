use std::collections::BTreeMap;

#[derive(Debug, Clone, Default)]
pub struct TypeScriptSnapshot {
    pub interfaces: BTreeMap<String, InterfaceDef>,
    pub aliases: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct InterfaceDef {
    pub fields: BTreeMap<String, FieldDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef {
    pub ty: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MigrationReport {
    pub breaking_changes: Vec<String>,
    pub additions: Vec<String>,
}

impl MigrationReport {
    pub fn is_empty(&self) -> bool {
        self.breaking_changes.is_empty() && self.additions.is_empty()
    }

    pub fn to_markdown(&self, from_label: &str, to_label: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "# Migration Guide: {} -> {}\n\n",
            from_label, to_label
        ));

        out.push_str("## Breaking Changes\n");
        if self.breaking_changes.is_empty() {
            out.push_str("- None detected\n");
        } else {
            for change in &self.breaking_changes {
                out.push_str(&format!("- {}\n", change));
            }
        }

        out.push_str("\n## Additions\n");
        if self.additions.is_empty() {
            out.push_str("- None detected\n");
        } else {
            for change in &self.additions {
                out.push_str(&format!("- {}\n", change));
            }
        }

        out
    }
}

pub fn diff_typescript(old_source: &str, new_source: &str) -> MigrationReport {
    let old_snapshot = parse_typescript_snapshot(old_source);
    let new_snapshot = parse_typescript_snapshot(new_source);

    let mut report = MigrationReport::default();

    for (name, old_interface) in &old_snapshot.interfaces {
        let Some(new_interface) = new_snapshot.interfaces.get(name) else {
            report
                .breaking_changes
                .push(format!("Type `{}` was removed", name));
            continue;
        };

        for (field, old_field) in &old_interface.fields {
            let Some(new_field) = new_interface.fields.get(field) else {
                report
                    .breaking_changes
                    .push(format!("`{}.{}` was removed", name, field));
                continue;
            };

            if old_field.ty != new_field.ty {
                report.breaking_changes.push(format!(
                    "`{}.{}` changed from `{}` to `{}`",
                    name, field, old_field.ty, new_field.ty
                ));
            }
            if old_field.optional != new_field.optional {
                report.breaking_changes.push(format!(
                    "`{}.{}` changed optionality from `{}` to `{}`",
                    name, field, old_field.optional, new_field.optional
                ));
            }
        }

        for field in new_interface.fields.keys() {
            if !old_interface.fields.contains_key(field) {
                report
                    .additions
                    .push(format!("`{}.{}` was added", name, field));
            }
        }
    }

    for name in new_snapshot.interfaces.keys() {
        if !old_snapshot.interfaces.contains_key(name) {
            report.additions.push(format!("Type `{}` was added", name));
        }
    }

    for (name, old_alias) in &old_snapshot.aliases {
        match new_snapshot.aliases.get(name) {
            None => report
                .breaking_changes
                .push(format!("Type alias `{}` was removed", name)),
            Some(new_alias) if new_alias != old_alias => report.breaking_changes.push(format!(
                "Type alias `{}` changed from `{}` to `{}`",
                name, old_alias, new_alias
            )),
            Some(_) => {}
        }
    }

    for name in new_snapshot.aliases.keys() {
        if !old_snapshot.aliases.contains_key(name) {
            report
                .additions
                .push(format!("Type alias `{}` was added", name));
        }
    }

    report
}

pub fn parse_typescript_snapshot(source: &str) -> TypeScriptSnapshot {
    let mut snapshot = TypeScriptSnapshot::default();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if let Some(body) = trimmed
            .strip_prefix("export interface ")
            .and_then(|rest| rest.strip_suffix(" {"))
        {
            let name = body.split('<').next().unwrap_or("").trim();
            if name.is_empty() {
                continue;
            }
            let mut interface = InterfaceDef::default();
            for field_line in lines.by_ref() {
                let trimmed = field_line.trim();
                if trimmed == "}" {
                    break;
                }
                if trimmed.starts_with("/**") || trimmed.starts_with('*') || trimmed.is_empty() {
                    continue;
                }

                if let Some((field, ty)) = parse_field(trimmed) {
                    interface.fields.insert(field, ty);
                }
            }
            snapshot.interfaces.insert(name.to_string(), interface);
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("export type ")
            && let Some((name, body)) = rest.split_once(" = ")
        {
            snapshot.aliases.insert(
                name.trim().to_string(),
                body.trim_end_matches(';').trim().to_string(),
            );
        }
    }

    snapshot
}

fn parse_field(line: &str) -> Option<(String, FieldDef)> {
    let normalized = line.trim_end_matches(';');
    let (field, ty) = normalized.split_once(':')?;
    let field = field.trim();
    let optional = field.ends_with('?');
    Some((
        field.trim_end_matches('?').to_string(),
        FieldDef {
            ty: ty.trim().to_string(),
            optional,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_detects_removed_and_changed_fields() {
        let old_ts = r#"
export interface User {
    id: number;
    email: string;
}
"#;
        let new_ts = r#"
export interface User {
    id: number | null;
}
"#;

        let report = diff_typescript(old_ts, new_ts);
        assert!(
            report
                .breaking_changes
                .contains(&"`User.id` changed from `number` to `number | null`".to_string())
        );
        assert!(
            report
                .breaking_changes
                .contains(&"`User.email` was removed".to_string())
        );
    }

    #[test]
    fn diff_detects_optionality_changes() {
        let old_ts = r#"
export interface User {
    name?: string;
}
"#;
        let new_ts = r#"
export interface User {
    name: string;
}
"#;

        let report = diff_typescript(old_ts, new_ts);
        assert!(
            report
                .breaking_changes
                .contains(&"`User.name` changed optionality from `true` to `false`".to_string())
        );
    }

    #[test]
    fn parser_handles_generic_interfaces() {
        let source = r#"
export interface ApiResponse<T> {
    payload?: T;
}
"#;

        let snapshot = parse_typescript_snapshot(source);
        assert!(snapshot.interfaces.contains_key("ApiResponse"));
        assert_eq!(
            snapshot.interfaces["ApiResponse"].fields["payload"],
            FieldDef {
                ty: "T".to_string(),
                optional: true,
            }
        );
    }
}
