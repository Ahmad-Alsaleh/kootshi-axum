use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String, // TODO: use a secret crate to wrap the password
}
