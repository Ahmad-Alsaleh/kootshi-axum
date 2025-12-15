use serde::{Deserialize, Serialize};
#[cfg(test)]
use sqlx::FromRow;
use uuid::Uuid;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Player,
    Business,
    Admin,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "sport", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Sport {
    Football,
    Padel,
    Basketball,
}

#[allow(unused)]
#[cfg_attr(test, derive(FromRow))]
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
#[cfg_attr(test, derive(FromRow))]
pub struct PlayerProfile {
    pub first_name: String,
    pub last_name: String,
    // TODO: make sure this is a set (ie items are unique) in the DB
    pub preferred_sports: Vec<Sport>,
}

#[allow(unused)]
#[cfg_attr(test, derive(FromRow))]
pub struct BusinessProfile {
    pub display_name: String,
}
