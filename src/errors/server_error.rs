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
    BusinessDisplayNameAlreadyExists,
    AuthTokenErr(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
    AuthTokenNotFoundInCookies,
    DataBase(String),
    AdminCannotSignup,
}

error_impl!(ServerError);

impl From<UserControllerError> for ServerError {
    fn from(user_controller_error: UserControllerError) -> Self {
        match user_controller_error {
            UserControllerError::UserNotFound => Self::UsernameNotFound,
            UserControllerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
            UserControllerError::BusinessDisplayNameAlreadyExists => {
                Self::BusinessDisplayNameAlreadyExists
            }
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
            // TODO: check if there is a better status code for admin can't signup (eg: forbidin,
            // unautherized,bad request!!,not acceptatle!!,misdirected request, unprocessable
            // entity)
            ServerError::PasswordAndConfirmPasswordAreDifferent
            | ServerError::UsernameNotFound
            | ServerError::AdminCannotSignup => StatusCode::BAD_REQUEST,
            ServerError::UsernameAlreadyExists | ServerError::BusinessDisplayNameAlreadyExists => {
                StatusCode::CONFLICT
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
