# Testing GearMesh

## Setup and standard verification

Follow [CONTRIBUTING.md](../CONTRIBUTING.md) for Rust 1.90.0, Node.js
22.12.0 and npm 10.9.0. Run `npm ci --ignore-scripts` from the repository root
to install the root lockfile's development dependencies, including TypeScript
5.9.3.

```bash
npm test
```

This runs, in order:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-features` (including generator snapshots)
4. `npm run test:codegen`'s underlying script: derive expansion, generation,
   TypeScript consumer check, and comparison with `types.expected.ts`

Any failed subprocess, missing output/expectation, type error or snapshot
difference fails the command. `UPDATE_SNAPSHOTS` must be unset: normal
verification rejects it to prevent accidentally accepting changed output.
The `Standard Verification` job in `.github/workflows/rust-ci.yaml` runs
`npm test` after installing the same npm dependencies.

With moon installed, `moon run :verify` calls the same command. Install npm
dependencies first. `moon run :test` and `:test-all` remain Rust-only checks.

## Focused checks

```bash
cargo test -p gear-mesh-derive --all-features
cargo test -p gear-mesh-generator --all-features test_option
cargo test -p gear-mesh-generator --all-features test_snapshot
npm run test:codegen
```

Rust unit tests live next to their implementation. Integration tests live in
`crates/gear-mesh/tests/` and `crates/gear-mesh-generator/tests/`.
For macro changes, exercise actual derive expansion as well as parser tests.

## Derive-to-TypeScript regression fixture

- `crates/gear-mesh/examples/codegen_fixture.rs`: a Cargo example defining real
  `#[derive(GearMesh)]` types and rendering their IR in an explicit order.
- `tests/e2e/types.expected.ts`: the reviewed expected generated source.
- `tests/e2e/consumer.ts`: valid uses and `@ts-expect-error` cases that must
  remain invalid (different brands, wrong integer representation, invalid
  enum value, undefined and missing nullable fields).
- `tests/e2e/tsconfig.json`: ES2020, strict checking, no emit. The runner
  uses `tsc --project` so the configuration is applied.
- `scripts/check-codegen.mjs`: generates into a fresh
  `tests/e2e/.generated-*/` directory, checks and compares, then removes that
  directory on success. Failed runs retain their files for inspection.

The fixture covers structs, primitive fields, branded newtypes, a unit enum,
`Vec<String>`, default nullable `Option<String>`, `u64` BigInt output,
JSDoc and serde camelCase field names. It neither substitutes hand-written
output for failed generation nor reuses output from an earlier run.

The current fixture is a baseline, not exhaustive configuration coverage.
Zod/validation output, alternate Option/Result modes, module organization and
state slots have Rust tests; they are not all type-checked by this TS fixture.
Compile-fail tests through actual macro expansion and runtime behavior of
generated code remain separate coverage gaps.

## Updating expected output

Only update expectations after reviewing the intended behavior change.
For the derive fixture, generate a candidate into the ignored target directory:

```bash
cargo run -p gear-mesh --all-features --example codegen-fixture -- target/codegen-candidate.ts
git diff --no-index -- tests/e2e/types.expected.ts target/codegen-candidate.ts
```

The diff command exits 1 when files differ. After reviewing the difference,
copy the candidate to `tests/e2e/types.expected.ts`, update consumer cases as
needed, then run `npm test`. Do not edit temporary output to pass verification.

Generator unit-test snapshots live in
`crates/gear-mesh-generator/tests/snapshots/`. To explicitly regenerate them:

```bash
UPDATE_SNAPSHOTS=1 cargo test -p gear-mesh-generator --all-features test_snapshot
git diff -- crates/gear-mesh-generator/tests/snapshots/
npm test
```

## Optional Docker verification

```bash
bash tests/e2e/run-docker-test.sh
```

This builds from the repository root and runs `npm test` inside a Linux
container with Rust 1.90.0, Node.js 22.12.0 and npm 10.9.0. It needs a running
Docker engine and network access for image/package downloads. It does not
mount or modify the host checkout. The two legacy scripts
`test-e2e.sh` and `test-e2e-simple.sh` now delegate to the local
`npm run test:codegen` check and work from any working directory.

## Separate checks and known limitations

- Security Audit, cargo-deny, rustdoc and Aeneas translation remain separate
  CI checks. Security Audit's known installation issue is tracked by #23.
- A default-feature `cargo build -p gear-mesh` currently fails because
  `cache.rs` uses serde/serde_json while those dependencies are enabled by
  the `cli` feature. This predates the environment work. The standard
  verification and fixture use `--all-features`, matching existing CI;
  passing them does not establish default-feature build support.
- The root npm lockfile is tracked. Cargo lockfiles follow the repository's
  existing ignored-lockfile policy, so Rust dependency resolution can vary
  between clean installations.

## Issue #24 implementation verification

The initial setup was checked on macOS with Rust 1.90.0, Node.js 22.22.2,
npm 10.9.7 and the locked TypeScript 5.9.3. CI and the Dockerfile specify
Node.js 22.12.0/npm 10.9.0; their execution is not implied by the local result.

| Stage | Evidence |
| --- | --- |
| Tooling and fixture | `npm ci --ignore-scripts` and `npm test` passed; real derive output matched the expectation and passed strict TS consumer checking. |
| Repository instructions | Reviewed `AGENTS.md` references against the actual crate files, specifications and commands. |
| Failure propagation | In a disposable copy, intentional TS errors, snapshot differences, generation errors and a failing Rust test all produced nonzero exits. The legacy wrapper also propagated the TS failure. Standard verification rejected `UPDATE_SNAPSHOTS=1`. |
| Skill | The skill-creator validator passed. The existing `User.nickname: Option<String>` walkthrough reached the parser's optional flag, `OptionStyle::Nullable`, the required `string \| null` output, two focused `test_option` tests and the TS consumer check. |
| Configuration and wrappers | `actionlint`, `shellcheck` and `git diff --check` passed. The legacy simple wrapper also passed when launched outside the repository. |

Docker verification was attempted but did not complete: the image build
stalled at the npm setup step, and a separate Node container was also very slow
to start Node/npm. The task's build and diagnostic container were stopped.
The Docker path therefore remains unverified in this environment. No remote
CI run was triggered, and fresh-session Skill discovery still needs to be
confirmed when opening the next Codex session.
