# gear-mesh Project Structure

## Directory Layout

```
gear-mesh/
├── crates/                      # Workspace members
│   ├── gear-mesh/              # Main crate (re-exports)
│   │   ├── src/
│   │   │   └── lib.rs
│   │   ├── tests/
│   │   │   └── integration_test.rs  # Integration tests
│   │   └── examples/
│   │       └── basic.rs        # Basic usage example
│   │
│   ├── gear-mesh-core/         # Core IR and types
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs        # Type definitions
│   │   │   ├── validation.rs   # Validation rules
│   │   │   ├── docs.rs         # Doc comment handling
│   │   │   └── docs_tests.rs   # Additional doc tests
│   │   └── Cargo.toml
│   │
│   ├── gear-mesh-derive/       # Proc-macro
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── parser.rs       # Type parsing
│   │   │   └── attributes.rs   # Attribute parsing
│   │   └── Cargo.toml
│   │
│   ├── gear-mesh-generator/    # TypeScript generator
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── typescript.rs   # Main generator
│   │   │   ├── branded.rs      # Branded type generation
│   │   │   ├── validation_gen.rs  # Validation generation
│   │   │   └── tests.rs        # Generator tests
│   │   └── Cargo.toml
│   │
│   └── gear-mesh-cli/          # CLI tool
│       ├── src/
│       │   ├── main.rs
│       │   ├── config.rs       # Configuration handling
│       │   ├── config_tests.rs # Config tests
│       │   ├── generate.rs     # Generate command
│       │   └── watch.rs        # Watch command
│       └── Cargo.toml
│
├── tests/                      # E2E tests
│   └── e2e/
│       ├── Dockerfile.test     # Docker test environment
│       ├── test-e2e-simple.sh  # E2E test script
│       └── run-docker-test.sh  # Docker test runner
│
├── docs/                       # Documentation
│   ├── TESTING.md             # Test coverage details
│   └── E2E_TEST_RESULTS.md    # E2E test results
│
├── Cargo.toml                 # Workspace configuration
├── Cargo.lock                 # Dependency lock file
├── README.md                  # Project README
└── .gitignore                 # Git ignore rules

```

## Crate Responsibilities

### gear-mesh
**Main entry point** - Re-exports all public APIs from other crates.
- Provides unified interface
- Contains integration tests
- Includes usage examples

### gear-mesh-core
**Core types and IR** - Language-agnostic intermediate representation.
- `GearMeshType`: Main IR type
- `ValidationRule`: Validation definitions
- `DocComment`: Documentation handling
- Type conversion utilities

### gear-mesh-derive
**Proc-macro** - `#[derive(GearMesh)]` implementation.
- Parses Rust types using `syn`
- Extracts attributes (`#[gear_mesh(...)]`, `#[validate(...)]`)
- Converts to IR

### gear-mesh-generator
**Code generation** - Generates TypeScript from IR.
- TypeScript type generation
- Branded type generation
- Validation function generation
- JSDoc comment generation

### gear-mesh-cli
**Command-line tool** - User-facing CLI.
- `generate`: Generate TypeScript definitions
- `watch`: Watch mode for auto-regeneration
- `init`: Initialize configuration file
- Configuration management

## Test Organization

### Unit Tests
- Located in each crate's `src/` directory
- Test individual functions and modules
- **Total: 32 tests**

### Integration Tests
- Located in `crates/gear-mesh/tests/`
- Test complete workflows
- **Total: 9 tests**

### E2E Tests
- Located in `tests/e2e/`
- Docker-based end-to-end testing
- Tests real-world usage scenarios

## Documentation

### User Documentation
- `README.md`: Quick start and overview
- `docs/TESTING.md`: Test coverage details
- `docs/E2E_TEST_RESULTS.md`: E2E test results

### Code Documentation
- Inline doc comments in source files
- Examples in `crates/gear-mesh/examples/`

## Build Artifacts

Generated during build (not in version control):
- `target/`: Compiled binaries and libraries
- `Cargo.lock`: Dependency versions (committed for applications)
- `bindings/`: Generated TypeScript files (example output)

## Configuration Files

- `Cargo.toml`: Workspace and package configuration
- `gear-mesh.toml`: User configuration for TypeScript generation
- `.gitignore`: Git ignore patterns
