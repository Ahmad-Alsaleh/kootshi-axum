use crate::models::UserRole;
use serde::Deserialize;
use serde_with::rust::double_option;
use validator::Validate;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
}

#[derive(Deserialize)]
pub struct UpdatePasswordPayload {
    pub username: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

#[derive(Deserialize)]
pub struct CreateCompanyPayload {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPersonalInfoPayload {
    pub username: Option<String>,
    #[serde(default, with = "double_option")]
    pub first_name: Option<Option<String>>,
    #[serde(default, with = "double_option")]
    pub last_name: Option<Option<String>>,
}
