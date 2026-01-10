use crate::models::{
    api_schemas::{
        common_schemas::UserProfile, impl_into_response_with_json_body,
        impl_into_response_with_no_body,
    },
    tables::Pitch,
};
use axum::http::StatusCode;
use serde::Serialize;
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

#[derive(Serialize)]
pub struct GetPitchesResponse(pub Vec<Pitch>);

impl_into_response_with_json_body!(GetUserPersonalInfoResponse);
impl_into_response_with_json_body!(LoginResponse);
impl_into_response_with_json_body!(SignupResponse, StatusCode::CREATED);
impl_into_response_with_no_body!(UpdateUserInfoResponse, StatusCode::NO_CONTENT);
impl_into_response_with_json_body!(GetPitchesResponse);
