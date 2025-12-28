# Missing Features GitHub Issues

This file contains drafts for missing features to be registered as GitHub Issues.

---

## Phase 1: 基盤強化 (v0.2.0)

### Issue 1: Crate構造の最適化 - `utils` の `core` への移動

**Title**: `[REFACTOR] Move utility functions to gear-mesh-core`

**Labels**: `refactor`, `v0.2.0`, `code-quality`

**Description**:
```markdown
## Problem
Currently, `utils.rs` is located in `gear-mesh-generator`, but functions like `is_builtin_type()` and `is_bigint_type()` could be useful in other crates, particularly `gear-mesh-derive`.

## Current Location
- `crates/gear-mesh-generator/src/utils.rs`

## Proposed Changes
1. Create `crates/gear-mesh-core/src/type_utils.rs`
2. Move the following functions:
   - `is_builtin_type(type_name: &str) -> bool`
   - `is_bigint_type(type_name: &str) -> bool`
3. Add new utility function:
   - `to_typescript_primitive(type_name: &str, use_bigint: bool) -> Option<&'static str>`
4. Update imports in `gear-mesh-generator`

## Benefits
- Eliminates code duplication
- Centralizes type judgment logic
- Makes utilities available to `derive` macro
- Improves code organization

## Files to Modify
- `crates/gear-mesh-core/src/lib.rs`
- `crates/gear-mesh-core/src/type_utils.rs` (new)
- `crates/gear-mesh-generator/src/utils.rs` (remove or deprecate)
- `crates/gear-mesh-generator/src/typescript.rs`
- `crates/gear-mesh-generator/src/validation_gen.rs`

## Priority
High (Foundation for other improvements)
```

---

### Issue 2: エラーメッセージの改善

**Title**: `[ENHANCEMENT] Improve error messages in derive macro`

**Labels**: `enhancement`, `v0.2.0`, `dx`, `error-handling`

**Description**:
```markdown
## Problem
Current error messages from the `#[derive(GearMesh)]` macro are generic and don't provide enough context or actionable suggestions.

## Current Behavior
```rust
// Generic error
Err(syn::Error::new(span, "Invalid type"))
```

## Proposed Solution
Create a structured error system with detailed, actionable error messages.

### Implementation
1. Create `crates/gear-mesh-derive/src/error.rs`
2. Define `GearMeshError` struct with `ErrorKind` enum
3. Provide context-aware error messages with suggestions

### Error Categories
- `UnsupportedType`: Type not supported by GearMesh
- `InvalidAttribute`: Invalid `#[gear_mesh(...)]` attribute
- `ValidationError`: Invalid validation rule

### Example Output
```
error: Unsupported type: `MyCustomType`
  --> src/main.rs:10:5
   |
10 |     field: MyCustomType,
   |            ^^^^^^^^^^^^
   |
   = help: Consider deriving GearMesh for MyCustomType first
```

## Benefits
- Better developer experience
- Faster debugging
- Clearer guidance for users

## Priority
High (Significantly improves DX)
```

---

### Issue 3: スナップショットテストの導入

**Title**: `[TEST] Add snapshot testing for code generation`

**Labels**: `testing`, `v0.2.0`, `quality`

**Description**:
```markdown
## Motivation
Currently, testing generated TypeScript code requires manual assertion of strings. Snapshot testing would make it easier to:
- Review changes in generated code
- Catch unintended regressions
- Visualize diffs during code review

## Proposed Solution
Use `insta` crate for snapshot testing.

### Implementation
1. Add `insta` to dev-dependencies
2. Create snapshot tests in `crates/gear-mesh-generator/src/tests.rs`
3. Add snapshots for:
   - Simple structs
   - Enums (all representations)
   - Branded types
   - Generic types
   - Validation rules
   - Zod schemas

