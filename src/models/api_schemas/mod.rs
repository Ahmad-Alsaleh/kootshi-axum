mod common_schemas;
mod requests;
mod responses;

pub use common_schemas::UserProfile;
pub use requests::{LoginPayload, SignupPayload};
pub use responses::{LoginResponse, SignupResponse, UserPersonalInfo};

macro_rules! impl_into_response {
    ($type:ty) => {
        impl axum::response::IntoResponse for $type {
            fn into_response(self) -> axum::response::Response {
                axum::Json(self).into_response()
            }
        }
    };

    ($type:ty, $status_code:expr) => {
        impl axum::response::IntoResponse for $type {
            fn into_response(self) -> axum::response::Response {
                ($status_code, axum::Json(self)).into_response()
            }
        }
    };
}
pub(crate) use impl_into_response;
