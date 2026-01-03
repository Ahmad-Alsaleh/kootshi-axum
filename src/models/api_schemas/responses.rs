use crate::models::api_schemas::{
    UserProfile, impl_into_response_with_json_body, impl_into_response_with_no_body,
};
use axum::http::StatusCode;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserPersonalInfo {
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

impl_into_response_with_json_body!(UserPersonalInfo);
impl_into_response_with_json_body!(LoginResponse);
impl_into_response_with_json_body!(SignupResponse, StatusCode::CREATED);
impl_into_response_with_no_body!(UpdateUserInfoResponse, StatusCode::NO_CONTENT);
