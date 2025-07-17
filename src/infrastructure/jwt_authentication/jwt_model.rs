use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Passport {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
