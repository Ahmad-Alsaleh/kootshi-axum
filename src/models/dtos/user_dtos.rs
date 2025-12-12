use crate::models::api_schemas::ProfileInfo;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserPersonalInfo {
    pub id: Uuid,
    pub username: String,
    #[serde(flatten)]
    pub profile_info: ProfileInfo,
}
