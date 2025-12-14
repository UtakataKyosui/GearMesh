# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-12-14

### Added

#### Core Features
- **Intermediate Representation (IR)**: Language-agnostic type representation
  - `GearMeshType` for representing Rust types
  - Support for structs, enums, newtypes, primitives
  - Generic type support
  - Documentation comment handling

- **Proc-Macro**: `#[derive(GearMesh)]` for automatic type conversion
  - Attribute parsing (`#[gear_mesh(...)]`, `#[validate(...)]`)
  - Support for branded types
  - BigInt auto-detection
  - Serde attribute integration

- **TypeScript Generator**: Convert IR to TypeScript code
  - Interface generation for structs
  - Union type generation for enums
  - Branded type generation with helper functions
  - BigInt support (u64, i64 → bigint)
  - JSDoc comment conversion
  - Optional field handling (Option<T> → T | null)
  - Collection support (Vec<T> → T[], HashMap → Record)

- **CLI Tool**: Command-line interface for type generation
  - `generate` command: Generate TypeScript definitions
  - `watch` command: Auto-regenerate on file changes
  - `init` command: Initialize configuration file
  - TOML-based configuration

#### Testing
- **41 total tests** across all crates
  - 17 unit tests in `gear-mesh-core`
  - 11 unit tests in `gear-mesh-generator`
  - 4 unit tests in `gear-mesh-cli`
  - 9 integration tests
- **E2E tests** with Docker support
- Comprehensive test coverage for core features

#### Documentation
- README with quick start guide
- Detailed test coverage documentation
- E2E test results
- Project structure documentation
- Contributing guidelines
- Usage examples

### Features in Detail

#### Branded Types
```rust
#[derive(GearMesh)]
#[gear_mesh(branded)]
struct UserId(i32);
```
Generates:
```typescript
type Brand<T, B> = T & { readonly __brand: B };
export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;
```

#### Doc Comment Conversion
```rust
/// User information
///
/// This struct represents a user in the system.
struct User {
    /// User's unique identifier
    id: UserId,
}
```
Generates:
```typescript
/**
 * User information
 *
 * This struct represents a user in the system.
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
}
```

#### BigInt Support
```rust
struct Product {
    price: u64,  // Automatically converted to bigint
}
```
Generates:
```typescript
export interface Product {
    price: bigint;
}
```

### Project Structure
- `gear-mesh`: Main crate with re-exports
- `gear-mesh-core`: Core IR and type definitions
- `gear-mesh-derive`: Proc-macro implementation
- `gear-mesh-generator`: TypeScript code generator
- `gear-mesh-cli`: Command-line tool

### Known Limitations
- Proc-macro requires manual type registration (planned for future release)
- CLI `generate` command requires source file parsing (planned)
- Validation generation is basic (will be enhanced)

[Unreleased]: https://github.com/yourusername/gear-mesh/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/gear-mesh/releases/tag/v0.1.0
