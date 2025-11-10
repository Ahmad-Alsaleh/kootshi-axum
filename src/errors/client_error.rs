use crate::errors::ServerError;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    InvalidUsernameOrPassword,
    LoginNeeded,
    FailedToRetrieveData,
    UnknownError,
}

impl From<&ServerError> for ClientError {
    fn from(server_error: &ServerError) -> Self {
        match server_error {
            ServerError::UsernameNotFound | ServerError::WrongPassword => {
                Self::InvalidUsernameOrPassword
            }
            ServerError::JwtError(_) | ServerError::JwtTokenNotFoundInCookies => Self::LoginNeeded,
            ServerError::DataBase(_) => Self::FailedToRetrieveData,
            ServerError::Base64(_) => Self::UnknownError,
        }
    }
}
