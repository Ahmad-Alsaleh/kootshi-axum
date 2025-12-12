use crate::models::{
    api_schemas::ProfileInfo,
    tables::{Sport, UserRole},
};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserLoginDetails {
    pub id: Uuid,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

pub struct UserForInsert<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub account_type: &'a ProfileInfo,
}

#[derive(Serialize)]
pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    #[serde(flatten)]
    pub profile_info: ProfileInfo,
}

#[derive(FromRow)]
pub struct RawUserInfo {
    pub id: Uuid,
    pub username: String,
    pub role: UserRole,

    // player profile
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub preferred_sports: Option<Vec<Sport>>,

    // business profile
    pub display_name: Option<String>,
}
