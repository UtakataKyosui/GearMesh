# GearMesh development

## Architecture and specifications

- `crates/gear-mesh-core/src/types.rs`: intermediate representation (IR) and type mappings.
- `crates/gear-mesh-derive/src/parser.rs` and `attributes.rs`: Rust syntax/attributes to IR; `lib.rs` emits proc-macro implementations.
- `crates/gear-mesh-generator/src/typescript.rs`: TypeScript rendering; `validation_gen.rs` handles validation/Zod. `lib.rs` defines generation options.
- `crates/gear-mesh/src/`: public facade, inventory collection, output orchestration, caching and migration support.
- Read `docs/ARCHITECTURE.md` for the data flow, `docs/STATE.md` for state slots, and `docs/TESTING.md` for executable verification. Consult public examples in `README.md` and the affected crate's README for intended behavior.

## Changes to generated code

- Trace a changed Rust type through parsing, IR, rendering and TypeScript consumers. Update only the layers that need the change.
- Preserve default mappings unless changing them is explicitly in scope. In particular, distinguish `Option<T>` nullability from optional keys, configured BigInt behavior, and serde names. Consider existing configuration modes and downstream compatibility.
- Add focused normal/error cases and update documentation/consumer examples when public behavior changes. Parser tests alone do not prove that a proc-macro expansion compiles.
- Edit the Rust fixture/generator, then regenerate output. Do not hand-edit disposable generated files to make checks pass.
- Snapshot expectations are reviewed contracts: compare the generated diff with the intended specification before updating them. `npm test` must never run with `UPDATE_SNAPSHOTS` set. See `docs/TESTING.md` for explicit regeneration commands.

## Setup and verification

- Follow `CONTRIBUTING.md` for Rust/Node setup and `npm ci --ignore-scripts`. Keep the root `package-lock.json` in sync with development dependencies. Do not install tooling implicitly from test commands.
- While editing, run `cargo test -p <affected-crate> --all-features` or a named test. For derive/output changes, also run `npm run test:codegen`.
- Before handing off code changes, run `npm test`: formatting, a default-feature build check, Clippy, workspace tests (including Rust snapshots), real derive generation, TypeScript consumer checking and output comparison. `moon run :verify` is an optional wrapper after dependency installation.
- Docker is optional; `bash tests/e2e/run-docker-test.sh` runs the standard checks in a container. Aeneas translation and security/dependency audits are separate CI checks; the standard command does not replace them.
- Report checks actually run and any failures or unavailable tooling. Do not weaken checks to hide existing failures.

## Repository hygiene

- Preserve unrelated user changes. Do not commit, push, create PRs, publish crates, or change remote services unless requested.
- Do not add credentials, personal Codex settings, generated caches, or machine-specific paths. Only the root npm lockfile is tracked; follow the existing policy for other lockfiles.
- For a type-support task, the project skill is `.agents/skills/gearmesh-type-support/SKILL.md`.
