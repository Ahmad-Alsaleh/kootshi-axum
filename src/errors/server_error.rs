use crate::errors::error_impl;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use base64_url::base64::DecodeError;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ServerError {
    UsernameNotFound,
    WrongPassword,
    JwtError(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
    JwtTokenNotFoundInCookies,
    DataBase(String),
    Base64(#[serde_as(as = "DisplayFromStr")] DecodeError),
}

error_impl!(ServerError);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::UsernameNotFound
            | Self::WrongPassword
            | Self::JwtError(_)
            | Self::JwtTokenNotFoundInCookies => StatusCode::UNAUTHORIZED,
            Self::DataBase(_) | ServerError::Base64(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
