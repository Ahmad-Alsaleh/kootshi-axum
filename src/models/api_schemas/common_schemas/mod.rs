use crate::models::tables::{BusinessProfile, PlayerProfile};
use serde::{Deserialize, Serialize};

pub mod _bookings_post_request;
pub mod booking;
pub mod pitch;
pub mod review;
pub mod time_slot;
pub mod user;

// TODO: remove me
// pub use self::_bookings_post_request::BookingsPostRequest;
// pub use self::booking::Booking;
// pub use self::pitch::Pitch;
// pub use self::review::Review;
// pub use self::time_slot::TimeSlot;
// pub use self::user::User;

#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Serialize, Deserialize)]
#[serde(
    tag = "account_type",
    content = "profile",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum UserProfile {
    Player(PlayerProfile),
    Business(BusinessProfile),
    Admin,
}
