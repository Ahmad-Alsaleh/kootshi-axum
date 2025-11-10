use crate::{
    configs::config,
    controllers::UserController,
    errors::ServerError,
    extractors::JwtToken,
    models::{LoginPayload, ModelManager, UserForLogin},
    secrets::SecretManager,
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
    let user =
        UserController::get_by_username::<UserForLogin>(&model_manager, &login_payload.username)
            .await
            .map_err(|err| ServerError::DataBase(err.to_string()))?;

    let Some(user) = user else {
        return Err(ServerError::UsernameNotFound);
    };

    let password_hash = base64_url::decode(&user.password_hash).map_err(ServerError::Base64)?;
    SecretManager::verify_secret(
        login_payload.password,
        user.password_salt,
        &config().password_key,
        &password_hash,
    )
    .map_err(|_| ServerError::WrongPassword)?;

    let jwt_token = JwtToken::new(user.id);
    let jwt_encoded_token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &jwt_token,
        &EncodingKey::from_secret(&config().jwt_key),
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
