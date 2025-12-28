# gear-mesh-core

Core types and intermediate representation for the gear-mesh type conversion system.

This crate provides the fundamental data structures used to represent Rust types in a language-agnostic format, which can then be converted to TypeScript or other target languages.

## Overview

`gear-mesh-core` defines:

- **Type Representation**: `GearMeshType`, `TypeKind`, `TypeRef` for representing Rust types
- **Validation Rules**: `ValidationRule` for runtime validation
- **Documentation**: `DocComment` for preserving doc comments
- **Attributes**: Type attributes like `branded`, `validate`, etc.

## Usage

This crate is typically used as a dependency by:
- `gear-mesh-derive` - The proc-macro that parses Rust types
- `gear-mesh-generator` - The code generator that produces TypeScript

Most users should use the main `gear-mesh` crate instead of depending on this directly.

## Example

```rust
use gear_mesh_core::{GearMeshType, TypeKind, PrimitiveType};

// Manually construct a type representation
let user_type = GearMeshType {
    name: "User".to_string(),
    kind: TypeKind::Struct(/* ... */),
    docs: None,
    generics: vec![],
    attributes: Default::default(),
};
```

## Features

- Serializable type representation using `serde`
- Support for complex Rust types (structs, enums, generics)
- Validation rule definitions
- Documentation preservation

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
