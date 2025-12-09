use crate::models::tables::Sport;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    // TODO: consider using validator like: #[validate(length(min = 8))]
    // TODO: use a secret crate to wrap the password
    pub password: String,
    pub confirm_password: String,
    pub account_type: AcountType,
}

#[derive(Deserialize)]
#[serde(tag = "account_type", content = "profile", rename_all = "snake_case")]
pub enum AccountType {
    Player {
        first_name: String,
        last_name: String,
        // TODO: make sure this is a set (ie items are unique)
        preferred_sports: Vec<Sport>,
    },
    Business {
        display_name: String,
    },
}
