use crate::{
    configs::config,
    controllers::UserController,
    errors::ServerError,
    extractors::JwtToken,
    models::{LoginPayload, ModelManager},
};
use axum::{Json, Router, extract::State, routing::post};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/login", post(login))
}

// TODO: allow the client to optionally pass a login token instead of the username (?) and password.
// TODO: read online what happens if someone steals the jwt auth-token and uses it to login on a different device.
// TODO: use HTTPS (useful for login)
async fn login(
    State(model_manager): State<ModelManager>,
    cookies: Cookies,
    Json(login_payload): Json<LoginPayload>,
) -> Result<Json<Value>, ServerError> {
    let user = UserController::get_by_login_payload(&model_manager, login_payload)
        .await
        .map_err(|err| ServerError::DataBase(err.to_string()))?;

    let Some(user) = user else {
        return Err(ServerError::WrongLoginCredentials);
    };

    let jwt_token = JwtToken::new(user.id);
    let jwt_encoded_token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &jwt_token,
        &EncodingKey::from_secret(&config().jwt_secret),
    )
    .map_err(ServerError::JwtError)?;

    let response = json!({
        "token": jwt_encoded_token
    });

    // TODO: set a max age and use refresh tokens
    // TODO: explicitly set all security-critical fields of the cookie
    let cookie = Cookie::build(("auth-token", jwt_encoded_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax) // TODO: should this be strict?
        .build();
    cookies.add(cookie);

    Ok(Json(response))
}

// TODO: add a /signup endpoint
