use crate::models::tables::UserRole;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetUserInfoQuery {
    pub user_role: UserRole,
}
