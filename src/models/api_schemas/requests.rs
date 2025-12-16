use crate::models::api_schemas::UserProfile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
    pub confirm_password: String, // TODO: use a secret crate to wrap the password
    #[serde(flatten)]
    pub profile: UserProfile,
}
