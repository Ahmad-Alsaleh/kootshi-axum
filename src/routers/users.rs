use crate::{
    controllers::UserController,
    errors::ServerError,
    extractors::AuthToken,
    middlewares,
    models::{
        ModelManager,
        api_schemas::{
            common_schemas::UserProfile,
            query_params::GetUserInfoQuery,
            requests::{UpdateUserInfoPayload, UpdateUserProfilePayload},
            responses::{GetUserPersonalInfoResponse, UpdateUserInfoResponse},
        },
    },
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    middleware,
    routing::get,
};
use uuid::Uuid;

pub fn get_router() -> Router<ModelManager> {
    Router::new()
        .route("/me", get(get_personal_info).patch(update_personal_info))
        .route(
            "/{user_id}",
            get(get_user_info).route_layer(middleware::from_fn(middlewares::authenticate_admin)),
        )
}

async fn get_personal_info(
    auth_token: AuthToken,
    State(model_manager): State<ModelManager>,
) -> Result<GetUserPersonalInfoResponse, ServerError> {
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
    let user_info = GetUserPersonalInfoResponse {
        id: user_info.id,
        username: user_info.username,
        profile,
    };

    Ok(user_info)
}

async fn get_user_info(
    Path(user_id): Path<Uuid>,
    Query(GetUserInfoQuery { user_role }): Query<GetUserInfoQuery>,
    State(model_manager): State<ModelManager>,
) -> Result<GetUserPersonalInfoResponse, ServerError> {
    let user_info =
        UserController::get_personal_info_by_id(&model_manager, user_id, user_role).await?;

    // TODO: consider implementing `impl From<controllers::users::UserPersonalInfo> for api_schemas::UserPersonalInfo `
    let profile = match user_info.profile {
        crate::controllers::UserProfile::Player(profile) => UserProfile::Player(profile),
        crate::controllers::UserProfile::Business(profile) => UserProfile::Business(profile),
        crate::controllers::UserProfile::Admin => UserProfile::Admin,
    };
    let user_info = GetUserPersonalInfoResponse {
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
