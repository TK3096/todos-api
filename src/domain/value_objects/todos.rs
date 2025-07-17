use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::todos::AddTodoEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AddTodoModel {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
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