### Example Test
```rust
use insta::assert_snapshot;

#[test]
fn test_user_type_generation() {
    let ty = create_user_type();
    let mut generator = TypeScriptGenerator::new(GeneratorConfig::default());
    let output = generator.generate(&[ty]);
    
    assert_snapshot!(output);
}
```

## Benefits
- Visual diff review
- Easier regression testing
- Better test maintainability

## Dependencies
- `insta = "1.34"`

## Priority
Medium
```

---

## Phase 2: 拡張性 (v0.3.0)

### Issue 4: プラグインシステムの実装

**Title**: `[FEATURE] Plugin system for custom type transformations`

**Labels**: `enhancement`, `v0.3.0`, `plugin-system`, `extensibility`

**Description**:
```markdown
## Feature Description
Implement a plugin system that allows users to customize type transformations for custom types (e.g., `DateTime`, `Decimal`, `Uuid`).

## Motivation
Users need to handle custom types that require special handling in TypeScript. Currently, extending GearMesh requires modifying core code.

## Proposed Solution

### Plugin Trait
```rust
// gear-mesh-core/src/plugin.rs
pub trait TypeTransformer: Send + Sync {
    fn can_handle(&self, type_name: &str) -> bool;
    fn transform_type(&self, type_ref: &TypeRef) -> Option<String>;
    fn transform_zod(&self, type_ref: &TypeRef) -> Option<String>;
    fn required_imports(&self) -> Vec<String>;
}
```

### Example Plugin
```rust
pub struct DateTimeTransformer;

impl TypeTransformer for DateTimeTransformer {
    fn can_handle(&self, type_name: &str) -> bool {
        matches!(type_name, "DateTime" | "NaiveDateTime")
    }
    
    fn transform_type(&self, _: &TypeRef) -> Option<String> {
        Some("Date".to_string())
    }
    
    fn transform_zod(&self, _: &TypeRef) -> Option<String> {
        Some("z.date()".to_string())
    }
    
    fn required_imports(&self) -> Vec<String> {
        vec![]
    }
}
```

### Usage
```rust
let config = GeneratorConfig::new()
    .with_transformer(Box::new(DateTimeTransformer));
```

## Implementation Steps
1. Define `TypeTransformer` trait in `gear-mesh-core`
2. Add `transformers: Vec<Box<dyn TypeTransformer>>` to `GeneratorConfig`
3. Update `TypeScriptGenerator` to check transformers before default handling
4. Add `TypeKind::Custom` variant for plugin-handled types

## Benefits
- Extensible without modifying core
- User-defined type conversions
- Easy integration with third-party crates

## Priority
High (Enables ecosystem growth)
```

---

### Issue 5: 高度なバリデーション機能

**Title**: `[FEATURE] Advanced validation with cross-field and custom rules`

**Labels**: `enhancement`, `v0.3.0`, `validation`

**Description**:
```markdown
## Feature Description
Extend validation system to support:
1. Cross-field validation (e.g., password confirmation)
2. Custom validation functions
3. Conditional validation
4. Custom error messages

## Current Limitations
- ✅ Basic rules (Range, Length, Email, URL, Pattern)
- ❌ Cross-field validation
- ❌ Custom validation functions
- ❌ Error message customization

## Proposed Extensions

### New ValidationRule Variants
```rust
pub enum ValidationRule {
    // Existing...
    
    Custom {
        function_name: String,
        error_message: Option<String>,
    },
    
    CrossField {
        fields: Vec<String>,
        rule: CrossFieldRule,
    },
    
    Conditional {
        condition: String,
        rule: Box<ValidationRule>,
    },
}

pub enum CrossFieldRule {
    Match,
    AtLeastOne,
    MutuallyExclusive,
}
```

### Example Usage
```rust
#[derive(GearMesh)]
struct UserRegistration {
    #[validate(length(min = 8))]
    password: String,
    
    #[validate(cross_field(match = "password"))]
    password_confirmation: String,
    
    #[validate(custom = "validateUsername", message = "Username taken")]
    username: String,
}
```

