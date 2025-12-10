use crate::errors::{ServerError, error_impl};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    InvalidUsernameOrPassword,
    InvalidUsername,
    LoginNeeded,
    FailedWhileRetrievingData,
    UsernameAlreadyExists,
    CompanyNameAlreadyExists,
    PasswordAndConfirmPasswordAreDifferent,
    CompanyNotFound,
}

error_impl!(ClientError);

impl From<&ServerError> for ClientError {
    fn from(server_error: &ServerError) -> Self {
        match server_error {
            ServerError::WrongPassword => Self::InvalidUsernameOrPassword,
            ServerError::UsernameNotFound => Self::InvalidUsername,
            ServerError::AuthTokenErr(_) | ServerError::AuthTokenNotFoundInCookies => {
                Self::LoginNeeded
            }
            ServerError::DataBase(_) => Self::FailedWhileRetrievingData,
            ServerError::PasswordAndConfirmPasswordAreDifferent => {
                Self::PasswordAndConfirmPasswordAreDifferent
            }
            ServerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
            ServerError::CompanyNameAlreadyExists => Self::CompanyNameAlreadyExists,
            ServerError::CompanyNotFound => Self::CompanyNotFound,
        }
    }
}
