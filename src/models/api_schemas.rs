use crate::models::tables::{BusinessProfile, PlayerProfile};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize)]
pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    #[serde(flatten)]
    pub profile: UserProfile,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "account_type", content = "profile", rename_all = "snake_case")]
pub enum UserProfile {
    Player(PlayerProfile),
    Business(BusinessProfile),
    Admin,
}
