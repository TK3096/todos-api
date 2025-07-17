use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;

use crate::{
    application::usecases::users::UsersUseCase,
    domain::{repositories::users::UsersRepository, value_objects::users::RegisterUserModel},
    infrastructure::app_state::repositories::users::UsersAppState,
};

pub fn routes(users_state: Arc<UsersAppState>) -> Router {
    let users_use_case = Arc::new(UsersUseCase::new(users_state));

    Router::new()
        .route("/register", post(register))
        .route("/", get(list))
        .with_state(users_use_case)
}

pub async fn register<T>(
    State(users_use_case): State<Arc<UsersUseCase<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match users_use_case.register(register_user_model).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"message": "Create user success" })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error" })),
        )
            .into_response(),
    }
}

pub async fn list<T>(State(users_use_case): State<Arc<UsersUseCase<T>>>) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match users_use_case.list().await {
        Ok(users) => (StatusCode::OK, Json(json!({"data": users }))).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error" })),
        )
            .into_response(),
    }
}
