use crate::{
    controllers::PitchController,
    errors::ServerError,
    extractors::AuthToken,
    models::{ModelManager, api_schemas::responses::GetPitchesResponse, tables::UserRole},
};
use axum::{Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/", get(get_pitches))
}

async fn get_pitches(
    auth_token: Option<AuthToken>,
    State(model_manager): State<ModelManager>,
) -> Result<GetPitchesResponse, ServerError> {
    let pitches = match auth_token {
        Some(AuthToken {
            user_id,
            user_role: UserRole::Business,
            ..
        }) => PitchController::get_pitches_by_business_id(user_id, &model_manager).await?,
        _ => PitchController::get_all_pitches(&model_manager).await?,
    };

    Ok(GetPitchesResponse(pitches))
}
