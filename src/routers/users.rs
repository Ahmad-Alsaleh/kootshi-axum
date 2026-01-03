use crate::{
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::{
            UpdateUserInfoPayload, UpdateUserInfoResponse, UpdateUserProfilePayload,
            UserPersonalInfo, UserProfile,
        },
    },
};
use axum::{Json, Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/me", get(get_personal_info).patch(update_personal_info))
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

async fn update_personal_info(
    auth_token: AuthToken,
    State(model_manager): State<ModelManager>,
    Json(new_user_info): Json<UpdateUserInfoPayload>,
) -> Result<UpdateUserInfoResponse, ServerError> {
    // TODO: i don't like this conversion, find a design that will fix this somehow (maybe use
    // a DTO with impl From, or use DDD, or keep all models in one place)
    let profile = new_user_info.profile.map(|profile| match profile {
        UpdateUserProfilePayload::Player(profile) => {
            crate::controllers::UpdateUserProfilePayload::Player(
                crate::controllers::UpdatePlayerProfilePayload {
                    first_name: profile.first_name,
                    last_name: profile.last_name,
                    preferred_sports: profile.preferred_sports,
                },
            )
        }
        UpdateUserProfilePayload::Business(profile) => {
            crate::controllers::UpdateUserProfilePayload::Business(
                crate::controllers::UpdateBusinessProfilePayload {
                    display_name: profile.display_name,
                },
            )
        }
    });

    let new_user_info = crate::controllers::UpdateUserInfoPayload {
        username: new_user_info.username,
        password: new_user_info.password,
        profile,
    };

    UserController::update_by_id(
        &model_manager,
        auth_token.user_id,
        auth_token.user_role,
        new_user_info,
    )
    .await?;

    Ok(UpdateUserInfoResponse)
}
