use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::todos::{AddTodoEntity, TodoEntity};

#[async_trait]
pub trait TodosRepository {
    async fn list(&self) -> Result<Vec<TodoEntity>>;
    async fn get(&self, id: String) -> Result<TodoEntity>;
    async fn add(&self, user_id: String, payload: AddTodoEntity) -> Result<TodoEntity>;
    async fn to_completed(&self, id: String) -> Result<TodoEntity>;
    async fn delete(&self, id: String) -> Result<()>;
}
