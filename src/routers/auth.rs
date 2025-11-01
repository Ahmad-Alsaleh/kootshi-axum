use crate::{errors::ServerError, models::LoginPayload};
use axum::{Json, Router, routing::post};

pub fn get_router() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(Json(body): Json<LoginPayload>) -> Result<Json<&'static str>, ServerError> {
    // TODO: use proper auth logic
    if body.username != "demo" || body.password != "password" {
        return Err(ServerError::WrongLoginCredentials);
    }

    // TODO: create a jwt token
    // TODO: set the jwt token as a cookie

    // TODO: check, maybe there is a status code for login better than 200
    Ok(Json("success"))
}
