use crate::models::tables::{BusinessProfile, PlayerProfile, Sport, UserRole};
use sqlx::FromRow;
use uuid::Uuid;

#[cfg_attr(test, derive(Debug))]
pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    pub profile: UserProfile,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UserProfile {
    Player(PlayerProfile),
    Business(BusinessProfile),
    Admin,
}

#[derive(FromRow)]
pub struct RawPlayerUser {
    pub username: String,
    #[sqlx(flatten)]
    pub profile: PlayerProfile,
}

#[derive(FromRow)]
pub struct RawBusinessUser {
    pub username: String,
    #[sqlx(flatten)]
    pub profile: BusinessProfile,
}

#[derive(FromRow)]
pub struct RawAdminUser {
    pub username: String,
}

#[derive(FromRow)]
pub struct UserLoginInfo {
    pub id: Uuid,
    pub role: UserRole,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

// TODO: rename to InsertUserPayload
pub struct InsertUserPayload<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub profile: &'a UserProfile,
}

pub struct UpdateUserInfoPayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub profile: Option<UpdateUserProfilePayload>,
}

pub enum UpdateUserProfilePayload {
    Player(UpdatePlayerProfilePayload),
    Business(UpdateBusinessProfilePayload),
}

pub struct UpdatePlayerProfilePayload {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_sports: Option<Vec<Sport>>,
}

pub struct UpdateBusinessProfilePayload {
    pub display_name: Option<String>,
}
