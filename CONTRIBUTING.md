# Contributing to gear-mesh

Thank you for your interest in contributing to gear-mesh! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Node.js 18+ and npm (for TypeScript validation)
- Docker (optional, for E2E tests)

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/gear-mesh.git
cd gear-mesh
```

2. Build the project:
```bash
cargo build --workspace
```

3. Run tests:
```bash
cargo test --workspace
```

## Project Structure

See [docs/PROJECT_STRUCTURE.md](docs/PROJECT_STRUCTURE.md) for detailed information about the project layout.

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

- Write clear, concise commit messages
- Add tests for new functionality
- Update documentation as needed

### 3. Run Tests

Before submitting a PR, ensure all tests pass:

```bash
# Unit and integration tests
cargo test --workspace

# E2E tests (optional but recommended)
./tests/e2e/run-docker-test.sh
```

### 4. Format Code

```bash
cargo fmt --all
cargo clippy --all-targets --all-features
```

### 5. Submit Pull Request

- Provide a clear description of the changes
- Reference any related issues
- Ensure CI passes

## Testing Guidelines

### Unit Tests

- Place unit tests in the same file as the code being tested
- Use `#[cfg(test)]` module
- Test edge cases and error conditions

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests

- Place integration tests in `crates/gear-mesh/tests/`
- Test complete workflows
- Use realistic scenarios

### E2E Tests

- Modify `tests/e2e/test-e2e-simple.sh` for new scenarios
- Ensure Docker tests pass
- Document any new test cases

## Code Style

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Address all `cargo clippy` warnings
- Write doc comments for public APIs

Example:
```rust
/// Generates TypeScript code from a GearMeshType.
///
/// # Arguments
///
/// * `types` - A slice of GearMeshType to generate code for
///
/// # Returns
///
/// A String containing the generated TypeScript code
pub fn generate(&mut self, types: &[GearMeshType]) -> String {
    // Implementation
}
```

### TypeScript (Generated Code)

- Follow TypeScript best practices
- Ensure generated code is valid and type-safe
- Include JSDoc comments when configured

## Adding New Features

### 1. Core Types (gear-mesh-core)

If adding new type support:
1. Update `TypeKind` enum in `types.rs`
2. Add serialization/deserialization support
3. Add unit tests

### 2. Proc-Macro (gear-mesh-derive)

If modifying the derive macro:
1. Update parser in `parser.rs`
2. Handle new attributes in `attributes.rs`
3. Test with various Rust types

### 3. Generator (gear-mesh-generator)

If adding TypeScript generation features:
1. Update `typescript.rs` or create new module
2. Add configuration options if needed
3. Add tests in `tests.rs`

### 4. CLI (gear-mesh-cli)

If adding CLI features:
1. Update command definitions in `main.rs`
2. Add configuration options in `config.rs`
3. Update `gear-mesh.toml` schema

## Documentation

### Code Documentation

- Write doc comments for all public APIs
- Include examples in doc comments
- Keep documentation up-to-date with code changes

### User Documentation

- Update `README.md` for user-facing changes
- Add examples to `crates/gear-mesh/examples/`
- Update relevant docs in `docs/`

## Release Process

(For maintainers)

1. Update version in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Run full test suite
4. Create git tag
5. Publish to crates.io

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in GitHub Discussions
- Check existing issues and PRs

## Code of Conduct

Be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive community.

## License

By contributing to gear-mesh, you agree that your contributions will be licensed under the same license as the project (MIT/Apache-2.0).
