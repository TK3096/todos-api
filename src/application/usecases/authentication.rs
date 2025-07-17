use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{
    domain::repositories::users::UsersRepository,
    infrastructure::jwt_authentication::{
        self,
        authentication_model::LoginModel,
        jwt_model::{Claims, Passport},
    },
};

pub struct AuthenticationUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    users_repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: UsersRepository + Send + Sync,
{
    pub fn new(users_repository: Arc<T>) -> Self {
        Self { users_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        if let Some(user) = self
            .users_repository
            .find_by_username(login_model.username.clone())
            .await?
        {
            if user.password == login_model.password {
                let access_token_claims = Claims {
                    sub: user.id.clone(),
                    exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                    iat: Utc::now().timestamp() as usize,
                };

                let refresh_token_claims = Claims {
                    sub: user.id,
                    exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
                    iat: Utc::now().timestamp() as usize,
                };

                let access_token =
                    jwt_authentication::generate_token("secret".to_string(), &access_token_claims)?;
                let refresh_token = jwt_authentication::generate_token(
                    "refresh_secret".to_string(),
                    &refresh_token_claims,
                )?;

                Ok(Passport {
                    refresh_token,
                    access_token,
                })
            } else {
                Err(anyhow::anyhow!("Invalid password"))
            }
        } else {
            Err(anyhow::anyhow!("User not found"))
        }
    }

    pub async fn refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let claims = jwt_authentication::verify_toke("refresh_secret".to_string(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token("secret".to_string(), &access_token_claims)?;

        let refresh_token = jwt_authentication::generate_token(
            "refresh_secret".to_string(),
            &refresh_token_claims,
        )?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }
}
