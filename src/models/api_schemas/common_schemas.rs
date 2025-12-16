use crate::models::tables::{BusinessProfile, PlayerProfile};
use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "account_type", content = "profile", rename_all = "snake_case")]
pub enum UserProfile {
    Player(PlayerProfile),
    Business(BusinessProfile),
    Admin,
}
