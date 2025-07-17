use std::sync::{Arc, Mutex};

use anyhow::{Ok, Result};
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::users::{RegisterUserEntity, UserEntity},
    repositories::users::UsersRepository,
};

#[derive(Clone)]
pub struct UsersAppState {
    users: Arc<Mutex<Vec<UserEntity>>>,
}

impl UsersAppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl UsersRepository for UsersAppState {
    async fn register(&self, payload: RegisterUserEntity) -> Result<()> {
        let user = UserEntity {
            id: Uuid::new_v4().to_string(),
            username: payload.username,
            password: payload.password, // In a real application, ensure to hash the password
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        self.users.lock().unwrap().push(user);

        Ok(())
    }

    async fn list(&self) -> Result<Vec<UserEntity>> {
        let users = self.users.lock().unwrap();

        Ok(users.clone())
    }

    async fn find_by_username(&self, username: String) -> Result<Option<UserEntity>> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|u| u.username == username).cloned();

        Ok(user)
    }
}
