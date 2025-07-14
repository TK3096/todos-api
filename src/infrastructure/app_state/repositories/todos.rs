use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::todos::{AddTodoEntity, TodoEntity},
    repositories::todos::TodosRepository,
};

#[derive(Clone)]
pub struct TodosAppState {
    todos: Arc<Mutex<Vec<TodoEntity>>>,
}

impl TodosAppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl TodosRepository for TodosAppState {
    async fn list(&self) -> Result<Vec<TodoEntity>> {
        let todos = self.todos.lock().unwrap();
        Ok(todos.clone())
    }

    async fn get(&self, id: String) -> Result<TodoEntity> {
        let todos = self.todos.lock().unwrap();

        let result = todos
            .iter()
            .find(|todo| todo.id == id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Todo not found"))?;

        Ok(result)
    }

    async fn add(&self, payload: AddTodoEntity) -> Result<TodoEntity> {
        let mut todos = self.todos.lock().unwrap();

        let new_todo = TodoEntity {
            id: Uuid::new_v4().to_string(),
            title: payload.title,
            completed: false,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        todos.push(new_todo.clone());

        Ok(new_todo)
    }

    async fn to_completed(&self, id: String) -> Result<TodoEntity> {
        let mut todos = self.todos.lock().unwrap();

        let todo = todos.iter_mut().find(|todo| todo.id == id);

        match todo {
            Some(t) => {
                t.completed = true;
                t.updated_at = chrono::Utc::now().naive_utc();
                Ok(t.clone())
            }
            None => return Err(anyhow::anyhow!("Todo not found")),
        }
    }

    async fn delete(&self, id: String) -> Result<()> {
        let mut todos = self.todos.lock().unwrap();

        let index = todos.iter().position(|todo| todo.id == id);

        match index {
            Some(i) => {
                todos.remove(i);
                Ok(())
            }
            None => Err(anyhow::anyhow!("Todo not found")),
        }
    }
}
