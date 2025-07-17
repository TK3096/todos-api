use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}
