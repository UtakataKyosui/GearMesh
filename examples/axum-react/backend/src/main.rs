use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use gear_mesh::GearMesh;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

// ============================================================================
// Types with GearMesh derive for TypeScript generation
// ============================================================================

/// User ID (Branded Type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, GearMesh)]
#[gear_mesh(branded)]
pub struct UserId(pub i32);

/// User information
#[derive(Debug, Clone, Serialize, Deserialize, GearMesh)]
pub struct User {
    /// User's unique identifier
    pub id: UserId,
    /// User's display name
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    /// User's email address
    pub email: String,
    /// User's age (optional)
    #[validate(range(min = 1, max = 100))]
    pub age: Option<i32>,
}

/// Request to create a new user
#[derive(Debug, Deserialize, GearMesh)]
pub struct CreateUserRequest {
    /// Display name
    pub name: String,
    /// Email address
    pub email: String,
    /// Age (optional)
    pub age: Option<i32>,
}

/// Response after creating a user
#[derive(Debug, Serialize, GearMesh)]
pub struct CreateUserResponse {
    /// The created user
    pub user: User,
    /// Success message
    pub message: String,
}

/// List of users
#[derive(Debug, Serialize, GearMesh)]
pub struct UserList {
    /// All users
    pub users: Vec<User>,
    /// Total count
    pub total: i32,
}

/// Error response
#[derive(Debug, Serialize, GearMesh)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

// ============================================================================
// Application State
// ============================================================================

#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
    next_id: Arc<RwLock<i32>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(vec![
                User {
                    id: UserId(1),
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                    age: Some(25),
                },
                User {
                    id: UserId(2),
                    name: "Bob".to_string(),
                    email: "bob@example.com".to_string(),
                    age: Some(30),
                },
            ])),
            next_id: Arc::new(RwLock::new(3)),
        }
    }
}

// ============================================================================
// API Handlers
// ============================================================================

/// Get all users
async fn get_users(State(state): State<AppState>) -> Json<UserList> {
    let users = state.users.read().await;
    Json(UserList {
        total: users.len() as i32,
        users: users.clone(),
    })
}

/// Get user by ID
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<ErrorResponse>)> {
    let users = state.users.read().await;
    users
        .iter()
        .find(|u| u.id.0 == id)
        .cloned()
        .map(Json)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("User with id {} not found", id),
                }),
            )
        })
}

/// Create new user
async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> (StatusCode, Json<CreateUserResponse>) {
    let mut users = state.users.write().await;
    let mut next_id = state.next_id.write().await;

    let user = User {
        id: UserId(*next_id),
        name: req.name,
        email: req.email,
        age: req.age,
    };

    *next_id += 1;
    users.push(user.clone());

    (
        StatusCode::CREATED,
        Json(CreateUserResponse {
            user,
            message: "User created successfully".to_string(),
        }),
    )
}

/// Delete user by ID
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut users = state.users.write().await;
    let initial_len = users.len();
    users.retain(|u| u.id.0 != id);

    if users.len() < initial_len {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("User with id {} not found", id),
            }),
        ))
    }
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() {
    // Generate TypeScript types automatically from all #[derive(GearMesh)] types
    gear_mesh::generate_types("../frontend/src/types/generated.ts")
        .expect("Failed to generate TypeScript types");

    let state = AppState::new();

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", get(get_user).delete(delete_user))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📝 API endpoints:");
    println!("   GET    /api/users");
    println!("   POST   /api/users");
    println!("   GET    /api/users/:id");
    println!("   DELETE /api/users/:id");

    axum::serve(listener, app).await.unwrap();
}
