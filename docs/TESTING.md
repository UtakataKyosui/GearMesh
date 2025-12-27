# Testing Documentation

## Overview

GearMesh has comprehensive test coverage across all crates.

## Test Statistics

| Crate | Unit Tests | Integration Tests |
|-------|-----------|-------------------|
| **gear-mesh-core** | ~17 | - |
| **gear-mesh-derive** | 0 | - |
| **gear-mesh-generator** | ~39 | 28 |
| **Total** | **~56** | **28** |

## Running Tests

### All Tests
```bash
cargo test --workspace
```

### Specific Crate
```bash
cargo test --package gear-mesh-core
cargo test --package gear-mesh-generator
```

### With Moonrepo
```bash
moon run :test                    # All crates
moon run gear-mesh-core:test      # Specific crate
```

---

## Code Generation Tests

Located in `crates/gear-mesh-generator/src/tests.rs` and `crates/gear-mesh-generator/tests/`

### Phase 1: Basic Types

#### Primitive Types
- `f64` → `number`
- `String` → `string`
- `bool` → `boolean`
- Integer types → `number`

#### BigInt Types
- `i64/u64` conversion
  - `use_bigint = true`: → `bigint`
  - `use_bigint = false`: → `number`

#### Branded Types
- Single Branded Type
- Multiple Branded Types

#### Collections
- `Option<T>` → `T | null` + `?`
- `Vec<T>` → `T[]`
- `HashMap<String, T>` → `Record<string, T>`

#### Enums
- Unit variants → `"A" | "B" | "C"`
- Tagged Unions (Ok/Err, Struct variants)

#### JSDoc
- Generation enabled/disabled

### Phase 2: Tuple Types

- Basic tuples
- Nested tuples

### Phase 3: Validation Rules

- Range validation
- Email validation
- Validation function generation

### Phase 4: Zod Schema

- Basic schema generation
- Schema with validation
- BigInt validation support in Zod

### Phase 5: Serde Attributes

- `rename_all`
- `rename` field attribute

---

## Unit Tests by Crate

### gear-mesh-core

#### types.rs
- Rename rules
- BigInt type detection
- TypeScript type name conversion

#### docs.rs
- Simple doc comment parsing
- JSDoc conversion

#### validation.rs
- Range validation
- Email validation

### gear-mesh-generator

#### typescript.rs
- Struct generation
- Branded type generation

---

## Integration Tests

Located in `tests/integration_test.rs`

- Complete flow for simple struct
- Complete flow for Branded Type
- Complete flow for enums
- Complete flow with BigInt
- Complete flow with optional fields
- Complete flow with validation

---

## Test Coverage

### Implemented Features

| Feature | Coverage |
|---------|----------|
| Type conversion (basic) | ✅ High |
| Branded Types | ✅ High |
| Doc comment conversion | ✅ High |
| Validation | ✅ High |
| BigInt support | ✅ High |
| Configuration | ✅ High |
| Integration flows | ✅ High |
| Tuple types | ✅ High |
| Serde attributes | ✅ High |
| Zod Schema | ✅ High |

### Untested Areas

- ❌ `gear-mesh-derive` proc-macro (manual testing only)
- ⚠️ Error handling (edge cases)

---

## Next Steps

1. **proc-macro tests**: Use `trybuild` for compile tests
2. **E2E tests**: Real Rust project type generation
3. **Performance tests**: Large type definition benchmarks
