use crate::errors::{ServerError, error_impl};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    InvalidUsernameOrPassword,
    LoginNeeded,
    DatabaseError,
    UsernameAlreadyExists,
    BusinessDisplayNameAlreadyExists,
    PasswordAndConfirmPasswordAreDifferent,
    AdminCannotCreateAccount,
}

error_impl!(ClientError);

impl From<&ServerError> for ClientError {
    fn from(server_error: &ServerError) -> Self {
        match server_error {
            ServerError::UsernameNotFound | ServerError::WrongPassword => {
                Self::InvalidUsernameOrPassword
            }
            ServerError::AuthTokenErr(_) | ServerError::AuthTokenNotFoundInCookies => {
                Self::LoginNeeded
            }
            ServerError::DataBase(_) => Self::DatabaseError,
            ServerError::PasswordAndConfirmPasswordAreDifferent => {
                Self::PasswordAndConfirmPasswordAreDifferent
            }
            ServerError::UsernameAlreadyExists => Self::UsernameAlreadyExists,
            ServerError::BusinessDisplayNameAlreadyExists => Self::BusinessDisplayNameAlreadyExists,
            ServerError::AdminCannotSignup => Self::AdminCannotCreateAccount,
        }
    }
}
