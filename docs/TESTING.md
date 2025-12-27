# Testing Documentation

## Overview

GearMesh has comprehensive test coverage across all crates with 69 total tests (61 passing + 8 ignored for future features).

## Test Statistics

| Crate | Unit Tests | Integration Tests | Total |
|-------|-----------|-------------------|-------|
| **gear-mesh-core** | 17 | - | 17 |
| **gear-mesh-derive** | 0 | - | 0 |
| **gear-mesh-generator** | 11 | 28 | 39 |
| **gear-mesh-cli** | 4 | - | 4 |
| **Total** | **32** | **28** | **60** |

**Additional**: 9 general integration tests

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

### Specific Test
```bash
cargo test --test codegen_test test_branded_type_generation
```

### With Moonrepo
```bash
moon run :test                    # All crates
moon run gear-mesh-core:test      # Specific crate
```

---

## Code Generation Tests (28 tests)

Located in `crates/gear-mesh-generator/tests/codegen_test.rs`

### Phase 1: Basic Types (15 tests)

#### Primitive Types
- `test_simple_struct_with_primitives`: `f64` → `number`
- `test_struct_with_string_types`: `String` → `string`
- `test_struct_with_boolean`: `bool` → `boolean`
- `test_struct_with_integer_types`: `i8/i16/i32/u8/u16/u32` → `number`

#### BigInt Types
- `test_struct_with_bigint_types`: `i64/u64/i128/u128` conversion
  - `use_bigint = true`: → `bigint`
  - `use_bigint = false`: → `number`

#### Branded Types
- `test_branded_type_generation`: Single Branded Type
- `test_multiple_branded_types`: Multiple Branded Types

#### Collections
- `test_optional_fields`: `Option<T>` → `T | null` + `?`
- `test_vec_array_generation`: `Vec<T>` → `T[]`
- `test_hashmap_generation`: `HashMap<String, T>` → `Record<string, T>`

#### Enums
- `test_simple_enum_generation`: Unit variants → `"A" | "B" | "C"`

#### JSDoc
- `test_jsdoc_generation`: JSDoc enabled
- `test_jsdoc_disabled`: JSDoc disabled

#### Complex Types
- `test_nested_types`: Nested structures
- `test_complex_real_world_example`: Comprehensive integration

### Phase 2: Tuple Types (3 tests)

- `test_tuple_2_elements`: `(f64, f64)` → `[number, number]`
- `test_tuple_3_elements`: `(i32, String, bool)` → `[number, string, boolean]`
- `test_nested_tuple`: `((i32, String), bool)` → `[[number, string], boolean]`

### Phase 3: Tagged Unions (4 tests)

- `test_enum_with_tuple_variant`: `Result { Ok(T), Err(E) }`
- `test_enum_with_struct_variant`: `Message { Text { content: String } }`
- `test_enum_mixed_variants`: Mixed Unit/Tuple/Struct variants
- `test_enum_internal_tagged`: `#[serde(tag = "type")]`

### Phase 4: Generic Types (3 tests)

- `test_generic_single_param`: `Container<T>`
- `test_generic_multiple_params`: `Pair<T, U>`
- `test_generic_with_constraints`: Constraints (ignored in TS)

### Phase 5: Validation Rules (5 tests - Ignored)

> **Note**: These tests are marked `#[ignore]` pending full validation generation implementation.

- `test_validation_range`: Range validation
- `test_validation_length`: String length validation
- `test_validation_email`: Email format validation
- `test_validation_url`: URL format validation
- `test_validation_pattern`: Regex pattern validation

### Phase 6: Zod Schema (3 tests - Ignored)

> **Note**: These tests are marked `#[ignore]` pending full Zod schema generation implementation.

- `test_zod_basic_schema`: Basic schema generation
- `test_zod_with_validation`: Schema with validation
- `test_zod_optional_fields`: Optional fields in schema

### Phase 7: Serde Attributes (3 tests)

