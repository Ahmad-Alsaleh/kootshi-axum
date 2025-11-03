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
    WrongLoginCredentials,
    JwtError(#[serde_as(as = "DisplayFromStr")] jsonwebtoken::errors::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ServerError::WrongLoginCredentials | ServerError::JwtError(_) => StatusCode::FORBIDDEN,
        };
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
