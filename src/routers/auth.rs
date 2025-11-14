use crate::{
    configs::config,
    controllers::UserController,
    errors::ServerError,
    extractors::JwtToken,
    models::{
        LoginPayload, ModelManager, SignupPayload, UpdatePasswordPayload, UserForInsertUser,
        UserForLogin,
    },
    secrets::SecretManager,
};
use axum::{Json, Router, extract::State, routing::post};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie::SameSite};

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        // TODO: check if using a hyphen is a good practice in RESTful APIs
        .route("/update-password", post(update_password))
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
            .await?;

    SecretManager::verify_secret(
        &login_payload.password,
        &user.password_salt,
        &config().password_key,
        &user.password_hash,
    )?;

    let jwt_token = JwtToken::new(user.id);
    let jwt_encoded_token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &jwt_token,
        &EncodingKey::from_secret(&config().jwt_key),
    )?;

    let response = json!({
        "token": jwt_encoded_token
    });

    // TODO: set a max age and use refresh tokens
    // TODO: explicitly set all security-critical fields of the cookie
    // TODO: consider using a jwt_salt/auth_token_salt
    let cookie = Cookie::build(("auth-token", jwt_encoded_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax) // TODO: should this be strict?
        .build();
    cookies.add(cookie);

    Ok(Json(response))
}

async fn signup(
    State(model_manager): State<ModelManager>,
    Json(signup_payload): Json<SignupPayload>,
) -> Result<Json<&'static str>, ServerError> {
    if signup_payload.password != signup_payload.confirm_password {
        return Err(ServerError::PasswordAndConfirmPasswordAreDifferent);
    }

    // TODO: validate the password (length, at least one special char, at least one number, etc.)
    // TODO: validate the username (only letters, numbers, . and _)

    // TODO: use the validation crate, remove ServerError::PasswordAndConfirmPasswordAreDifferent,
    // etc. and use a ServerError::ValidationError(message) which maps to StatusCode::BAD_REQUEST and
    // ClientError::InvalidInput(message)

    let user = UserForInsertUser {
        username: signup_payload.username,
        password: signup_payload.password,
        first_name: signup_payload.first_name,
        last_name: signup_payload.last_name,
    };

    UserController::insert_user(&model_manager, user).await?;

    Ok(Json("success"))
}

async fn update_password(
    State(model_manager): State<ModelManager>,
    Json(update_password_payload): Json<UpdatePasswordPayload>,
) -> Result<Json<&'static str>, ServerError> {
    if update_password_payload.new_password != update_password_payload.confirm_new_password {
        return Err(ServerError::PasswordAndConfirmPasswordAreDifferent);
    }

    // TODO: validate the password (length, at least one special char, at least one number, etc.)

    UserController::update_password_by_username(
        &model_manager,
        &update_password_payload.username,
        &update_password_payload.new_password,
    )
    .await?;

    Ok(Json("success"))
}
