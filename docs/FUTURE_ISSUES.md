# Missing Features GitHub Issues

This file contains drafts for missing features to be registered as GitHub Issues.

## Phase 2: Enhanced Features

### Issue 1: Advanced Validation Generation

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

## Priority
Medium

## Related
- docs/IMPLEMENTATION_STATUS.md
```

---

### Issue 2: Plugin System

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
