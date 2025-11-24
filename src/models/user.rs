use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[cfg(test)]
#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    // TODO: make sure this is not serialized,
    // maybe use a secret crate to wrap the password, or
    // add a serder attr to skip serializing this field
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

#[derive(Serialize, FromRow)]
pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(FromRow)]
pub struct UserForLogin {
    pub id: Uuid,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

#[derive(FromRow)]
pub struct UserForUpdatePassword {
    pub password_salt: Vec<u8>,
}

// TODO: rename to UserForInsert
pub struct UserForInsertUser {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

pub trait UserFromRow {}

#[cfg(test)]
impl UserFromRow for User {}

impl UserFromRow for UserPersonalInfo {}
impl UserFromRow for UserForLogin {}
impl UserFromRow for UserForUpdatePassword {}
impl UserFromRow for UserForInsertUser {}
