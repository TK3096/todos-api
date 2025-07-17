use serde::{Deserialize, Serialize};

use crate::domain::entities::todos::AddTodoEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTodoModel {
    pub title: String,
}

impl AddTodoModel {
    pub fn to_entity(self) -> AddTodoEntity {
        AddTodoEntity {
            title: self.title.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TodoErrorMessage {
    NotFound,
}

impl TodoErrorMessage {
    pub fn to_string(&self) -> String {
        match self {
            TodoErrorMessage::NotFound => "NotFound".to_string(),
        }
    }
}
