use crate::{
    errors::{ClientError, ServerError},
    extractors::JwtToken,
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

pub async fn generate_request_id(mut request: Request) -> Request {
    let request_id = Uuid::new_v4();
    request.extensions_mut().insert(request_id);
    request
}

pub async fn authenticate(
    jwt_token: Result<JwtToken, ServerError>,
    request: Request,
    next: Next,
) -> Result<Response, ServerError> {
    match jwt_token {
        Ok(_) => Ok(next.run(request).await),
        Err(err) => Err(err),
    }
}

pub async fn map_response(Extension(request_id): Extension<Uuid>, response: Response) -> Response {
    // change the response body if there is a server error
    if let Some(server_error) = response.extensions().get::<ServerError>() {
        let client_error = ClientError::from(server_error);
        let body = json!({
            "request_id": request_id,
            "status": response.status().as_u16(),
            "message": client_error,
        });

        let (parts, _body) = response.into_parts();
        let mut response = (parts.status, Json(body)).into_response();
        response.extensions_mut().insert(client_error);
        response.extensions_mut().extend(parts.extensions);
        response
    } else {
        response
    }
}

pub async fn log_response(
    Extension(request_id): Extension<Uuid>,
    method: Method,
    uri: Uri,
    response: Response,
) -> Response {
    let server_error = response.extensions().get::<ServerError>();
    let client_error = response.extensions().get::<ClientError>().copied();

    let log_line = RequestLogInfo::new(
        request_id,
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
