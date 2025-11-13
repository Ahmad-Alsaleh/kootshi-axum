use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    // TODO: consider using validator like: #[validate(length(min = 8))]
    pub password: String, // TODO: use a secret crate to wrap the password
    pub confirm_password: String, // TODO: use a secret crate to wrap the password
    pub first_name: Option<String>,
    pub last_name: Option<String>,
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
