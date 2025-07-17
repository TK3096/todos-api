# Todos API

A RESTful API for managing todo items built with Rust using Axum web framework and clean architecture principles.

## Features

### Current Features
- ✅ Create new todo items
- ✅ Retrieve all todos
- ✅ Retrieve todo by ID
- ✅ Update todo status (completed/incomplete)
- ✅ Delete todo items
- ✅ Clean architecture implementation with Domain-Driven Design
- ✅ Async/await support
- ✅ JSON serialization/deserialization
- ✅ Structured logging with tracing

### Planned Features
- 🔜 **Authentication** - JWT-based authentication system
- 🔜 **File Persistence** - Save todos to local file storage


## Architecture

This project follows clean architecture principles with the following layers:

```
src/
├── main.rs                     # Application entry point
├── lib.rs                      # Library configuration
├── application/                # Application layer
│   └── usecases/              # Business use cases
│       └── todos.rs           # Todo business logic
├── domain/                     # Domain layer
│   ├── entities/              # Domain entities
│   │   └── todos.rs           # Todo entity definitions
│   └── repositories/          # Repository interfaces
│       └── todos.rs           # Todo repository trait
└── infrastructure/            # Infrastructure layer
    ├── app_state/             # Application state management
    │   └── repositories/      # Repository implementations
    └── axum_http/             # HTTP layer (Axum)
        ├── http_serve.rs      # Server configuration
        ├── default_routers.rs # Route definitions
        └── routers/           # Route handlers
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

### Get All Todos
```http
GET /todos
```

### Get Todo by ID
```http
GET /todos/{id}
```

### Create New Todo
```http
POST /todos
Content-Type: application/json

{
  "title": "Learn Rust"
}
```

### Update Todo
```http
PUT /todos/{id}
Content-Type: application/json

{
  "title": "Learn Rust Advanced",
  "completed": true
}
```

### Delete Todo
```http
DELETE /todos/{id}
```

## Data Models

### TodoEntity
```rust
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Learn Rust",
  "completed": false,
  "created_at": "2024-01-01T12:00:00",
  "updated_at": "2024-01-01T12:00:00"
}
```

### AddTodoEntity (Request)
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

- **Domain Layer**: Contains business entities and repository interfaces
- **Application Layer**: Contains use cases and business logic
- **Infrastructure Layer**: Contains external concerns (HTTP, database, etc.)

This separation ensures:
- Business logic is independent of external frameworks
- Easy testing and mocking
- Clear separation of concerns
- Maintainable and scalable code