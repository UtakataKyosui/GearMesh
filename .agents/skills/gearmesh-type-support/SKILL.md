---
name: gearmesh-type-support
description: Add or change Rust type support in GearMesh, tracing derive parsing through IR and TypeScript generation to consumer verification. Use for type mappings and attributes that affect generated types.
---

# GearMesh type support

Read the repository `AGENTS.md` and `docs/TESTING.md` for shared rules and
commands. Keep the requested type/attribute and configuration scope explicit.

1. Define the Rust input and expected TypeScript before editing. Check default
   and affected non-default options in `crates/gear-mesh-generator/src/lib.rs`.
   Distinguish nullable values, optional object keys, BigInt representation,
   serde-renamed fields and branded identity. Record a compatibility decision
   when an existing output would change.
2. Trace only the needed layers: `gear-mesh-derive/src/parser.rs` and
   `attributes.rs` for parsing; `gear-mesh-core/src/types.rs` for IR and primitive
   mappings; `gear-mesh-generator/src/typescript.rs` for rendering and
   `validation_gen.rs` for affected Zod/validation output. Paths are under
   `crates/`. Inspect `gear-mesh/src/` when collection or output is involved.
3. Add focused regression cases for the requested behavior and relevant invalid
   input. Existing parser tests use `syn::parse_quote!`; generator tests and
   snapshots are in `crates/gear-mesh-generator/src/tests.rs`. A parsed IR test
   does not verify generated macro code: add a real derive fixture/integration
   case when expansion is affected. For diagnostics, verify a failing expansion
   if relevant; do not assume existing compile-fail coverage.
4. Extend `crates/gear-mesh/examples/codegen_fixture.rs` and
   `tests/e2e/consumer.ts` when the mapping should be
   covered end to end. The example has direct access to `serde_json`, required
   by current macro output, and runs with all features. Keep type ordering
   explicit. Assert both valid assignments and targeted invalid assignments
   with `@ts-expect-error`; broad casts would hide a mapping regression.
5. Run focused Rust tests and `npm run test:codegen`. Review any candidate
   output against the specification before updating snapshots using the
   commands in `docs/TESTING.md`. Extend TS dependency/configuration coverage
   if the change requires it, rather than claiming the basic fixture checks
   all generated modes.
6. Update affected public examples/specifications and run `npm test`. Report
   the changed mapping, compatibility implications, and checks actually run.

For a read-only walkthrough, trace the existing `Option<String>` field
`User.nickname` in `crates/gear-mesh/examples/codegen_fixture.rs`: locate its parser/IR representation,
confirm `OptionStyle::Nullable`, inspect the expected required `string | null`
property and its positive/negative consumer cases, then run the focused
`test_option` tests and `npm run test:codegen`. A walkthrough does not require
adding a new feature or rewriting expectations.
