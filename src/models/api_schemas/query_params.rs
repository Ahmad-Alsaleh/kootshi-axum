use crate::models::tables::UserRole;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetUserInfoQuery {
    pub user_role: UserRole,
}
