use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[allow(unused)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Player,
    Business,
    Admin,
}

#[allow(unused)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "sport", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum Sport {
    Football,
    Padel,
    Basketball,
}

#[allow(unused)]
#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    // TODO: make sure this is not serialized,
    // maybe use a secret crate to wrap the password, or
    // add a serder attr to skip serializing this field
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
    pub role: UserRole,
}

#[allow(unused)]
pub struct PlayerProfile {
    first_name: String,
    last_name: String,
    // TODO: make sure this is a set (ie items are unique) in the DB
    preferred_sports: Vec<String>,
}

#[allow(unused)]
pub struct BusinessProfile {
    display_name: String,
}
