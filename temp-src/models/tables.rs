use serde::Deserialize;
use uuid::Uuid;

// #[cfg_attr(test, derive(Debug))]
// #[derive(sqlx::Type)]
// #[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Player,
    Business,
    Admin,
}

// #[derive(sqlx::Type)]
// #[sqlx(type_name = "sport", rename_all = "lowercase")]
#[derive(Deserialize)]
pub enum Sport {
    Football,
    Padel,
    Basketball,
}

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

pub struct PlayerProfile {
    first_name: String,
    last_name: String,
    // TODO: make sure this is a set (ie items are unique) in the DB
    preferred_sports: Vec<String>,
}

pub struct BusinessProfile {
    display_name: String,
}
