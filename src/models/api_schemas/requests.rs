use crate::models::{api_schemas::common_schemas::UserProfile, tables::Sport};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignupPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
    pub confirm_password: String, // TODO: use a secret crate to wrap the password
    #[serde(flatten)]
    pub profile: UserProfile,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserInfoPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(flatten)]
    pub profile: Option<UpdateUserProfilePayload>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub enum UpdateUserProfilePayload {
    #[serde(rename = "player_profile")]
    Player(UpdatePlayerProfilePayload),
    #[serde(rename = "business_profile")]
    Business(UpdateBusinessProfilePayload),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePlayerProfilePayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_sports: Option<Vec<Sport>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateBusinessProfilePayload {
    pub display_name: Option<String>,
}
