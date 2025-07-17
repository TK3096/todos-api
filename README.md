# Todos API

A RESTful API for managing todo items built with Rust using Axum web framework and clean architecture principles.

## Features

### Current Features
- ✅ Create new todo items
- ✅ Retrieve all todos
- ✅ Retrieve todo by ID
- ✅ Update todo status (completed/incomplete)
- ✅ Delete todo items
- ✅ User management (create, retrieve, update, delete users)
- ✅ JWT Authentication and authorization
- ✅ Clean architecture implementation with Domain-Driven Design
- ✅ Value objects for domain modeling
- ✅ Async/await support
- ✅ JSON serialization/deserialization
- ✅ Structured logging with tracing
- ✅ HTTP middleware support

### Planned Features
- 🔜 **File Persistence** - Save todos to local file storage


## Architecture

This project follows clean architecture principles with the following layers:

```
src/
├── main.rs                     # Application entry point
├── lib.rs                      # Library configuration
├── application/                # Application layer
│   └── usecases/              # Business use cases
│       ├── authentication.rs  # Authentication business logic
│       ├── todos.rs           # Todo business logic
│       └── users.rs           # User business logic
├── domain/                     # Domain layer
│   ├── entities/              # Domain entities
│   │   ├── todos.rs           # Todo entity definitions
│   │   └── users.rs           # User entity definitions
│   ├── repositories/          # Repository interfaces
│   │   ├── todos.rs           # Todo repository trait
│   │   └── users.rs           # User repository trait
│   └── value_objects/         # Domain value objects
│       ├── todos.rs           # Todo value objects
│       └── users.rs           # User value objects
└── infrastructure/            # Infrastructure layer
    ├── app_state/             # Application state management
    │   └── repositories/      # Repository implementations
    ├── axum_http/             # HTTP layer (Axum)
    │   ├── http_serve.rs      # Server configuration
    │   ├── default_routers.rs # Route definitions
    │   ├── middleware.rs      # HTTP middleware
    │   └── routers/           # Route handlers
    │       ├── authentication.rs # Auth route handlers
    │       ├── todos.rs       # Todo route handlers
    │       └── users.rs       # User route handlers
    └── jwt_authentication/    # JWT authentication infrastructure
        ├── authentication_model.rs # Auth models
        └── jwt_model.rs       # JWT models
```

## Prerequisites

- Rust 1.70+ (uses 2024 edition)
- Cargo

## Installation

1. Clone the repository:
```bash
git clone https://github.com/TK3096/todos-api.git
cd todos-api
```

2. Build the project:
```bash
cargo build
```

3. Run the application:
```bash
cargo run
```

The server will start on `http://localhost:3000` (or the configured port).

## API Endpoints

### Authentication
```http
POST /auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password"
}
```

```http
POST /auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password"
}
```

### Users
```http
GET /users
Authorization: Bearer {jwt_token}
```

```http
GET /users/{id}
Authorization: Bearer {jwt_token}
```

```http
POST /users
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password"
}
```

```http
PUT /users/{id}
Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "email": "updated@example.com"
}
```

```http
DELETE /users/{id}
Authorization: Bearer {jwt_token}
```

### Todos
```http
GET /todos
Authorization: Bearer {jwt_token}
```

```http
GET /todos/{id}
Authorization: Bearer {jwt_token}
```

```http
POST /todos
Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "title": "Learn Rust"
}
```

```http
PUT /todos/{id}
Authorization: Bearer {jwt_token}
Content-Type: application/json

{
  "title": "Learn Rust Advanced",
  "completed": true
}
```

```http
DELETE /todos/{id}
Authorization: Bearer {jwt_token}
```

## Data Models

### UserEntity
```rust
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "password_hash": "hashed_password",
  "created_at": "2024-01-01T12:00:00",
  "updated_at": "2024-01-01T12:00:00"
}
```

### TodoEntity
```rust
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "user_id": "550e8400-e29b-41d4-a716-446655440001",
  "title": "Learn Rust",
  "completed": false,
  "created_at": "2024-01-01T12:00:00",
  "updated_at": "2024-01-01T12:00:00"
}
```

### AddTodoRequest
```rust
{
  "title": "Learn Rust"
}
```

## Dependencies

- **axum** - Modern web framework for Rust
- **tokio** - Async runtime
- **serde** - Serialization/deserialization
- **chrono** - Date and time handling
- **uuid** - UUID generation
- **tracing** - Structured logging
- **anyhow** - Error handling
- **jsonwebtoken** - JWT token handling
- **bcrypt** - Password hashing
- **tower** - Middleware and service abstractions

## Development

### Running Tests
```bash
cargo test
```

### Running with Debug Logging
```bash
RUST_LOG=debug cargo run
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Project Structure

The project follows Domain-Driven Design (DDD) and Clean Architecture principles:

- **Domain Layer**: Contains business entities, value objects, and repository interfaces
- **Application Layer**: Contains use cases and business logic
- **Infrastructure Layer**: Contains external concerns (HTTP, JWT authentication, database, etc.)

This separation ensures:
- Business logic is independent of external frameworks
- Easy testing and mocking
- Clear separation of concerns
- Maintainable and scalable code
- Domain modeling with value objects for better type safety