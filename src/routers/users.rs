use crate::{
    controllers::UserController,
    errors::ServerError,
    models::{ModelManager, UpdatePasswordPayload},
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, put},
};

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/password", put(update_password))
        .route("/{username}", delete(delete_user))
}

// TODO: test this endpoint
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

// TODO: test this endpoint
async fn delete_user(
    State(model_manager): State<ModelManager>,
    Path(username): Path<String>,
) -> Result<StatusCode, ServerError> {
    UserController::delete_by_username(&model_manager, &username).await?;
    Ok(StatusCode::NO_CONTENT)
}
