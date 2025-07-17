use anyhow::Result;
use async_trait::async_trait;

use crate::domain::entities::users::{RegisterUserEntity, UserEntity};

#[async_trait]
pub trait UsersRepository {
    async fn register(&self, user: RegisterUserEntity) -> Result<()>;
    async fn list(&self) -> Result<Vec<UserEntity>>;
    async fn find_by_username(&self, username: String) -> Result<Option<UserEntity>>;
}
