use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::todos::{AddTodoEntity, TodoEntity},
    repositories::todos::TodosRepository,
};

pub struct TodosUseCase<T>
where
    T: TodosRepository + Send + Sync,
{
    todo_repository: Arc<T>,
}

impl<T> TodosUseCase<T>
where
    T: TodosRepository + Send + Sync,
{
    pub fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    pub async fn list(&self) -> Result<Vec<TodoEntity>> {
        let result = self.todo_repository.list().await?;

        Ok(result)
    }

    pub async fn get(&self, id: String) -> Result<TodoEntity> {
        let result = self.todo_repository.get(id).await?;

        Ok(result)
    }

    pub async fn add(&self, payload: AddTodoEntity) -> Result<TodoEntity> {
        let result = self.todo_repository.add(payload).await?;

        Ok(result)
    }

    pub async fn to_completed(&self, id: String) -> Result<TodoEntity> {
        let result = self.todo_repository.to_completed(id).await?;

        Ok(result)
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.todo_repository.delete(id).await
    }
}
