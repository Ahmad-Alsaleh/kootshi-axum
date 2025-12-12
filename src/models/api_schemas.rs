use crate::models::tables::Sport;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    // TODO: use a secret crate to wrap the password
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    // TODO: consider using validator like: #[validate(length(min = 8))]
    // TODO: use a secret crate to wrap the password
    pub password: String,
    pub confirm_password: String,
    #[serde(flatten)]
    pub profile_info: ProfileInfo,
}

// TODO: this is used for both requests and response, so maybe rename this file or place this
// struct somewhere else (dto? ig dto is a very good idea)
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "account_type", content = "profile", rename_all = "snake_case")]
pub enum ProfileInfo {
    Player {
        first_name: String,
        last_name: String,
        // TODO: make sure this is a set (ie items are unique)
        preferred_sports: Vec<Sport>,
    },
    Business {
        display_name: String,
    },
    Admin,
}
