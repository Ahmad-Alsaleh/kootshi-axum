use crate::{errors::ServerError, models::LoginPayload};
use axum::{Json, Router, routing::post};
use tower_cookies::{Cookie, Cookies};

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

    // TODO: create a real jwt token
    // TODO: set a max age and use refresh tokens
    let cookie = Cookie::build(("token", "user-1.exp.sign"))
        .path("/")
        .http_only(true)
        .build();
    cookies.add(cookie);

    // TODO: check, maybe there is a status code for login better than 200
    Ok(Json("success"))
}
