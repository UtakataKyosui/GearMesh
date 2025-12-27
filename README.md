# gear-mesh ⚙️

Next-generation Rust to TypeScript type definition sharing library.

![Rust](https://img.shields.io/badge/Rust-1.90%2B-orange)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

## Features

| Feature | Description |
|---------|-------------|
| **Branded Types** | Convert Rust newtype patterns to TypeScript Branded Types |
| **Doc Comments** | Rust doc comments → JSDoc |
| **Validation** | Generate runtime validation functions |
| **Zod Schema** | Generate Zod schemas for runtime validation |
| **BigInt Support** | Automatically use `bigint` for `u64`/`i64` |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
gear-mesh = "0.1"
```

## Quick Start

### 1. Define your types

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

### 2. Generate TypeScript

Create a `main.rs` (or a separate binary/test) to run the generation:

```rust
fn main() {
    // Generate TypeScript types to the "generated" directory
    gear_mesh::generate_types_to_dir("generated")
        .expect("Failed to generate TypeScript types");
}
```

### 3. Generated TypeScript

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

## Configuration

You can customize the generation by using `GeneratorConfig`:

```rust
use gear_mesh::{GeneratorConfig, generate_with_config};

fn main() {
    let config = GeneratorConfig::new()
        .with_bigint(true)
        .with_branded(true)
        .with_zod(true) // Generate Zod schemas
        .with_validation(true); // Generate validation functions

    gear_mesh::generate_with_config("generated", config)
        .expect("Failed to generate");
}
```

## Comparison with Existing Crates

| Feature | ts-rs | typeshare | specta | **gear-mesh** |
|---------|-------|-----------|--------|---------------|
| Basic type conversion | ✅ | ✅ | ✅ | ✅ |
| Branded Types | ❌ | ❌ | ❌ | ✅ |
| Doc comment conversion | ❌ | ❌ | ❌ | ✅ |
| Zod Schema | ❌ | ❌ | ❌ | ✅ |
| Validation embedding | ❌ | ❌ | ❌ | ✅ |
| Auto BigInt | Manual | Manual | Manual | ✅ Auto |

## Crate Structure

- `gear-mesh` - Main crate with re-exports
- `gear-mesh-core` - Intermediate representation (IR)
- `gear-mesh-derive` - `#[derive(GearMesh)]` proc-macro
- `gear-mesh-generator` - TypeScript code generator

## Implementation Status

gear-mesh v0.1.0 implements:

- ✅ Basic type conversion
- ✅ Branded Type generation
- ✅ Doc comment conversion
- ✅ BigInt support
- ✅ Zod Schema generation
- ✅ Validation rules

See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) for detailed status and [docs/FUTURE_ISSUES.md](docs/FUTURE_ISSUES.md) for planned features.

## Testing

gear-mesh has comprehensive test coverage updated regularly.

- **Unit tests** covering core logic, generator, and derive macros.
- **Integration tests** validation the full pipeline from Rust types to TypeScript output.
- **E2E tests** ensuring compatibility with real TypeScript projects using Docker.

Run all tests:

```bash
cargo test --workspace
```

Or using [moonrepo](https://moonrepo.dev/):

```bash
moon run :test
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
