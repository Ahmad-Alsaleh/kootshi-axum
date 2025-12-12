use crate::models::tables::{Sport, UserRole};
use sqlx::FromRow;
use uuid::Uuid;

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
