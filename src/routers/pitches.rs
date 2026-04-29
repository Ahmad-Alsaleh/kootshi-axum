use crate::{
    controllers::PitchController,
    errors::ServerError,
    extractors::AuthToken,
    models::{
        ModelManager,
        api_schemas::responses::{GetPitchesResponse, Pitch},
        tables::UserRole,
    },
};
use axum::{Router, extract::State, routing::get};

pub fn get_router() -> Router<ModelManager> {
    Router::new().route("/", get(get_pitches))
}

async fn get_pitches(
    auth_token: Option<AuthToken>,
    State(model_manager): State<ModelManager>,
) -> Result<GetPitchesResponse, ServerError> {
    // let pitches = match auth_token {
    //     Some(AuthToken {
    //         user_id,
    //         user_role: UserRole::Business,
    //         ..
    //     }) => PitchController::get_pitches_by_business_id(user_id, &model_manager).await?,
    //     _ => PitchController::get_all_pitches(&model_manager).await?,
    // };
    //

    let pitches = vec![
        Pitch {
            id: String::from("pitch-1"),
            name: String::from("Al Forsan"),
            sport: String::from("football"),
            location: String::from("Sharjah, UAE"),
            address: String::from("pitch_one_address"),
            price_per_hour: 249.99,
            rating: 3.1,
            review_count: 42,
            image_url: String::from(
                "https://unsplash.com/photos/group-of-people-playing-soccer-on-soccer-field-8-s5QuUBtyM",
            ),
            amenities: vec![String::from("water"), String::from("lights")],
            description: String::from("pitch-1 description"),
            owner_name: String::from("Alsaleh"),
            owner_avatar: String::from(
                "https://unsplash.com/photos/group-of-people-playing-soccer-on-soccer-field-8-s5QuUBtyM",
            ),
        },
        Pitch {
            id: todo!(),
            name: todo!(),
            sport: todo!(),
            location: todo!(),
            address: todo!(),
            price_per_hour: todo!(),
            rating: todo!(),
            review_count: todo!(),
            image_url: todo!(),
            amenities: todo!(),
            description: todo!(),
            owner_name: todo!(),
            owner_avatar: todo!(),
        },
    ];

    Ok(GetPitchesResponse(pitches))
}
