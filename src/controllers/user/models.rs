use crate::models::tables::{Sport, UserRole};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct RawUserPersonalInfo {
    // core user profile
    pub username: String,
    pub role: UserRole,

    // player profile
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_sports: Option<Vec<Sport>>,

    // business profile
    pub display_name: Option<String>,
}

pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    pub profile: UserProfile,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UserProfile {
    Player {
        first_name: String,
        last_name: String,
        // TODO: make sure this is a set (ie items are unique)
        preferred_sports: Vec<Sport>,
    },
    Business {
        display_name: String,
    },
    Admin,
}

#[derive(FromRow)]
pub struct UserLoginInfo {
    pub id: Uuid,
    pub role: UserRole,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

pub struct UserForInsert<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub profile: &'a UserProfile,
}
