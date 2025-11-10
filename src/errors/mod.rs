mod client_error;
mod server_error;

pub use client_error::ClientError;
pub use server_error::ServerError;

macro_rules! error_impl {
    ($type: ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }
        impl std::error::Error for $type {}
    };
}

pub(crate) use error_impl;
