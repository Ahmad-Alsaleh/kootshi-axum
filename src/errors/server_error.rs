use crate::{
    controllers::{CompanyControllerError, UserControllerError},
    errors::error_impl,
    secrets::SecretDoesNotMatchTarget,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ServerError {
    UsernameNotFound,
    WrongPassword,
    PasswordAndConfirmPasswordAreDifferent,
    UsernameAlreadyExists,
    JwtError(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
    JwtTokenNotFoundInCookies,
    DataBase(String),
    CompanyNameAlreadyExists,
}

error_impl!(ServerError);

impl From<UserControllerError> for ServerError {
    fn from(user_controller_error: UserControllerError) -> Self {
        match user_controller_error {
            UserControllerError::UserNotFound => Self::UsernameNotFound,
            UserControllerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
            UserControllerError::Sqlx(err) => Self::DataBase(err.to_string()),
        }
    }
}

impl From<CompanyControllerError> for ServerError {
    fn from(company_controller_error: CompanyControllerError) -> Self {
        match company_controller_error {
            CompanyControllerError::CompanyNameAlreadyExists => Self::CompanyNameAlreadyExists,
            CompanyControllerError::Sqlx(err) => Self::DataBase(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ServerError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::JwtError(err)
    }
}

impl From<SecretDoesNotMatchTarget> for ServerError {
    fn from(_err: SecretDoesNotMatchTarget) -> Self {
        Self::WrongPassword
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::UsernameNotFound
            | Self::WrongPassword
            | Self::JwtError(_)
            | Self::JwtTokenNotFoundInCookies => StatusCode::UNAUTHORIZED,
            Self::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordAndConfirmPasswordAreDifferent
            | Self::UsernameAlreadyExists
            | Self::CompanyNameAlreadyExists => StatusCode::BAD_REQUEST,
        };
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
