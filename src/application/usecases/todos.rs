use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::todos::TodoEntity, repositories::todos::TodosRepository,
    value_objects::todos::AddTodoModel,
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

    pub async fn add(&self, user_id: String, todo_model: AddTodoModel) -> Result<TodoEntity> {
        let todo_entity = todo_model.to_entity();
        let result = self.todo_repository.add(user_id, todo_entity).await?;

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
