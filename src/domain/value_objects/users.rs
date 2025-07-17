use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::users::RegisterUserEntity;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RegisterUserModel {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 3, message = "Password cannot be empty, at least 3 characters"))]
    pub password: String,
}

impl RegisterUserModel {
    pub fn to_entity(self) -> RegisterUserEntity {
        RegisterUserEntity {
            username: self.username.clone(),
            password: self.password.clone(),
        }
    }
}
