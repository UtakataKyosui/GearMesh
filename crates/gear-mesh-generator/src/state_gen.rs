//! 状態スロットのTypeScript生成
//!
//! 生成されるのは投影であって、ストアではありません。値を保持するのは
//! Rust側だけであり、生成コードはモジュールスコープに状態値を持ちません。

use std::collections::BTreeSet;

use gear_mesh_core::{
    StateSlot, TypeRef, is_builtin_type, is_internal_type, to_typescript_primitive,
};

use crate::{GeneratorConfig, TypeScriptGenerator};

/// 4スペースで書かれたランタイム部分を、設定のインデントに合わせて整形する
fn reindent(source: &str, indent: &str) -> String {
    if indent == "    " {
        return source.to_string();
    }

    source
        .lines()
        .map(|line| {
            let depth = line.len() - line.trim_start_matches(' ').len();
            let (levels, rest) = (depth / 4, depth % 4);
            format!(
                "{}{}{}",
                indent.repeat(levels),
                " ".repeat(rest),
                line.trim_start_matches(' ')
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 生成物の先頭に置くランタイム
///
/// トランスポートは保持しますが、状態値そのものは保持しません。
const STATE_RUNTIME: &str = r#"/** A state value together with the revision it was produced at. */
export interface Stamped<T> {
    readonly rev: number;
    readonly value: T;
}

/** Stops a listener registered with `subscribe` or `watch`. */
export type Unsubscribe = () => void;

/**
 * Reaches the Rust side. Supplied by the host application — Tauri
 * `invoke`/`listen`, HTTP + SSE, or a test double — so that the slots below
 * stay independent of any transport and of any UI framework.
 */
export interface StateTransport {
    /** Reads the current value of a slot. */
    read<T>(command: string): Promise<Stamped<T>>;
    /** Delivers every subsequent change of a slot. */
    subscribe<T>(
        event: string,
        handler: (next: Stamped<T>) => void,
    ): Promise<Unsubscribe> | Unsubscribe;
}

let transport: StateTransport | null = null;

/** Registers the transport. Call this once, during application start-up. */
export function setStateTransport(next: StateTransport): void {
    transport = next;
}

function requireTransport(): StateTransport {
    if (transport === null) {
        throw new Error(
            "gear-mesh: no state transport registered — call setStateTransport() before using a state slot.",
        );
    }
    return transport;
}

/**
 * A value owned by the Rust side. The UI may read it and follow it; it has no
 * way to write it and must not keep an authoritative copy of it.
 */
export interface ReadonlyStateSlot<T> {
    readonly key: string;
    readonly getCommand: string;
    readonly changedEvent: string;
    /** Reads the current value. */
    get(): Promise<Stamped<T>>;
    /** Follows changes. Does not deliver the value the slot holds right now. */
    subscribe(handler: (next: Stamped<T>) => void): Promise<Unsubscribe>;
    /**
     * Reads the current value and follows changes, dropping anything older
     * than the newest revision already delivered — so a slow initial read can
     * never overwrite a change that arrived before it resolved.
     */
    watch(
        handler: (value: T) => void,
        onError?: (error: unknown) => void,
    ): Promise<Unsubscribe>;
}

function readonlySlot<T>(
    key: string,
    getCommand: string,
    changedEvent: string,
): ReadonlyStateSlot<T> {
    const get = (): Promise<Stamped<T>> => requireTransport().read<T>(getCommand);

    const subscribe = async (
        handler: (next: Stamped<T>) => void,
    ): Promise<Unsubscribe> => await requireTransport().subscribe<T>(changedEvent, handler);

    const watch = async (
        handler: (value: T) => void,
        onError?: (error: unknown) => void,
    ): Promise<Unsubscribe> => {
        // The highest revision handed to the caller. It orders deliveries; it
        // is not a copy of the value.
        let delivered = -1;
        let stopped = false;

        const deliver = (next: Stamped<T>): void => {
            if (stopped || next.rev <= delivered) {
                return;
            }
            delivered = next.rev;
            handler(next.value);
        };

        // Subscribe first, so no change can slip through while the initial
        // read is in flight.
        const unsubscribe = await subscribe(deliver);
        get()
            .then(deliver)
            .catch((error: unknown) => {
                if (!stopped) {
                    onError?.(error);
                }
            });

        return () => {
            stopped = true;
            unsubscribe();
        };
    };

    return { key, getCommand, changedEvent, get, subscribe, watch };
}
"#;

/// 状態スロットのTypeScript生成器
pub struct StateGenerator {
    config: GeneratorConfig,
}

impl StateGenerator {
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// 値の型を別モジュールから取り込まずに生成する
    pub fn generate(&self, slots: &[StateSlot]) -> String {
        self.generate_with_types_module(slots, None)
    }

    /// 値の型を `types_module` から取り込む形で生成する
    pub fn generate_with_types_module(
        &self,
        slots: &[StateSlot],
        types_module: Option<&str>,
    ) -> String {
        let mut output = String::new();

        output.push_str("// Auto-generated by gear-mesh. Do not edit.\n");
        output.push_str("//\n");
        output
            .push_str("// State slots project Rust-owned state into the UI. Read through `get`,\n");
        output.push_str("// follow changes with `subscribe`, or combine both with `watch`.\n\n");

        if let Some(module) = types_module {
            let imports = self.imported_type_names(slots);
            if !imports.is_empty() {
                output.push_str(&format!(
                    "import type {{ {} }} from '{}';\n\n",
                    imports.into_iter().collect::<Vec<_>>().join(", "),
                    module
                ));
            }
        }

        output.push_str(&reindent(STATE_RUNTIME, &self.config.indent));

        for slot in slots {
            output.push('\n');
            output.push_str(&self.render_slot(slot));
        }

        output
    }

    fn render_slot(&self, slot: &StateSlot) -> String {
        let mut output = String::new();

        if self.config.generate_jsdoc
            && let Some(docs) = &slot.docs
        {
            output.push_str(&docs.to_jsdoc());
            output.push('\n');
        }

        let value_type = self.value_type(&slot.ty);
        let binding = slot.binding_name();
        let indent = &self.config.indent;

        output.push_str(&format!(
            "export const {binding}: ReadonlyStateSlot<{value_type}> = readonlySlot<{value_type}>(\n"
        ));
        output.push_str(&format!("{indent}{:?},\n", slot.key));
        output.push_str(&format!("{indent}{:?},\n", slot.get_command));
        output.push_str(&format!("{indent}{:?},\n", slot.changed_event));
        output.push_str(");\n");

        output
    }

    fn value_type(&self, type_ref: &TypeRef) -> String {
        TypeScriptGenerator::new(self.config.clone()).type_ref_to_typescript(type_ref)
    }

    /// スロットの値の型のうち、別モジュールから取り込む必要があるもの
    fn imported_type_names(&self, slots: &[StateSlot]) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        for slot in slots {
            collect_custom_type_names(&slot.ty, self.config.use_bigint, &mut names);
        }
        names
    }
}

fn collect_custom_type_names(type_ref: &TypeRef, use_bigint: bool, names: &mut BTreeSet<String>) {
    let name = type_ref.name.as_str();
    let is_primitive = to_typescript_primitive(name, use_bigint).is_some();

    if !is_primitive && !is_builtin_type(name) && !is_internal_type(name) {
        names.insert(name.to_string());
    }

    for generic in &type_ref.generics {
        collect_custom_type_names(generic, use_bigint, names);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gear_mesh_core::DocComment;

    fn generate(slots: &[StateSlot]) -> String {
        StateGenerator::new(GeneratorConfig::new()).generate(slots)
    }

    #[test]
    fn slots_are_emitted_with_names_derived_from_the_key() {
        let output = generate(&[StateSlot::read_only(
            "effective_theme",
            TypeRef::new("ResolvedTheme"),
        )]);

        assert!(output.contains(
            "export const effectiveTheme: ReadonlyStateSlot<ResolvedTheme> = readonlySlot<ResolvedTheme>("
        ));
        assert!(output.contains("\"effective_theme\","));
        assert!(output.contains("\"state://effective_theme/get\","));
        assert!(output.contains("\"state://effective_theme/changed\","));
    }

    #[test]
    fn read_only_slots_get_no_writer() {
        let output = generate(&[StateSlot::read_only("theme", TypeRef::new("Theme"))]);

        assert!(
            !output.contains("set("),
            "a read-only slot must not expose a writer"
        );
        assert!(output.contains("get()"));
        assert!(output.contains("subscribe("));
        assert!(output.contains("watch("));
    }

    #[test]
    fn the_projection_keeps_no_value_of_its_own() {
        let output = generate(&[StateSlot::read_only("theme", TypeRef::new("Theme"))]);

        // The only module-scoped binding is the transport handle; a cached
        // value here would make the generated code a second source of truth.
        let module_scoped: Vec<_> = output
            .lines()
            .filter(|line| line.starts_with("let ") || line.starts_with("var "))
            .collect();
        assert_eq!(
            module_scoped,
            vec!["let transport: StateTransport | null = null;"]
        );
    }

    #[test]
    fn explicit_command_names_are_used_verbatim() {
        let slot = StateSlot::read_only("theme", TypeRef::new("Theme"))
            .with_get_command("get_theme")
            .with_changed_event("theme-changed");

        let output = generate(&[slot]);

        assert!(output.contains("\"get_theme\","));
        assert!(output.contains("\"theme-changed\","));
    }

    #[test]
    fn value_types_follow_the_generator_config() {
        let slot = StateSlot::read_only("opened_at", TypeRef::new("u64"));

        let bigint = StateGenerator::new(GeneratorConfig::new().with_bigint(true))
            .generate(std::slice::from_ref(&slot));
        assert!(bigint.contains("ReadonlyStateSlot<bigint>"));

        let number =
            StateGenerator::new(GeneratorConfig::new().with_bigint(false)).generate(&[slot]);
        assert!(number.contains("ReadonlyStateSlot<number>"));
    }

    #[test]
    fn only_custom_value_types_are_imported() {
        let slots = vec![
            StateSlot::read_only("theme", TypeRef::new("Theme")),
            StateSlot::read_only(
                "recent_files",
                TypeRef::with_generics("Vec", vec![TypeRef::new("String")]),
            ),
        ];

        let output = StateGenerator::new(GeneratorConfig::new())
            .generate_with_types_module(&slots, Some("./index"));

        assert!(output.contains("import type { Theme } from './index';"));
        assert!(!output.contains("String"));
    }

    #[test]
    fn slot_docs_become_jsdoc() {
        let slot = StateSlot::read_only("theme", TypeRef::new("Theme"))
            .with_docs(DocComment::parse("The theme the user selected."));

        assert!(generate(&[slot]).contains("The theme the user selected."));
    }

    #[test]
    fn reindent_respects_a_custom_indent() {
        let config = GeneratorConfig::new().with_indent("  ");
        let output = StateGenerator::new(config)
            .generate(&[StateSlot::read_only("theme", TypeRef::new("Theme"))]);

        assert!(output.contains("\n  readonly rev: number;\n"));
        assert!(output.contains("\n  \"theme\",\n"));
    }
}
