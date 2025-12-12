use crate::{
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    models::{ModelManager, dtos::UserPersonalInfo},
};
use axum::{Json, Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/me", get(get_personal_info))
}

async fn get_personal_info(
    auth_token: AuthToken,
    State(model_manager): State<ModelManager>,
) -> Result<Json<UserPersonalInfo>, ServerError> {
    // TODO: `UserPersonalInfo` is the type returned form the contoller and the api schema
    // i am not usre what to do. should i create two identical types (one in dtos and one in
    // api_schemas), or should i keep one (but where should i place it? ig api_schemas)
    let user = UserController::get_personal_info_by_id(&model_manager, auth_token.user_id).await?;
    Ok(Json(user))
}
