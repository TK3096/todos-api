use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::infrastructure::jwt_authentication;

pub async fn user_authentication(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            let access_token = get_cookie_value(cookie_str, "act");

            if let Some(token) = access_token {
                if let Ok(claims) = jwt_authentication::verify_toke("secret".to_string(), token) {
                    req.extensions_mut().insert(claims.sub);

                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
