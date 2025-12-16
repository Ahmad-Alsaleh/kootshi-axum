use crate::models::api_schemas::{UserProfile, impl_into_response};
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

impl_into_response!(UserPersonalInfo);
impl_into_response!(LoginResponse);
impl_into_response!(SignupResponse, StatusCode::CREATED);
