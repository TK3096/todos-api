pub mod authentication_model;
pub mod jwt_model;

use anyhow::{Ok, Result};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::infrastructure::jwt_authentication::jwt_model::Claims;

pub fn generate_token(secret: String, claims: &Claims) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_toke(secret: String, token: String) -> Result<Claims> {
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
