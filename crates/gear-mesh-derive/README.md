# gear-mesh-derive

Procedural macro for deriving `GearMesh` trait on Rust types.

This crate provides the `#[derive(GearMesh)]` macro that automatically implements type conversion for Rust structs and enums.

## Overview

The derive macro analyzes your Rust types and generates the necessary metadata for TypeScript code generation, including:

- Type structure and fields
- Generic parameters
- Documentation comments
- Validation rules
- Serde attributes

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gear-mesh = "0.1"
```

Then derive `GearMesh` on your types:

```rust
use gear_mesh::GearMesh;

#[derive(GearMesh)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}
```

## Attributes

### Type-level Attributes

- `#[gear_mesh(branded)]` - Generate as a Branded Type
- `#[gear_mesh(validate)]` - Enable validation
- `#[gear_mesh(bigint = "auto")]` - Auto-detect BigInt types

### Field-level Attributes

- `#[validate(range(min = 0, max = 100))]` - Numeric range validation
- `#[validate(length(min = 1, max = 20))]` - String length validation
- `#[validate(email)]` - Email format validation
- `#[validate(url)]` - URL format validation
- `#[validate(pattern = "regex")]` - Regex pattern validation

## Example

```rust
use gear_mesh::GearMesh;

/// User ID (Branded Type)
#[derive(GearMesh)]
#[gear_mesh(branded)]
pub struct UserId(pub i32);

/// User information
#[derive(GearMesh)]
pub struct User {
    /// User's unique identifier
    pub id: UserId,
    
    /// User's display name
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    
    /// User's email address
    #[validate(email)]
    pub email: String,
    
    /// User's age
    #[validate(range(min = 0, max = 150))]
    pub age: Option<i32>,
}
```

## Implementation Details

The macro:
1. Parses the Rust type using `syn`
2. Extracts type information, docs, and attributes
3. Converts to `GearMeshType` intermediate representation
4. Serializes to JSON for runtime access
5. Implements the `GearMeshExport` trait
6. Registers the type with `inventory` for automatic collection

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
