use crate::models::tables::Sport;
use serde::{Deserialize, Serialize};

// TODO: this is used for both requests and response, so maybe rename this file or place this
// struct somewhere else (dto? ig dto is a very good idea)
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize)]
#[serde(tag = "account_type", content = "profile", rename_all = "snake_case")]
pub enum ProfileInfo {
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
