use crate::errors::{ServerError, error_impl};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    UserNotFound,
    InvalidUsernameOrPassword,
    LoginNeeded,
    DatabaseError,
    UsernameAlreadyExists,
    BusinessDisplayNameAlreadyExists,
    PasswordAndConfirmPasswordAreDifferent,
    AdminCannotCreateAccount,
    ThisOperationIsForAdminsOnly,
}

error_impl!(ClientError);

impl From<&ServerError> for ClientError {
    fn from(server_error: &ServerError) -> Self {
        match server_error {
            ServerError::UserIsNotAdmin => Self::ThisOperationIsForAdminsOnly,
            ServerError::UserNotFound => Self::UserNotFound,
            ServerError::InvalidUsernameForLogin | ServerError::WrongPassword => {
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
