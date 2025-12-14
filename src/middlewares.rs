use crate::{
    errors::{ClientError, ServerError},
    extractors::AuthToken,
    logging::RequestLogInfo,
};
use axum::{
    Extension, Json,
    extract::Request,
    http::{Method, Uri},
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;

pub async fn generate_request_id(mut request: Request) -> Request {
    let request_id = Uuid::new_v4();
    request.extensions_mut().insert(request_id);
    request
}

/// Changes the response body if there is a server error
pub async fn insert_response_body_on_error(
    Extension(request_id): Extension<Uuid>,
    response: Response,
) -> Response {
    if let Some(server_error) = response.extensions().get::<ServerError>() {
        let client_error = ClientError::from(server_error);
        let body = json!({
            "request_id": request_id,
            "status": response.status().as_u16(),
            "message": client_error,
        });

        let (parts, _body) = response.into_parts();
        let mut response = (parts.status, Json(body)).into_response();
        response.extensions_mut().extend(parts.extensions);
        response.extensions_mut().insert(client_error);
        response
    } else {
        response
    }
}

pub async fn log_response(
    Extension(request_id): Extension<Uuid>,
    auth_token: Option<AuthToken>,
    method: Method,
    uri: Uri,
    response: Response,
) -> Response {
    let server_error = response.extensions().get::<ServerError>();
    let client_error = response.extensions().get::<ClientError>();
    let (user_id, user_role) = auth_token
        .map(|token| (token.user_id, token.user_role))
        .unzip();

    let log_line = RequestLogInfo::new(
        request_id,
        user_id,
        user_role,
        &uri,
        &method,
        response.status(),
        server_error,
        client_error,
    );

    // TODO: use proper logging
    println!("{}", json!(log_line));

    response
}
