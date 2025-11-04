use crate::{
    errors::ServerError,
    models::{Context, LoginPayload},
};
use axum::{Json, Router, routing::post};
use jsonwebtoken::{EncodingKey, Header};
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(
    cookies: Cookies,
    Json(body): Json<LoginPayload>,
) -> Result<Json<&'static str>, ServerError> {
    // TODO: use proper auth logic
    if body.username != "demo" || body.password != "password" {
        return Err(ServerError::WrongLoginCredentials);
    }

    let context = Context::new(Uuid::nil()); // TODO: use a real id (once porpoer auth logic is done)
    let jwt_encoded_token = jsonwebtoken::encode(
        &Header::default(),
        &context,
        &EncodingKey::from_secret(b"my-secret"), // TODO: use an env var thro a Config object
    )
    .map_err(ServerError::JwtError)?;

    // TODO: set a max age and use refresh tokens
    let cookie = Cookie::build(("token", jwt_encoded_token))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    // TODO: check, maybe there is a status code for login better than 200, maybe 202 (accepted)?
    Ok(Json("success"))
}

// TODO: add a /signup endpoint
