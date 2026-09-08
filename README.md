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
| **State Slots** | Project Rust-owned state values into the UI, one slot per value |

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
        .with_validation(true)
        .with_option_style(gear_mesh::OptionStyle::Nullable)
        .with_result_style(gear_mesh::ResultStyle::TaggedUnion);

    gear_mesh::generate_with_config("generated", config)
        .expect("Failed to generate");
}
```

## Validation

gear-mesh supports automatic generation of Zod schemas with validation rules from Rust attributes.

### Available Validation Rules

| Validation | Rust Attribute | Generated Zod | Description |
|------------|----------------|---------------|-------------|
| **Range** | `#[validate(range(min = 0, max = 100))]` | `.min(0).max(100)` | Numeric range validation |
| **Length** | `#[validate(length(min = 1, max = 20))]` | `.min(1).max(20)` | String length validation |
| **Email** | `#[validate(email)]` | `.email()` | Email format validation |
| **URL** | `#[validate(url)]` | `.url()` | URL format validation |
| **Pattern** | `#[validate(pattern = "^[A-Z]")]` | `.regex(/^[A-Z]/)` | Regex pattern matching |

### Usage Example

```rust
use gear_mesh::GearMesh;

#[derive(GearMesh)]
struct User {
    /// User's display name (1-20 characters)
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    
    /// User's email address
    #[validate(email)]
    pub email: String,
    
    /// User's age (1-150)
    #[validate(range(min = 1, max = 150))]
    pub age: i32,
    
    /// User's website (nullable)
    #[validate(url)]
    pub website: Option<String>,
}
```

### Generated Zod Schema

```typescript
import { z } from 'zod';

export interface User {
    /** User's display name (1-20 characters) */
    name: string;
    /** User's email address */
    email: string;
    /** User's age (1-150) */
    age: number;
    /** User's website (nullable) */
    website: string | null;
}

export const UserSchema = z.object({
    name: z.string().min(1).max(20),
    email: z.string().email(),
    age: z.number().min(1).max(150),
    website: z.string().url().nullable(),
});
```

### BigInt Validation

When using `use_bigint` configuration, range validations automatically use BigInt literals:

```rust
#[derive(GearMesh)]
struct Transaction {
    #[validate(range(min = 0, max = 1000000000))]
    pub amount: u64,  // Automatically becomes bigint in TypeScript
}
```

Generated TypeScript:

```typescript
export const TransactionSchema = z.object({
    amount: z.bigint().min(0n).max(1000000000n),
});
```

## State Slots

Types are not the only knowledge shared across the boundary. A state value that
Rust owns also needs a way to be read and a way to announce that it changed —
command and event names that are otherwise written by hand on both sides, with
no type checking to catch a rename.

Declare the slots on the struct that holds the state, and the names come from
one key:

```rust
use gear_mesh::{GearMesh, GearMeshState};

#[derive(GearMesh)]
enum Theme { Light, Dark, System }

#[derive(GearMeshState)]
struct AppState {
    /// The theme the user selected.
    #[state]
    theme: Theme,

    /// Not projected: no `#[state]`, no slot.
    listeners: usize,
}
```

```rust
gear_mesh::generate_state("../frontend/src/types/state.ts", Some("./index"))
    .expect("Failed to generate state slots");
```

The generated projection holds no value of its own — it reads through to Rust:

```typescript
export const theme: ReadonlyStateSlot<Theme> = readonlySlot<Theme>(
    "theme",
    "state://theme/get",
    "state://theme/changed",
);
```

```typescript
setStateTransport({
  read: (command) => invoke(command),
  subscribe: (event, handler) => listen(event, (e) => handler(e.payload as never)),
});

// Reads the current value, follows changes, and drops anything older than the
// newest revision already delivered — so a slow initial read can never
// overwrite a change that arrived first.
const stop = await theme.watch((value) => {
  document.documentElement.dataset["theme"] = value;
});
```

Slots are read-only: the UI projects the value, it never keeps a second
authoritative copy of it. Writable slots are the next phase. See
[docs/STATE.md](docs/STATE.md) for the design and the rationale.

## Comparison with Existing Crates


| Feature | ts-rs | typeshare | specta | **gear-mesh** |
|---------|-------|-----------|--------|---------------|
| Basic type conversion | ✅ | ✅ | ✅ | ✅ |
| Branded Types | ❌ | ❌ | ❌ | ✅ |
| Doc comment conversion | ❌ | ❌ | ❌ | ✅ |
| Zod Schema | ❌ | ❌ | ❌ | ✅ |
| Validation embedding | ❌ | ❌ | ❌ | ✅ |
| Auto BigInt | Manual | Manual | Manual | ✅ Auto |
| State slots | ❌ | ❌ | ❌ | ✅ |

## Crate Structure

- `gear-mesh` - Main crate with re-exports
- `gear-mesh-core` - Intermediate representation (IR)
- `gear-mesh-derive` - `#[derive(GearMesh)]` and `#[derive(GearMeshState)]` proc-macros
- `gear-mesh-generator` - TypeScript code generator

## Implementation Status

gear-mesh v0.1.0 implements:

- ✅ Basic type conversion
- ✅ Branded Type generation
- ✅ Doc comment conversion
- ✅ BigInt support
- ✅ Zod Schema generation
- ✅ Validation rules
- ✅ State slots (read-only projection)

See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) for detailed status and [docs/FUTURE_ISSUES.md](docs/FUTURE_ISSUES.md) for planned features.

## Testing

gear-mesh has comprehensive test coverage updated regularly.

- **Unit tests** covering core logic, generator, and derive macros.
- **Integration tests** validation the full pipeline from Rust types to TypeScript output.
- **E2E tests** generating TypeScript from a real Rust derive fixture and checking a TypeScript consumer. Docker is optional.

Run all tests:

```bash
npm ci --ignore-scripts
npm test
```

Or using [moonrepo](https://moonrepo.dev/):

```bash
moon run :verify
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for tool versions and
[docs/TESTING.md](docs/TESTING.md) for focused checks and snapshot updates.

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
