use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    // TODO: consider using validator like: #[validate(length(min = 8))]
    pub password: String, // TODO: use a secret crate to wrap the password
}
