// use crate::models::request_payloads::AcountType;
use crate::models::tables::Sport;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[cfg_attr(test, derive(Debug))]
#[derive(sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Player,
    Business,
    Admin,
}

#[cfg(test)]
#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: UserRole,
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

// -----

pub struct UserForInsert<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub account_type: &'a AcountType<'a>,
}

pub enum AcountType<'a> {
    Player {
        first_name: &'a str,
        last_name: &'a str,
        // TODO: make sure this is a set (ie items are unique)
        preferred_sports: &'a [Sport],
    },
    Business {
        display_name: &'a str,
    },
}

// -----

pub trait UserFromRow {}

#[cfg(test)]
impl UserFromRow for User {}

impl UserFromRow for UserPersonalInfo {}
impl UserFromRow for UserForLogin {}
impl UserFromRow for UserForUpdatePassword {}
impl UserFromRow for UserForInsert<'_> {}