### Generated Zod Schema
```typescript
export const UserRegistrationSchema = z.object({
    password: z.string().min(8),
    password_confirmation: z.string(),
    username: z.string(),
}).refine(
    (data) => data.password === data.password_confirmation,
    { message: "Passwords must match", path: ["password_confirmation"] }
).refine(
    (data) => validateUsername(data.username),
    { message: "Username taken", path: ["username"] }
);
```

## Implementation Steps
1. Extend `ValidationRule` enum in `gear-mesh-core`
2. Update attribute parser in `gear-mesh-derive`
3. Implement Zod schema generation for new rules
4. Add comprehensive tests

## Priority
Medium
```

---

### Issue 6: モジュール分割とimport管理

**Title**: `[FEATURE] Module organization and automatic import generation`

**Labels**: `enhancement`, `v0.3.0`, `code-generation`

**Description**:
```markdown
## Problem
Currently, all types are generated into a single file, which:
- Becomes unwieldy for large projects
- Doesn't match typical TypeScript project structure
- Requires manual import management

## Proposed Solution
Implement automatic module organization based on type dependencies.

### Module Organizer
```rust
// gear-mesh-generator/src/module_organizer.rs
pub struct ModuleOrganizer {
    dependency_graph: HashMap<String, Vec<String>>,
}

impl ModuleOrganizer {
    pub fn organize(&self, types: &[GearMeshType]) 
        -> HashMap<String, Vec<GearMeshType>>;
    
    pub fn generate_imports(&self, module: &str, types: &[GearMeshType]) 
        -> Vec<String>;
}
```

### Output Structure
```
generated/
├── models/
│   ├── user.ts
│   ├── user-id.ts
│   └── post.ts
├── index.ts  // Re-exports
└── schemas.ts  // Zod schemas
```

### Example Generated File
```typescript
// models/user.ts
import { UserId } from './user-id';
import { Post } from './post';

export interface User {
    id: UserId;
    posts: Post[];
}
```

## Configuration
```rust
pub struct GeneratorConfig {
    // ...
    pub module_strategy: ModuleStrategy,
}

pub enum ModuleStrategy {
    SingleFile,
    PerType,
    ByNamespace { separator: String },
}
```

## Benefits
- Better organization for large projects
- Matches TypeScript conventions
- Automatic import management
- Prevents circular dependencies

## Priority
Medium
```

---

## Phase 3: 最適化 (v0.4.0)

### Issue 7: インクリメンタル生成

**Title**: `[PERFORMANCE] Incremental code generation with caching`

**Labels**: `performance`, `v0.4.0`, `optimization`

**Description**:
```markdown
## Motivation
For large projects, regenerating all types on every change is slow. Incremental generation would:
- Speed up development workflow
- Reduce CI/CD time
- Enable efficient watch mode

## Proposed Solution
Implement hash-based caching to skip unchanged types.

### Implementation
```rust
// gear-mesh-generator/src/incremental.rs
pub struct IncrementalGenerator {
    cache: HashMap<String, u64>,
}

impl IncrementalGenerator {
    pub fn generate_if_changed(
        &mut self,
        ty: &GearMeshType,
        config: &GeneratorConfig,
    ) -> Option<String> {
        let hash = self.compute_hash(ty, config);
        
        if self.cache.get(&ty.name) == Some(&hash) {
            return None; // Skip unchanged
        }
        
        // Generate and update cache
        let output = generate(ty, config);
        self.cache.insert(ty.name.clone(), hash);
        Some(output)
    }
}
```

### Cache Storage
- Store in `.gear-mesh-cache` directory
- JSON format for portability
- Include version info for invalidation

### Configuration
```rust
pub struct GeneratorConfig {
    // ...
    pub enable_cache: bool,
    pub cache_dir: PathBuf,
}
```

## Benefits
- 10-100x faster for large projects
- Better developer experience
- Efficient CI/CD

