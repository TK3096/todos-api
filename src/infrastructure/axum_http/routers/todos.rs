use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use serde_json::json;

use crate::{
    application::usecases::todos::TodosUseCase,
    domain::{
        repositories::todos::TodosRepository,
        value_objects::todos::{AddTodoModel, TodoErrorMessage},
    },
    infrastructure::app_state::repositories::todos::TodosAppState,
};

pub fn routes() -> Router {
    let app_state = Arc::new(TodosAppState::new());
    let todos_use_case = Arc::new(TodosUseCase::new(app_state));

    Router::new()
        .route("/", post(add_todo))
        .route("/", get(list))
        .route("/{id}", get(get_todo))
        .route("/to_completed/{id}", patch(to_completed))
        .route("/{id}", delete(delete_todo))
        .with_state(todos_use_case)
}

pub async fn add_todo<T>(
    State(todos_use_case): State<Arc<TodosUseCase<T>>>,
    Json(add_todo_model): Json<AddTodoModel>,
) -> impl IntoResponse
where
    T: TodosRepository + Send + Sync,
{
    match todos_use_case.add(add_todo_model).await {
        Ok(todo) => (StatusCode::CREATED, Json(json!({"data": todo}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn list<T>(State(todos_use_case): State<Arc<TodosUseCase<T>>>) -> impl IntoResponse
where
    T: TodosRepository + Send + Sync,
{
    match todos_use_case.list().await {
        Ok(todos) => (
            StatusCode::OK,
            Json(json!({
                "data": todos,
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn get_todo<T>(
    State(todos_use_case): State<Arc<TodosUseCase<T>>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    T: TodosRepository + Send + Sync,
{
    match todos_use_case.get(id).await {
        Ok(todo) => (StatusCode::OK, Json(json!({"data": todo}))).into_response(),
        Err(e) => {
            let error_message = e.to_string();

            if error_message.contains(&TodoErrorMessage::NotFound.to_string()) {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Todo not found"})),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response()
            }
        }
    }
}

pub async fn to_completed<T>(
    State(todos_use_case): State<Arc<TodosUseCase<T>>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    T: TodosRepository + Send + Sync,
{
    match todos_use_case.to_completed(id).await {
        Ok(todo) => (StatusCode::OK, Json(json!({"data": todo}))).into_response(),
        Err(e) => {
            let error_message = e.to_string();

            if error_message.contains(&TodoErrorMessage::NotFound.to_string()) {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Todo not found"})),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response()
            }
        }
    }
}

pub async fn delete_todo<T>(
    State(todos_use_case): State<Arc<TodosUseCase<T>>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    T: TodosRepository + Send + Sync,
{
    match todos_use_case.delete(id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Success" }))).into_response(),
        Err(e) => {
            let error_message = e.to_string();

            if error_message.contains(&TodoErrorMessage::NotFound.to_string()) {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Todo not found"})),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Internal Server Error"})),
                )
                    .into_response()
            }
        }
    }
}
