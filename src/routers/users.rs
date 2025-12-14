use crate::{
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::{UserPersonalInfo, UserProfile},
    },
};
use axum::{Json, Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/me", get(get_personal_info))
}

async fn get_personal_info(
    auth_token: AuthToken,
    State(model_manager): State<ModelManager>,
) -> Result<Json<UserPersonalInfo>, ServerError> {
    // TODO: make this function use a different sql query based on role (from auth_token). this
    // will remove RawUserInfo struct.
    let user_info =
        UserController::get_personal_info_by_id(&model_manager, auth_token.user_id).await?;

    let profile = match user_info.profile {
        crate::controllers::UserProfile::Player {
            first_name,
            last_name,
            preferred_sports,
        } => UserProfile::Player {
            first_name,
            last_name,
            preferred_sports,
        },
        crate::controllers::UserProfile::Business { display_name } => {
            UserProfile::Business { display_name }
        }
        crate::controllers::UserProfile::Admin => UserProfile::Admin,
    };
    let user_info = UserPersonalInfo {
        id: user_info.id,
        username: user_info.username,
        profile,
    };

    Ok(Json(user_info))
}
