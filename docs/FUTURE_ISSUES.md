# 未実装機能のGitHub Issues

このファイルには、gear-meshの未実装機能をGitHub Issueとして登録するための下書きが含まれています。

## Phase 2: 拡張機能

### Issue 1: 高度なバリデーション生成

**Title**: `[FEATURE] Advanced Validation Generation`

**Labels**: `enhancement`, `phase-2`, `validation`

**Description**:
```markdown
## Feature Description
Enhance validation generation to support custom validation rules and complex validation logic.

## Current Status
- ✅ Basic validation rules (Range, Length, Email, URL, Pattern)
- ✅ Simple validation function generation
- ✅ Basic Zod schema generation
- ❌ Custom validation rules
- ❌ Complex validation logic
- ❌ Error message customization

## Proposed Features
1. **Custom Validation Rules**
   - Allow users to define custom validation functions
   - Support for cross-field validation
   
2. **Error Message Customization**
   - Customizable error messages per validation rule
   - i18n support for error messages

3. **Advanced Zod Integration**
   - More sophisticated Zod schema generation
   - Support for Zod refinements and transforms

## Example Usage
```rust
#[derive(GearMesh)]
#[gear_mesh(validate)]
struct User {
    #[validate(custom = "validate_username")]
    username: String,
    
    #[validate(email, message = "Invalid email format")]
    email: String,
    
    #[validate(cross_field = "password_confirmation")]
    password: String,
}
```

## Implementation Plan
1. Extend ValidationRule enum in gear-mesh-core
2. Update validation generator in gear-mesh-generator
3. Add tests for new validation types
4. Update documentation

## Priority
Medium

## Related
- docs/IMPLEMENTATION_STATUS.md
```

---

### Issue 2: プラグインシステム

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

## Example Plugin
```rust
use gear_mesh::Plugin;

pub struct DateTimePlugin;

impl Plugin for DateTimePlugin {
    fn transform(&self, rust_type: &RustType) -> Option<TypeScriptType> {
        match rust_type.name.as_str() {
            "DateTime" => Some(TypeScriptType::Custom("Date".to_string())),
            "NaiveDate" => Some(TypeScriptType::Custom("string".to_string())),
            _ => None,
        }
    }

    fn generate_helpers(&self) -> String {
        r#"
        export function parseDateTime(value: string): Date {
            return new Date(value);
        }
        "#.to_string()
    }
}
```

## Implementation Plan
1. Design Plugin trait API
2. Implement plugin loader and registry
3. Add plugin configuration to gear-mesh.toml
4. Create example plugins
5. Document plugin development guide

## Priority
Medium

## Related
- FILE.md (original design)
- docs/IMPLEMENTATION_STATUS.md
```

---

### Issue 3: マイグレーション支援

**Title**: `[FEATURE] Type Migration Support`

**Labels**: `enhancement`, `phase-2`, `migration`

**Description**:
```markdown
## Feature Description
Automatically detect type changes and generate migration scripts to help transition from old to new types.

## Motivation
When API types change, frontend code needs to be updated. This feature helps automate that process.

## Proposed Features
1. **Type Change Detection**
   - Compare old and new type definitions
   - Identify breaking changes

2. **Migration Script Generation**
   - Generate TypeScript migration functions
   - Handle field renames, splits, merges

3. **Version Management**
   - Track type definition versions
   - Support gradual migration

## Example
```rust
#[derive(GearMesh)]
struct User {
    id: i32,
    #[gear_mesh(migrate_from = "name", split_on = " ")]
    first_name: String,
    last_name: String,
    email: String,
}
```

Generated migration:
```typescript
export function migrateUser(old: OldUser): User {
    const [firstName, ...lastNameParts] = old.name.split(' ');
    return {
        id: old.id,
        first_name: firstName,
        last_name: lastNameParts.join(' '),
        email: old.email,
    };
}
```

## Implementation Plan
1. Design migration attribute syntax
2. Implement type diff detection
3. Create migration code generator
4. Add version tracking
5. Write migration guide

