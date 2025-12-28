# gear-mesh-generator

TypeScript code generator for gear-mesh.

This crate converts Rust type definitions (represented as `GearMeshType`) into TypeScript code, including interfaces, Zod schemas, and Branded Types.

## Overview

`gear-mesh-generator` provides:

- **TypeScript Interface Generation**: Convert Rust structs/enums to TypeScript interfaces
- **Zod Schema Generation**: Generate runtime validation schemas
- **Branded Type Support**: Create type-safe wrappers
- **BigInt Handling**: Automatic `bigint` for large integer types
- **Documentation Preservation**: Convert Rust doc comments to JSDoc

## Usage

This crate is typically used through the main `gear-mesh` crate, but can be used directly:

```rust
use gear_mesh_generator::{GeneratorConfig, TypeScriptGenerator};
use gear_mesh_core::GearMeshType;

let config = GeneratorConfig::new()
    .with_bigint(true)
    .with_zod(true)
    .with_validation(true);

let generator = TypeScriptGenerator::new(config);
let typescript_code = generator.generate(&my_type);
```

## Configuration Options

```rust
GeneratorConfig::new()
    .with_bigint(true)        // Use bigint for u64/i64
    .with_zod(true)           // Generate Zod schemas
    .with_validation(true)    // Include validation rules
    .with_branded(true)       // Generate Branded Types
    .with_jsdoc(true)         // Include JSDoc comments
```

## Generated Output

### TypeScript Interface

```typescript
export interface User {
    id: number;
    name: string;
    email: string;
}
```

### Zod Schema

```typescript
export const UserSchema = z.object({
    id: z.number(),
    name: z.string().min(1).max(20),
    email: z.string().email(),
});
```

### Branded Type

```typescript
type Brand<T, B> = T & { readonly __brand: B };

export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;
```

## Features

- **Type Safety**: Generates type-safe TypeScript code
- **Validation**: Zod schemas with validation rules
- **Documentation**: Preserves doc comments as JSDoc
- **Customizable**: Flexible configuration options
- **BigInt Support**: Automatic handling of large integers

## Architecture

The generator consists of several modules:

- `typescript.rs` - TypeScript interface generation
- `validation_gen.rs` - Zod schema generation
- `branded.rs` - Branded Type generation
- `utils.rs` - Utility functions for type checking

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
