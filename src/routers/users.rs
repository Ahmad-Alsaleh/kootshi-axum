use crate::{
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::{UserPersonalInfo, UserProfile},
    },
};
use axum::{Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/me", get(get_personal_info))
}

async fn get_personal_info(
    auth_token: AuthToken,
    State(model_manager): State<ModelManager>,
) -> Result<UserPersonalInfo, ServerError> {
    let user_info = UserController::get_personal_info_by_id(
        &model_manager,
        auth_token.user_id,
        auth_token.user_role,
    )
    .await?;

    let profile = match user_info.profile {
        crate::controllers::UserProfile::Player(profile) => UserProfile::Player(profile),
        crate::controllers::UserProfile::Business(profile) => UserProfile::Business(profile),
        crate::controllers::UserProfile::Admin => UserProfile::Admin,
    };
    let user_info = UserPersonalInfo {
        id: user_info.id,
        username: user_info.username,
        profile,
    };

    Ok(user_info)
}