## Priority
Medium

## Related
- FILE.md (original design)
```

---

## Phase 3: エコシステム

### Issue 4: VS Code拡張機能

**Title**: `[FEATURE] VS Code Extension for gear-mesh`

**Labels**: `enhancement`, `phase-3`, `ide-integration`

**Description**:
```markdown
## Feature Description
Create a VS Code extension that provides IDE integration for gear-mesh.

## Proposed Features
1. **Hover Preview**
   - Show generated TypeScript type on hover over Rust code
   
2. **Attribute Completion**
   - Auto-complete for `#[gear_mesh(...)]` attributes
   
3. **Real-time Type Checking**
   - Warn about type inconsistencies
   
4. **Quick Actions**
   - Generate TypeScript from current file
   - Jump to generated TypeScript definition

## Implementation Plan
1. Set up VS Code extension project
2. Implement Language Server Protocol
3. Add hover provider
4. Add completion provider
5. Publish to VS Code marketplace

## Priority
Low (Phase 3)

## Related
- docs/IMPLEMENTATION_STATUS.md
```

---

### Issue 5: 双方向同期（実験的）

**Title**: `[FEATURE] Bidirectional Type Sync (Experimental)`

**Labels**: `enhancement`, `phase-3`, `experimental`

**Description**:
```markdown
## Feature Description
Enable TypeScript → Rust type conversion for bidirectional synchronization.

## Motivation
Sometimes it's easier to define types in TypeScript first, especially for frontend-driven development.

## Proposed Solution
```bash
gear-mesh reverse --input schema/api.ts --output src/types.rs
```

## Challenges
- TypeScript AST parsing
- Mapping TypeScript types to Rust
- Handling TypeScript-specific features
- Merging with existing Rust code

## Implementation Plan
1. Research TypeScript AST parsers
2. Design reverse transformation rules
3. Implement basic TypeScript → Rust conversion
4. Add conflict resolution
5. Mark as experimental feature

## Priority
Low (Experimental)

## Related
- FILE.md (original design)
```

---

## 短期改善項目

### Issue 6: proc-macroの完全統合

**Title**: `[IMPROVEMENT] Complete proc-macro Integration`

**Labels**: `enhancement`, `v0.2.0`, `proc-macro`

**Description**:
```markdown
## Description
Currently, the proc-macro requires manual type registration. Implement automatic type extraction from proc-macro.

## Current Limitation
The `#[derive(GearMesh)]` macro is implemented but doesn't automatically register types for CLI generation.

## Proposed Solution
1. Use proc-macro to emit type information at compile time
2. Collect type information in a central registry
3. CLI reads from registry for generation

## Implementation Plan
1. Design type registry format
2. Update proc-macro to emit registration code
3. Update CLI to read from registry
4. Add integration tests

## Priority
High (v0.2.0)
```

---

### Issue 7: エラーメッセージの改善

**Title**: `[IMPROVEMENT] Better Error Messages`

**Labels**: `enhancement`, `v0.2.0`, `dx`

**Description**:
```markdown
## Description
Improve error messages to be more helpful and actionable.

## Current Issues
- Generic error messages
- Lack of context in errors
- No suggestions for fixes

## Proposed Improvements
1. **Detailed Error Context**
   - Show source location
   - Highlight problematic code
   
2. **Actionable Suggestions**
   - Suggest fixes for common errors
   - Provide examples of correct usage

3. **Error Categories**
   - Parse errors
   - Type conversion errors
   - Configuration errors

## Priority
Medium (v0.2.0)
```

---

## 使用方法

これらのIssueをGitHubに登録する際は:

1. GitHubリポジトリの Issues タブを開く
2. "New Issue" をクリック
3. 上記のテンプレートをコピー&ペースト
4. 適切なラベルを付与
5. マイルストーンを設定（該当する場合）

## 優先度の目安

- **High**: v0.2.0で実装予定
- **Medium**: v0.3.0 - v0.5.0で実装予定
- **Low**: v1.0.0以降で実装予定
