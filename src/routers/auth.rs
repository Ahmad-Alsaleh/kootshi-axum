use crate::{configs::config, errors::ServerError, extractors::JwtToken, models::LoginPayload};
use axum::{Json, Router, routing::post};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}

// TODO: allow the client to optionally pass a login token instead of the username (?) and password.
// TODO: read online what happens if someone steals the jwt auth-token and uses it to login on a different device.
async fn login(
    cookies: Cookies,
    Json(body): Json<LoginPayload>,
) -> Result<Json<Value>, ServerError> {
    // TODO: (imp) use proper auth logic
    if body.username != "demo" || body.password != "password" {
        return Err(ServerError::WrongLoginCredentials);
    }

    let context = JwtToken::new(Uuid::nil()); // TODO: (imp) use a real id (once porpoer auth logic is done)
    let jwt_encoded_token = jsonwebtoken::encode(
        &Header::default(),
        &context,
        &EncodingKey::from_secret(config().jwt_secret.as_bytes()),
    )
    .map_err(ServerError::JwtError)?;

    let response = json!({
        "token": jwt_encoded_token
    });

    // TODO: set a max age and use refresh tokens
    let cookie = Cookie::build(("auth-token", jwt_encoded_token))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    Ok(Json(response))
}

// TODO: add a /signup endpoint
