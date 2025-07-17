use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;
use serde_json::json;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    domain::repositories::users::UsersRepository,
    infrastructure::{
        app_state::repositories::users::UsersAppState,
        jwt_authentication::authentication_model::LoginModel,
    },
};

pub fn routes(users_state: Arc<UsersAppState>) -> Router {
    let authentication_use_case = Arc::new(AuthenticationUseCase::new(users_state));

    Router::new()
        .route("/login", post(login))
        .route("/refresh-token", post(refresh_token))
        .with_state(authentication_use_case)
}

pub async fn login<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    match authentication_use_case.login(login_model).await {
        Ok(passport) => {
            let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (
                StatusCode::OK,
                headers,
                Json(json!({ "message": "Login Successfully" })),
            )
                .into_response()
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized" })),
        )
            .into_response(),
    }
}

pub async fn refresh_token<T>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: UsersRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_use_case.refresh_token(refresh_token).await {
            Ok(passport) => {
                let act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (
                    StatusCode::OK,
                    Json(json!({ "message": "Refresh token successfully" })),
                )
                    .into_response()
            }
            Err(_) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Unauthorized" })),
            )
                .into_response(),
        };
        return response;
    };

    (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": "Bad Request"})),
    )
        .into_response()
}
