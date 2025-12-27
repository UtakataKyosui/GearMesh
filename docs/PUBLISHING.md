# Publishing to crates.io

This document describes how to publish gear-mesh to crates.io.

## Prerequisites

1. **crates.io Account**: Create an account at https://crates.io
2. **GitHub Repository**: Ensure the repository is public
3. **Version Bump**: Update version in `Cargo.toml` if needed

## Publishing Methods

### Method 1: Trusted Publishing (Recommended)

Trusted Publishing allows publishing from GitHub Actions without storing API tokens.

#### Setup Steps

1. **Configure crates.io**
   - Go to https://crates.io/settings/tokens
   - Click "New Token" → "Trusted Publishing"
   - Set GitHub repository: `UtakataKyosui/GearMesh`
   - Set workflow: `.github/workflows/release.yaml`
   - Save the token configuration

2. **Create Release Workflow**

Already configured in `.github/workflows/release.yaml`. The workflow will:
- Trigger on version tags (e.g., `v0.1.0`)
- Build and test all crates
- Publish to crates.io in dependency order

3. **Publish a Release**

```bash
# Create and push a version tag
git tag v0.1.0
git push origin v0.1.0
```

The GitHub Actions workflow will automatically publish to crates.io.

### Method 2: Manual Publishing with API Token

If you prefer manual publishing or need to publish outside of CI:

#### Setup

1. **Get API Token**
   - Go to https://crates.io/settings/tokens
   - Create a new token with "publish-update" scope
   - Copy the token

2. **Login**

```bash
cargo login <your-token>
```

#### Publishing Order

**Important**: Publish in dependency order:

```bash
# 1. Core (no dependencies)
cd crates/gear-mesh-core
cargo publish

# Wait for crates.io to index (usually 1-2 minutes)
sleep 120

# 2. Derive (depends on core)
cd ../gear-mesh-derive
cargo publish

sleep 120

# 3. Generator (depends on core)
cd ../gear-mesh-generator
cargo publish

sleep 120

# 4. Main crate (depends on all)
cd ../gear-mesh
cargo publish
```

## Pre-publish Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test --workspace`
- [ ] Clippy is clean: `cargo clippy --workspace --all-targets`
- [ ] Formatting is correct: `cargo fmt --all --check`
- [ ] Documentation builds: `cargo doc --workspace --no-deps`
- [ ] Examples work: Test `examples/simple-bigint` and `examples/axum-react`
- [ ] README is up-to-date
- [ ] CHANGELOG is updated
- [ ] Version numbers are correct in all `Cargo.toml` files

## Dry Run

Test publishing without actually uploading:

```bash
cargo publish --dry-run --package gear-mesh-core
cargo publish --dry-run --package gear-mesh-derive
cargo publish --dry-run --package gear-mesh-generator
cargo publish --dry-run --package gear-mesh
```

## Version Management

gear-mesh uses workspace-level version management. To bump the version:

1. Update `version` in root `Cargo.toml`:
   ```toml
   [workspace.package]
   version = "0.2.0"
   ```

2. Update `CHANGELOG.md`

3. Commit and tag:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to 0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```

## Troubleshooting

### "crate not found" Error

If you get an error about a dependency not being found:
- Wait 1-2 minutes for crates.io to index the previous crate
- Check that the dependency version matches what was just published

### Documentation Not Building

Ensure all public APIs have documentation:
```bash
cargo doc --workspace --no-deps --document-private-items
```

### Yanking a Version

If you need to yank a published version:
```bash
cargo yank --vers 0.1.0 gear-mesh
```

## References

- [Cargo Book: Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Trusted Publishing](https://blog.rust-lang.org/2023/06/23/Trusted-Publishing.html)
- [GitHub Actions for Rust](https://github.com/actions-rs)
