use crate::{
    configs::config,
    controllers::{UserController, UserForInsert},
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::{LoginPayload, SignupPayload, UserProfile},
    },
    secrets::SecretManager,
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
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

    let auth_token = AuthToken::new(user_login_info.id, user_login_info.role);
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

// TODO: test this function
async fn signup(
    State(model_manager): State<ModelManager>,
    Json(signup_payload): Json<SignupPayload>,
) -> Result<impl IntoResponse, ServerError> {
    if signup_payload.password != signup_payload.confirm_password {
        return Err(ServerError::PasswordAndConfirmPasswordAreDifferent);
    }

    // TODO: validate the password (length, at least one special char, at least one number, etc.)
    // TODO: validate the username (only letters, numbers, . and _)

    // TODO: use the validation crate, remove ServerError::PasswordAndConfirmPasswordAreDifferent,
    // etc. and use a ServerError::ValidationError(message) which maps to StatusCode::BAD_REQUEST and
    // ClientError::InvalidInput(message)

    let profile = match signup_payload.profile {
        UserProfile::Player {
            first_name,
            last_name,
            preferred_sports,
        } => crate::controllers::UserProfile::Player {
            first_name,
            last_name,
            preferred_sports,
        },
        UserProfile::Business { display_name } => {
            crate::controllers::UserProfile::Business { display_name }
        }
        UserProfile::Admin => crate::controllers::UserProfile::Admin,
    };
    let user = UserForInsert {
        username: &signup_payload.username,
        password: &signup_payload.password,
        profile: &profile,
    };

    let id = UserController::insert_user(&model_manager, user).await?;

    let response = json!({
        "user_id": id
    });

    Ok((StatusCode::CREATED, Json(response)))
}

// TODO: implement /logout
