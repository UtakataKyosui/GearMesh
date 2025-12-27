# Architecture

## Overview

GearMesh is a Rust to TypeScript type definition sharing library with a layered architecture.

## Crate Structure

```
GearMesh/
├── crates/
│   ├── gear-mesh-core          # Core types and IR
│   ├── gear-mesh-derive        # Proc-macro
│   └── gear-mesh-generator     # Generator + Facade
```

### Dependency Graph

```
gear-mesh-generator (+ facade)
    ↓
    ├─→ gear-mesh-core
    └─→ gear-mesh-derive
            ↓
        gear-mesh-core
```

---

## Crate Responsibilities

### gear-mesh-core
**Core types and intermediate representation**

- `GearMeshType`: Main IR type
- `ValidationRule`: Validation definitions
- `DocComment`: Documentation handling
- Type conversion utilities

**Key modules**:
- `types.rs`: Type definitions and IR
- `validation.rs`: Validation rules
- `docs.rs`: Doc comment parsing and conversion

### gear-mesh-derive
**Procedural macro**

- `#[derive(GearMesh)]` implementation
- Parses Rust types using `syn`
- Extracts attributes (`#[gear_mesh(...)]`, `#[validate(...)]`)
- Converts to IR

**Key modules**:
- `lib.rs`: Macro entry point
- `parser.rs`: Type parsing
- `attributes.rs`: Attribute parsing

**Note**: Must remain separate due to Rust proc-macro constraints (proc-macro crates can only export macro functions).

### gear-mesh-generator
**Code generation and main API**

- TypeScript type generation
- Branded type generation
- Validation function generation
- Zod schema generation
- JSDoc comment generation
- **Facade**: Re-exports all core types and derive macro

**Key modules**:
- `typescript.rs`: Main generator
- `branded.rs`: Branded type generation
- `validation_gen.rs`: Validation generation
- `lib.rs`: Facade re-exports

---

## Data Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Rust Type + #[derive(GearMesh)]                          │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. gear-mesh-derive (Proc-macro)                            │
│    - Parse DeriveInput                                       │
│    - Extract attributes                                      │
│    - Generate GearMeshType                                   │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. gear-mesh-core (IR)                                      │
│    - GearMeshType (JSON serializable)                       │
│    - Language-agnostic type information                     │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. gear-mesh-generator (Code Generation)                    │
│    - TypeScriptGenerator                                    │
│    - BrandedTypeGenerator                                   │
│    - ValidationGenerator                                    │
└─────────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. TypeScript Type Definitions (.ts)                        │
│    - interface/type definitions                             │
│    - JSDoc comments                                          │
│    - Branded Types                                           │
│    - Validation functions / Zod Schemas                      │
└─────────────────────────────────────────────────────────────┘
```

---

## Design Principles

### 1. Separation of Concerns
- Each crate has a single, clear responsibility
- Dependencies flow in one direction (lower → upper layers)

### 2. Language-Agnostic IR
- Core IR is independent of both Rust and TypeScript
- Enables future support for other target languages

### 3. Extensibility
- Easy to add new types (`TypeKind` variants)
- Easy to add new generators (new modules)
- Configuration-driven feature toggles

### 4. Type Safety
- Leverages Rust's type system
- Compile-time checks via proc-macro
- Detailed error reporting with `syn::Error`

---

## Type Mapping

| Rust Type | TypeScript Type | Notes |
|-----------|-----------------|-------|
| `i8/i16/i32/u8/u16/u32` | `number` | |
| `i64/u64/i128/u128` | `bigint` | Configurable |
| `f32/f64` | `number` | |
| `String/&str` | `string` | |
| `bool` | `boolean` | |
| `()` | `void` | |
| `Option<T>` | `T \| null` + `?` | Optional marker |
| `Vec<T>` | `T[]` | |
| `HashMap<String, T>` | `Record<string, T>` | |
| `(T, U)` | `[T, U]` | Tuple |
| `enum` (unit) | `"A" \| "B"` | Union type |
| `enum` (data) | Tagged union | |
| `struct` (newtype) | Branded Type | With `#[gear_mesh(branded)]` |

---

## Implementation Status

### ✅ Fully Implemented (v0.1.0)

- Basic type conversion
- Branded Type generation
- Doc comment conversion
- BigInt support
- Optional fields
- Collections (Vec, HashMap)
- Enums (unit variants)
- Tuple types
- Tagged Unions (enum with data)
- Generic types
- Serde rename attributes
- JSDoc generation
- Zod schema generation
- Validation rules

### 📝 Future Work

1. **Advanced Validation** (v0.2.0)
   - Custom validation rules
   - Complex cross-field validation

2. **Plugin System** (v0.3.0)
   - Plugin API design
   - Plugin loader
   - Custom transformers

3. **IDE Integration** (v1.0.0)
   - VS Code extension
   - Language Server Protocol
   - Real-time type checking

---

## Configuration

### GeneratorConfig

```rust
pub struct GeneratorConfig {
    pub use_bigint: bool,           // i64/u64 → bigint
    pub generate_branded: bool,     // Branded Type generation
    pub generate_validation: bool,  // Validation functions
    pub generate_zod: bool,         // Zod schemas
    pub generate_jsdoc: bool,       // JSDoc comments
    pub indent: String,             // Indentation
}
```

---

## Testing Strategy

### Unit Tests
- Test individual functions and modules
- Located in each crate's `src/` directory

### Integration Tests
- Test complete workflows
- Located in `tests/` directories and `crates/gear-mesh-generator/src/tests.rs`

### E2E Tests
- Docker-based end-to-end testing
- Real-world usage scenarios

See [TESTING.md](TESTING.md) for details.

---

## Performance Considerations

### Caching
- Moonrepo provides intelligent task caching
- 7-day cache lifetime
- Per-crate cache keys

### Parallel Execution
- GitHub Actions matrix strategy
- Faster CI feedback

### Build Times
- Rust compilation: ~5s
- TypeScript generation: <1s
- TypeScript validation: ~2s

---

## Future Architecture

### Planned Improvements

1. **Multi-language Support**
   - Abstract generator interface
   - Language-specific generators
   - Shared IR

2. **Incremental Generation**
   - Only regenerate changed types
   - Dependency tracking
   - Faster watch mode

3. **Bidirectional Sync** (Experimental)
   - TypeScript → Rust type conversion
   - Conflict resolution
   - Merge strategies

---

## References

- [TESTING.md](TESTING.md): Test documentation
- [MOONREPO.md](MOONREPO.md): Moonrepo integration
- [README.md](../README.md): User guide
- [CHANGELOG.md](../CHANGELOG.md): Version history
