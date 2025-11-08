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
    JwtTokenNotFoundInCookies,
    DataBase(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::WrongLoginCredentials | Self::JwtError(_) | Self::JwtTokenNotFoundInCookies => {
                StatusCode::UNAUTHORIZED
            }
            Self::DataBase(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
