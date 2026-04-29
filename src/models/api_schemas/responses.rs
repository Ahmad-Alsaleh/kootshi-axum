use crate::models::api_schemas::{
    common_schemas::UserProfile, impl_into_response_with_json_body, impl_into_response_with_no_body,
};
use axum::http::StatusCode;
use serde::Serialize;
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[derive(Serialize)]
pub struct GetUserPersonalInfoResponse {
    pub id: Uuid,
    pub username: String,
    #[serde(flatten)]
    pub profile: UserProfile,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub auth_token: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub user_id: Uuid,
}

pub struct UpdateUserInfoResponse;

/////////// ------
#[skip_serializing_none]
#[derive(Serialize)]
// TODO: check if we can use snake_case here and modify the fetching function in JS
#[serde(rename_all = "camelCase")]
pub struct Pitch {
    pub id: String,
    pub name: String,
    pub sport: String,
    pub location: String,
    pub address: String,
    pub price_per_hour: f64,
    pub rating: f64,
    pub review_count: i32,
    pub image_url: String,
    pub amenities: Vec<String>,
    pub description: String,
    pub owner_name: String,
    pub owner_avatar: String,
}
/////////// ------

#[derive(Serialize)]
pub struct GetPitchesResponse(pub Vec<Pitch>);

impl_into_response_with_json_body!(GetUserPersonalInfoResponse);
impl_into_response_with_json_body!(LoginResponse);
impl_into_response_with_json_body!(SignupResponse, StatusCode::CREATED);
impl_into_response_with_no_body!(UpdateUserInfoResponse, StatusCode::NO_CONTENT);
impl_into_response_with_json_body!(GetPitchesResponse);
