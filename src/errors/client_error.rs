use crate::errors::{ServerError, error_impl};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    InvalidUsernameOrPassword,
    InvalidUsername,
    LoginNeeded,
    DatabaseError,
    UsernameAlreadyExists,
    PasswordAndConfirmPasswordAreDifferent,
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
            ServerError::DataBase(_) | ServerError::UnexpectedNullValueFetchedFromDb { .. } => {
                Self::DatabaseError
            }
            ServerError::PasswordAndConfirmPasswordAreDifferent => {
                Self::PasswordAndConfirmPasswordAreDifferent
            }
            ServerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
        }
    }
}
