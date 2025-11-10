use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    // TODO: make sure this is not serialized, maybe use a secret crate to wrap the password, or
    // add a serder attr to skip serializing this field
    pub password_hash: String,
    pub password_salt: String,
}

#[derive(FromRow)]
pub struct UserForLogin {
    pub id: Uuid,
    pub password_hash: String,
    pub password_salt: String,
}

pub trait FromUser {}
impl FromUser for User {}
impl FromUser for UserForLogin {}
