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
