# gear-mesh ⚙️

Next-generation Rust to TypeScript type definition sharing library.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

## Features

| Feature | Description |
|---------|-------------|
| **Branded Types** | Convert Rust newtype patterns to TypeScript Branded Types |
| **Doc Comments** | Rust doc comments → JSDoc |
| **Validation** | Generate runtime validation functions |
| **BigInt Support** | Automatically use `bigint` for `u64`/`i64` |
| **Watch Mode** | Real-time regeneration on file changes |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gear-mesh = "0.1"
```

## Quick Start

```rust
use gear_mesh::GearMesh;

#[derive(GearMesh)]
#[gear_mesh(branded)]
struct UserId(i32);

/// User information
#[derive(GearMesh)]
struct User {
    /// User's unique identifier
    id: UserId,
    /// User's display name
    name: String,
}
```

Generated TypeScript:

```typescript
// Branded Type helper
type Brand<T, B> = T & { readonly __brand: B };

export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;

/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
    /** User's display name */
    name: string;
}
```

## CLI Usage

```bash
# Initialize configuration
gear-mesh init

# Generate TypeScript definitions
gear-mesh generate

# Watch mode (auto-regenerate on changes)
gear-mesh watch
```

## Configuration

Create `gear-mesh.toml` in your project root:

```toml
input = "src"
output = "bindings"
output_file = "types.ts"

use_bigint = true
generate_branded = true
generate_validation = false
generate_zod = false
generate_jsdoc = true
```

## Comparison with Existing Crates

| Feature | ts-rs | typeshare | specta | **gear-mesh** |
|---------|-------|-----------|--------|---------------|
| Basic type conversion | ✅ | ✅ | ✅ | ✅ |
| Branded Types | ❌ | ❌ | ❌ | ✅ |
| Doc comment conversion | ❌ | ❌ | ❌ | ✅ |
| Validation embedding | ❌ | ❌ | ❌ | ✅ |
| Auto BigInt | Manual | Manual | Manual | ✅ Auto |
| Watch mode | ❌ | ❌ | ❌ | ✅ |

## Crate Structure

- `gear-mesh` - Main crate with re-exports
- `gear-mesh-core` - Intermediate representation (IR)
- `gear-mesh-derive` - `#[derive(GearMesh)]` proc-macro
- `gear-mesh-generator` - TypeScript code generator
- `gear-mesh-cli` - Command-line tool

## Implementation Status

gear-mesh v0.1.0 implements all Phase 1 MVP features:

- ✅ Basic type conversion
- ✅ Branded Type generation
- ✅ Doc comment conversion
- ✅ BigInt support
- ✅ Watch mode

See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) for detailed status and [docs/FUTURE_ISSUES.md](docs/FUTURE_ISSUES.md) for planned features.

## Testing

gear-mesh has comprehensive test coverage:

- **56 total tests** across all crates
- **17 unit tests** in `gear-mesh-core`
- **11 unit tests** in `gear-mesh-generator`
- **4 unit tests** in `gear-mesh-cli`
- **24 integration tests** for end-to-end workflows
  - 9 general integration tests
  - 15 code generation tests (Rust → TypeScript)

Run all tests:

```bash
cargo test --workspace
```

Run E2E tests in Docker:

```bash
./tests/e2e/run-docker-test.sh
```

See [docs/TESTING.md](docs/TESTING.md) for detailed test coverage information and [docs/E2E_TEST_RESULTS.md](docs/E2E_TEST_RESULTS.md) for end-to-end test results.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
