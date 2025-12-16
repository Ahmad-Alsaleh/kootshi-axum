use crate::{
    configs::config,
    controllers::{UserController, UserForInsert},
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::{LoginPayload, LoginResponse, SignupPayload, SignupResponse, UserProfile},
    },
    secrets::SecretManager,
};
use axum::{Json, Router, extract::State, routing::post};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
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
) -> Result<LoginResponse, ServerError> {
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

    // TODO: set a max age and use refresh tokens
    // TODO: explicitly set all security-critical fields of the cookie
    let cookie = Cookie::build(("auth-token", encoded_auth_token.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax) // TODO: should this be strict?
        .build();
    cookies.add(cookie);

    Ok(LoginResponse {
        auth_token: encoded_auth_token,
    })
}

async fn signup(
    State(model_manager): State<ModelManager>,
    Json(signup_payload): Json<SignupPayload>,
) -> Result<SignupResponse, ServerError> {
    if signup_payload.password != signup_payload.confirm_password {
        return Err(ServerError::PasswordAndConfirmPasswordAreDifferent);
    }

    // TODO: validate the password (length, at least one special char, at least one number, etc.)
    // TODO: validate the username (only letters, numbers, . and _)

    // TODO: use the validation crate, remove ServerError::PasswordAndConfirmPasswordAreDifferent,
    // etc. and use a ServerError::ValidationError(message) which maps to StatusCode::BAD_REQUEST and
    // ClientError::InvalidInput(message)

    let profile = match signup_payload.profile {
        UserProfile::Player(profile) => crate::controllers::UserProfile::Player(profile),
        UserProfile::Business(profile) => crate::controllers::UserProfile::Business(profile),
        UserProfile::Admin => return Err(ServerError::AdminCannotSignup),
    };
    let user = UserForInsert {
        username: &signup_payload.username,
        password: &signup_payload.password,
        profile: &profile,
    };

    let user_id = UserController::insert_user(&model_manager, user).await?;

    Ok(SignupResponse { user_id })
}

// TODO: implement /logout. ig for this, i need to use session-based auth (ie, auth-token stores a
// session id instead of the user id, and the DB will store mappings from session ids to user ids)
