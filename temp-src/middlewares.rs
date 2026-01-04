use crate::{
    errors::{ClientError, ServerError},
    extractors::AuthToken,
    models::RequestLogInfo,
};
use axum::{
    Extension, Json,
    extract::Request,
    http::{Method, Uri},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

pub async fn authenticate(
    auth_token: Result<AuthToken, ServerError>,
    request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    match auth_token {
        Ok(_) => Ok(next.run(request).await),
        Err(err) => Err(err),
    }
}