- `test_serde_rename_field`: `user_id` → `userId`
- `test_serde_rename_mixed`: Mixed renamed/non-renamed fields
- `test_serde_rename_with_optional`: Rename with optional fields

---

## Unit Tests by Crate

### gear-mesh-core (17 tests)

#### types.rs (6 tests)
- Rename rules (CamelCase, SnakeCase)
- BigInt type detection
- TypeScript type name conversion
- TypeRef creation
- Struct type serialization
- Type attributes defaults

#### docs.rs (9 tests)
- Simple doc comment parsing
- Doc with example code
- JSDoc conversion
- Multiline descriptions
- Multiple examples
- Custom sections
- Inline JSDoc (empty/with content)

#### validation.rs (2 tests)
- Range validation
- Email validation

### gear-mesh-generator (11 tests)

#### typescript.rs (2 tests)
- Simple struct generation
- Branded type generation

#### branded.rs (1 test)
- Branded type generation

#### validation_gen.rs (1 test)
- Validation function generation

#### tests.rs (7 tests)
- Enum with data generation
- Nested types generation
- JSDoc disabled generation
- Tuple type generation
- Option type generation
- Empty struct generation
- Branded type without flag

### gear-mesh-cli (4 tests)

#### config.rs (4 tests)
- Default configuration
- GeneratorConfig conversion
- TOML serialization
- TOML deserialization

---

## Integration Tests (9 tests)

Located in `tests/integration_test.rs`

- `test_simple_struct_generation`: Complete flow for simple struct
- `test_branded_type_generation`: Complete flow for Branded Type
- `test_enum_generation`: Complete flow for enum
- `test_bigint_generation`: Complete flow with BigInt
- `test_optional_fields`: Complete flow with optional fields
- `test_validation_generation`: Complete flow with validation
- `test_multiple_types_generation`: Multiple types
- `test_vec_generation`: Vec type
- `test_hashmap_generation`: HashMap type

---

## Test Coverage

### Implemented Features

| Feature | Test Count | Coverage |
|---------|-----------|----------|
| Type conversion (basic) | 15 | ✅ High |
| Branded Types | 4 | ✅ High |
| Doc comment conversion | 9 | ✅ High |
| Validation | 3 | ⚠️ Medium |
| BigInt support | 3 | ✅ High |
| Configuration | 4 | ✅ High |
| Integration flows | 9 | ✅ High |
| Tuple types | 3 | ✅ High |
| Tagged Unions | 4 | ✅ High |
| Generic types | 3 | ✅ High |
| Serde attributes | 3 | ✅ High |

### Untested Areas

- ❌ `gear-mesh-derive` proc-macro (manual testing only)
- ⚠️ CLI `generate` command (actual file generation)
- ⚠️ CLI `watch` command (file watching)
- ⚠️ Error handling (edge cases)

---

## Test Helpers

### `assert_contains(output, expected)`
Verifies that generated code contains expected string.

```rust
assert_contains(&output, "export interface User {");
```

### `assert_not_contains(output, unexpected)`
Verifies that generated code does not contain unexpected string.

```rust
assert_not_contains(&output, "/**");  // When JSDoc disabled
```

---

## Expected Output Examples

### Branded Type
```typescript
type Brand<T, B> = T & { readonly __brand: B };

export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;
```

### Struct
```typescript
export interface User {
    id: number;
    name: string;
    email: string;
}
```

### Optional Fields
```typescript
export interface User {
    id: number;
    email?: string | null;
    age?: number | null;
}
```

### With JSDoc
```typescript
/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: number;
    /** User's display name */
    name: string;
}
```

---

## Next Steps

1. **proc-macro tests**: Use `trybuild` for compile tests
2. **E2E tests**: Real Rust project type generation
3. **Error handling tests**: Invalid input handling
4. **Performance tests**: Large type definition benchmarks
5. **Validation generation**: Complete implementation
6. **Zod schema generation**: Complete implementation
