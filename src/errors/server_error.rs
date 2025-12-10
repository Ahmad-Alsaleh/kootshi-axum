use crate::{
    controllers::UserControllerError, errors::error_impl, secrets::SecretDoesNotMatchTarget,
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
    AuthTokenErr(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
    AuthTokenNotFoundInCookies,
    UnexpectedNullValueFetchedFromDb {
        table_name: &'static str,
        column_name: &'static str,
        explanation: &'static str,
    },
    DataBase(String),
}

error_impl!(ServerError);

impl From<UserControllerError> for ServerError {
    fn from(user_controller_error: UserControllerError) -> Self {
        match user_controller_error {
            UserControllerError::UserNotFound => Self::UsernameNotFound,
            UserControllerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
            UserControllerError::UnexpectedNullValueFetchedFromDb {
                table_name,
                column_name,
                explanation,
            } => Self::UnexpectedNullValueFetchedFromDb {
                table_name,
                column_name,
                explanation,
            },
            UserControllerError::Sqlx(err) => Self::DataBase(err.to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ServerError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::AuthTokenErr(err)
    }
}

impl From<SecretDoesNotMatchTarget> for ServerError {
    fn from(_err: SecretDoesNotMatchTarget) -> Self {
        Self::WrongPassword
    }
}

impl From<&ServerError> for StatusCode {
    fn from(server_error: &ServerError) -> Self {
        match server_error {
            ServerError::WrongPassword
            | ServerError::AuthTokenErr(_)
            | ServerError::AuthTokenNotFoundInCookies => StatusCode::UNAUTHORIZED,
            ServerError::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::PasswordAndConfirmPasswordAreDifferent | ServerError::UsernameNotFound => {
                StatusCode::BAD_REQUEST
            }
            ServerError::UsernameAlreadyExists => StatusCode::CONFLICT,
            ServerError::UnexpectedNullValueFetchedFromDb { .. } => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from(&self);
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