## Implementation Requirements
1. Add `Hash` trait to `GearMeshType` and `GeneratorConfig`
2. Implement cache serialization/deserialization
3. Add cache invalidation logic
4. Update tests

## Priority
Low (Optimization for large projects)
```

---

### Issue 8: プロパティベーステスト

**Title**: `[TEST] Add property-based testing with proptest`

**Labels**: `testing`, `v0.4.0`, `quality`

**Description**:
```markdown
## Motivation
Property-based testing can catch edge cases that unit tests miss by:
- Testing with randomly generated inputs
- Verifying invariants hold for all inputs
- Finding minimal failing cases

## Proposed Tests

### 1. Generated TypeScript is Syntactically Valid
```rust
proptest! {
    #[test]
    fn generated_typescript_is_valid(
        ty in arbitrary_gear_mesh_type()
    ) {
        let output = generate_typescript(&ty);
        assert!(is_valid_typescript(&output));
    }
}
```

### 2. Roundtrip Serialization
```rust
proptest! {
    #[test]
    fn roundtrip_serialization(ty in arbitrary_gear_mesh_type()) {
        let json = serde_json::to_string(&ty).unwrap();
        let deserialized: GearMeshType = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(ty, deserialized);
    }
}
```

### 3. Zod Schema Validity
```rust
proptest! {
    #[test]
    fn zod_schema_is_valid(ty in arbitrary_gear_mesh_type()) {
        let schema = generate_zod_schema(&ty);
        assert!(is_valid_zod_schema(&schema));
    }
}
```

## Implementation
1. Add `proptest` to dev-dependencies
2. Create `tests/property_tests.rs`
3. Implement `Arbitrary` for `GearMeshType`
4. Add TypeScript/Zod validation helpers

## Dependencies
- `proptest = "1.4"`
- TypeScript compiler for validation

## Priority
Low (Quality improvement)
```

---

### Issue 9: JSDoc強化

**Title**: `[ENHANCEMENT] Enhanced JSDoc with type info and validation rules`

**Labels**: `enhancement`, `v0.4.0`, `documentation`

**Description**:
```markdown
## Current State
Basic JSDoc comments are generated from Rust doc comments.

```typescript
/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
}
```

## Proposed Enhancement
Include type information, validation rules, and metadata.

```typescript
/**
 * User information
 * 
 * @remarks
 * This type is automatically generated from Rust.
 * Do not modify manually.
 * 
 * @see {@link UserSchema} for runtime validation
 */
export interface User {
    /**
     * User's unique identifier
     * @type {UserId}
     * @readonly
     */
    id: UserId;
    
    /**
     * User's display name
     * @type {string}
     * @minLength 1
     * @maxLength 20
     */
    name: string;
}
```

## Implementation
```rust
impl DocComment {
    pub fn to_enhanced_jsdoc(
        &self, 
        field: &FieldInfo,
        config: &GeneratorConfig
    ) -> String {
        // Include:
        // - Type information
        // - Validation constraints
        // - Readonly/optional markers
        // - Related schemas
    }
}
```

## Benefits
- Better IDE autocomplete
- Inline documentation of constraints
- Improved developer experience

## Priority
Low (Nice to have)
```

---

## その他の改善

### Issue 10: TypeScript型マッピングの最適化

**Title**: `[ENHANCEMENT] Improve TypeScript type mapping for Option and Result`

**Labels**: `enhancement`, `type-mapping`

**Description**:
```markdown
## Current Behavior
- `Option<T>` → `T | null`
- `Result<T, E>` → `T` (error type ignored)

## Problems
1. `Option<T>` doesn't distinguish between `null` and `undefined`
2. `Result<T, E>` loses error information

## Proposed Solution

### Configurable Option Mapping
```rust
pub enum OptionStyle {
    Nullable,        // T | null
    Optional,        // T | undefined
    Both,            // T | null | undefined
}
```

