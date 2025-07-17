use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::users::UserEntity, repositories::users::UsersRepository,
    value_objects::users::RegisterUserModel,
};

pub struct UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    user_repository: Arc<T>,
}

impl<T> UsersUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    pub async fn register(&self, user_model: RegisterUserModel) -> Result<()> {
        let user_entity = user_model.to_entity();

        self.user_repository.register(user_entity).await?;

        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<UserEntity>> {
        let users = self.user_repository.list().await?;

        Ok(users)
    }
}
