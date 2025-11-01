use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Clone, Copy, Debug)]
pub enum ServerError {
    WrongLoginCredentials,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status_code = match self {
            ServerError::WrongLoginCredentials => StatusCode::FORBIDDEN,
        };
        let mut response = status_code.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
