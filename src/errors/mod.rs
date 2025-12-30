mod client_error;
mod server_error;

pub use client_error::ClientError;
pub use server_error::ServerError;

macro_rules! error_impl {
    ($type:ty) => {
        impl ::core::fmt::Display for $type {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::write!(f, "{self:?}")
            }
        }
        impl ::core::error::Error for $type {}
    };
}

pub(crate) use error_impl;
