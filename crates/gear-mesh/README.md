# gear-mesh ⚙️

Next-generation Rust to TypeScript type definition sharing library.

[![Crates.io](https://img.shields.io/crates/v/gear-mesh.svg)](https://crates.io/crates/gear-mesh)
[![Documentation](https://docs.rs/gear-mesh/badge.svg)](https://docs.rs/gear-mesh)
![Rust](https://img.shields.io/badge/Rust-1.90%2B-orange)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)

## Features

- **🏷️ Branded Types**: Convert Rust newtype patterns to TypeScript Branded Types
- **📝 Doc Comments**: Automatically convert Rust doc comments to JSDoc
- **✅ Validation**: Generate Zod schemas with validation rules
- **🔢 BigInt Support**: Automatically use `bigint` for `u64`/`i64`
- **🎯 Type Safety**: Full type safety from Rust to TypeScript

## Quick Start

### Installation

```toml
[dependencies]
gear-mesh = "0.1"
```

### Basic Usage

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
    #[validate(length(min = 1, max = 20))]
    name: String,
    /// User's email address
    #[validate(email)]
    email: String,
}

fn main() {
    // Generate TypeScript types
    gear_mesh::generate_types_to_dir("generated")
        .expect("Failed to generate TypeScript types");
}
```

### Generated TypeScript

```typescript
// Branded Type
type Brand<T, B> = T & { readonly __brand: B };
export type UserId = Brand<number, "UserId">;

// Interface with JSDoc
/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
    /** User's display name */
    name: string;
    /** User's email address */
    email: string;
}

// Zod Schema with Validation
export const UserSchema = z.object({
    id: z.number(),
    name: z.string().min(1).max(20),
    email: z.string().email(),
});
```

## Validation Rules

| Rule | Attribute | Generated Zod |
|------|-----------|---------------|
| Range | `#[validate(range(min = 0, max = 100))]` | `.min(0).max(100)` |
| Length | `#[validate(length(min = 1, max = 20))]` | `.min(1).max(20)` |
| Email | `#[validate(email)]` | `.email()` |
| URL | `#[validate(url)]` | `.url()` |
| Pattern | `#[validate(pattern = "^[A-Z]")]` | `.regex(/^[A-Z]/)` |

## Configuration

```rust
use gear_mesh::{GeneratorConfig, generate_with_config};

let config = GeneratorConfig::new()
    .with_bigint(true)        // Use bigint for u64/i64
    .with_zod(true)           // Generate Zod schemas
    .with_validation(true)    // Include validation rules
    .with_branded(true)       // Generate Branded Types
    .with_jsdoc(true);        // Include JSDoc comments

gear_mesh::generate_with_config("generated", config)
    .expect("Failed to generate");
```

## Examples

See the [examples](https://github.com/UtakataKyosui/GearMesh/tree/main/examples) directory for complete examples:

- **simple-bigint**: Basic BigInt usage
- **axum-react**: Full-stack application with Axum backend and React frontend

## Comparison with Existing Crates

| Feature | ts-rs | typeshare | specta | **gear-mesh** |
|---------|-------|-----------|--------|---------------|
| Basic type conversion | ✅ | ✅ | ✅ | ✅ |
| Branded Types | ❌ | ❌ | ❌ | ✅ |
| Doc comment conversion | ❌ | ❌ | ❌ | ✅ |
| Zod Schema | ❌ | ❌ | ❌ | ✅ |
| Validation embedding | ❌ | ❌ | ❌ | ✅ |
| Auto BigInt | Manual | Manual | Manual | ✅ Auto |

## Documentation

- [API Documentation](https://docs.rs/gear-mesh)
- [GitHub Repository](https://github.com/UtakataKyosui/GearMesh)
- [Examples](https://github.com/UtakataKyosui/GearMesh/tree/main/examples)

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](https://github.com/UtakataKyosui/GearMesh/blob/main/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://github.com/UtakataKyosui/GearMesh/blob/main/LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](https://github.com/UtakataKyosui/GearMesh/blob/main/CONTRIBUTING.md) for details.
