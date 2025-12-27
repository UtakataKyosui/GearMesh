# Axum + React Example

Full-stack example demonstrating type-safe communication between Rust (Axum) and TypeScript (React) using GearMesh.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ Backend (Rust + Axum)                                        │
│                                                              │
│  #[derive(GearMesh)]                                         │
│  struct User { ... }         ──────┐                         │
│                                     │                        │
│  REST API Endpoints                 │ GearMesh               │
│  - GET /api/users                   │ generates              │
│  - POST /api/users                  │ TypeScript             │
│  - GET /api/users/:id               │                        │
│  - DELETE /api/users/:id            │                        │
└─────────────────────────────────────┼────────────────────────┘
                                      │
                                      ▼
                            generated.ts (Types)
                                      │
┌─────────────────────────────────────┼────────────────────────┐
│ Frontend (TypeScript + React)       │                        │
│                                     │                        │
│  import type { User } from './types/generated'               │
│                                                              │
│  Type-safe API calls using generated types                   │
│  - Autocomplete                                              │
│  - Compile-time type checking                                │
│  - No manual type definitions needed                         │
└─────────────────────────────────────────────────────────────┘
```

## Features

- **Type Safety**: Rust types automatically converted to TypeScript
- **Branded Types**: `UserId` as a Branded Type for extra type safety
- **JSDoc**: Rust doc comments preserved in TypeScript
- **Full CRUD**: Create, Read, Delete operations
- **Modern Stack**: Axum + React + Vite

## Setup

### Prerequisites

- Rust (1.75+)
- Node.js (20+)

### 1. Start Backend (Auto-generates Types)

```bash
cd backend
cargo run
```

The backend automatically generates TypeScript types on startup using the `export_types!` macro.

### 2. Start Frontend

```bash
cd frontend
npm install
npm run dev
```

Frontend runs on `http://localhost:5173`

## Project Structure

```
axum-react/
├── backend/
│   ├── src/
│   │   └── main.rs          # Axum server with GearMesh types
│   ├── Cargo.toml
│   └── gear-mesh.toml       # GearMesh configuration
│
└── frontend/
    ├── src/
    │   ├── App.tsx          # React app with type-safe API calls
    │   ├── App.css
    │   ├── types/
    │   │   └── generated.ts # Generated TypeScript types
    │   └── main.tsx
    ├── package.json
    └── vite.config.ts
```

## Type Sharing Example

### Rust (Backend)

```rust
use gear_mesh::GearMesh;

/// User ID (Branded Type)
#[derive(GearMesh)]
#[gear_mesh(branded)]
pub struct UserId(pub i32);

/// User information
#[derive(GearMesh)]
pub struct User {
    /// User's unique identifier
    pub id: UserId,
    /// User's display name
    pub name: String,
    /// User's email address
    pub email: String,
    /// User's age (optional)
    pub age: Option<i32>,
}
```

### TypeScript (Frontend) - Generated

```typescript
type Brand<T, B> = T & { readonly __brand: B };

/**
 * User ID (Branded Type)
 */
export type UserId = Brand<number, "UserId">;
export const UserId = (value: number): UserId => value as UserId;

/**
 * User information
 */
export interface User {
    /** User's unique identifier */
    id: UserId;
    /** User's display name */
    name: string;
    /** User's email address */
    email: string;
    /** User's age (optional) */
    age?: number | null;
}
```

### TypeScript (Frontend) - Usage

```typescript
import type { User, CreateUserRequest } from './types/generated';

// Type-safe API call
const createUser = async (request: CreateUserRequest) => {
    const response = await fetch('/api/users', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    });
    
    const data: CreateUserResponse = await response.json();
    return data.user; // TypeScript knows this is User type
};
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | Get all users |
| POST | `/api/users` | Create new user |
| GET | `/api/users/:id` | Get user by ID |
| DELETE | `/api/users/:id` | Delete user |

## Development Workflow

1. **Define types in Rust** with `#[derive(GearMesh)]`
2. **List types in `export_types!` macro** in main.rs
3. **Run backend** - Types are automatically generated
4. **Use generated types** in React components
5. **Enjoy type safety** across the stack!

## Benefits

✅ **No manual type definitions** - Types are generated automatically  
✅ **Always in sync** - Backend changes automatically reflected in frontend  
✅ **Type safety** - Compile-time errors instead of runtime errors  
✅ **Better DX** - Autocomplete and IntelliSense work perfectly  
✅ **Documentation** - JSDoc comments from Rust preserved  

## Learn More

- [GearMesh Documentation](../../README.md)
- [Axum Documentation](https://docs.rs/axum)
- [React Documentation](https://react.dev)
- [Vite Documentation](https://vitejs.dev)
