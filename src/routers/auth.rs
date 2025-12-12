use crate::{
    configs::config,
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    models::{ModelManager, api_schemas::auth::LoginPayload},
    secrets::SecretManager,
};
use axum::{Json, Router, extract::State, routing::post};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/login", post(login))
}

// TODO: allow the client to optionally pass a login token instead of the username (?) and password (e.g. a bearer)
// TODO: use HTTPS (useful for login)
async fn login(
    State(model_manager): State<ModelManager>,
    cookies: Cookies,
    Json(login_payload): Json<LoginPayload>,
) -> Result<Json<Value>, ServerError> {
    let user_login_info =
        UserController::get_login_info_by_username(&model_manager, &login_payload.username).await?;

    SecretManager::verify_secret(
        &login_payload.password,
        &user_login_info.password_salt,
        &config().password_key,
        &user_login_info.password_hash,
    )?;

    let auth_token = AuthToken::new(user_login_info.id);
    let encoded_auth_token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &auth_token,
        &EncodingKey::from_secret(&config().auth_token_key),
    )?;

    let response = json!({
        "auth_token": encoded_auth_token
    });

    // TODO: set a max age and use refresh tokens
    // TODO: explicitly set all security-critical fields of the cookie
    // TODO: consider using an auth_token_salt
    let cookie = Cookie::build(("auth-token", encoded_auth_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax) // TODO: should this be strict?
        .build();
    cookies.add(cookie);

    Ok(Json(response))
}
