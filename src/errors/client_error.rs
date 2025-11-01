use crate::errors::ServerError;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientError {
    InvalidUsernameOrPassword,
}

impl From<ServerError> for ClientError {
    fn from(server_error: ServerError) -> Self {
        match server_error {
            ServerError::WrongLoginCredentials => Self::InvalidUsernameOrPassword,
        }
    }
}