### Better Result Mapping
```rust
pub enum ResultStyle {
    OkOnly,          // T (current behavior)
    TaggedUnion,     // { ok: T } | { err: E }
    SuccessError,    // { success: true, data: T } | { success: false, error: E }
}
```

### Configuration
```rust
pub struct GeneratorConfig {
    // ...
    pub option_style: OptionStyle,
    pub result_style: ResultStyle,
}
```

### Example Output (TaggedUnion)
```typescript
export type Result<T, E> = 
    | { ok: T }
    | { err: E };

export type ApiResponse = Result<User, ApiError>;
```

## Benefits
- More accurate type mapping
- Better error handling in TypeScript
- Flexibility for different use cases

## Priority
Medium
```

---

## Usage

To register these issues:

1. Open GitHub Issues tab
2. Click "New Issue"
3. Copy the template content
4. Add appropriate labels
5. Submit

## Labels to Create

- `v0.2.0`, `v0.3.0`, `v0.4.0` - Version milestones
- `refactor` - Code refactoring
- `dx` - Developer experience
- `error-handling` - Error handling improvements
- `testing` - Testing improvements
- `quality` - Code quality
- `plugin-system` - Plugin system
- `extensibility` - Extensibility features
- `code-generation` - Code generation
- `performance` - Performance optimization
- `optimization` - General optimization
- `documentation` - Documentation
- `type-mapping` - Type mapping improvements
- `validation` - Validation features

**Title**: `[FEATURE] Plugin System for Custom Type Transformations`

**Labels**: `enhancement`, `phase-2`, `plugin-system`

**Description**:
```markdown
## Feature Description
Implement a plugin system that allows users to customize type transformations and add custom code generation.

## Motivation
Users need to handle custom types (e.g., DateTime, Decimal, custom domain types) that require special handling in TypeScript.

## Proposed Solution
Create a plugin API that allows:
1. Custom type transformations
2. Helper function generation
3. Import statement customization

## Priority
Medium
```

---

### Issue 3: Type Migration Support

**Title**: `[FEATURE] Type Migration Support`

**Labels**: `enhancement`, `phase-2`, `migration`

**Description**:
```markdown
## Feature Description
Automatically detect type changes and generate migration scripts to help transition from old to new types.

## Motivation
When API types change, frontend code needs to be updated. This feature helps automate that process.

## Priority
Medium
```

---

## Phase 3: Ecosystem

### Issue 4: VS Code Extension

**Title**: `[FEATURE] VS Code Extension for gear-mesh`

**Labels**: `enhancement`, `phase-3`, `ide-integration`

**Description**:
```markdown
## Feature Description
Create a VS Code extension that provides IDE integration for gear-mesh.

## Proposed Features
1. **Hover Preview**
2. **Attribute Completion**
3. **Real-time Type Checking**
4. **Quick Actions**

## Priority
Low (Phase 3)
```

---

### Issue 5: Bidirectional Sync (Experimental)

**Title**: `[FEATURE] Bidirectional Type Sync (Experimental)`

**Labels**: `enhancement`, `phase-3`, `experimental`

**Description**:
```markdown
## Feature Description
Enable TypeScript → Rust type conversion for bidirectional synchronization.

## Priority
Low (Experimental)
```

---

## Short-term Improvements

### Issue 7: Improved Error Messages

**Title**: `[IMPROVEMENT] Better Error Messages`

**Labels**: `enhancement`, `v0.2.0`, `dx`

**Description**:
```markdown
## Description
Improve error messages to be more helpful and actionable.

## Current Issues
- Some generic error messages
- Lack of context in parse errors

## Proposed Improvements
1. **Detailed Error Context**
2. **Actionable Suggestions**

## Priority
Medium (v0.2.0)
```

---

## Usage

To register these issues:

1. Open GitHub Issues tab
2. Click "New Issue"
3. Copy paste the template
4. Add labels
