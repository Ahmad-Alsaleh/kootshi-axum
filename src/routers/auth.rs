use crate::{errors::ServerError, models::LoginPayload};
use axum::{Json, Router, routing::post};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}

#[derive(Serialize)]
struct Claims {
    user_id: String,
}

async fn login(
    cookies: Cookies,
    Json(body): Json<LoginPayload>,
) -> Result<Json<&'static str>, ServerError> {
    // TODO: use proper auth logic
    if body.username != "demo" || body.password != "password" {
        return Err(ServerError::WrongLoginCredentials);
    }

    // TODO: set a max age and use refresh tokens
    let claims = Claims {
        user_id: String::from("user-1"), // use a real id (once porpoer auth logic is done)
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"my-secret"), // TODO: use an env var
    )
    .unwrap(); // TODO: remove unwrap and handle the error
    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    // TODO: check, maybe there is a status code for login better than 200
    Ok(Json("success"))
}
