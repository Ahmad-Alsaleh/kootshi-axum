use crate::{
    errors::{ClientError, ServerError},
    models::RequestLogInfo,
};
use axum::{
    Json,
    http::{Method, Uri},
    response::{IntoResponse, Response},
};
use serde_json::json;

pub async fn map_response(method: Method, uri: Uri, response: Response) -> Response {
    let (server_error, client_error) = response
        .extensions()
        .get::<ServerError>()
        .map(|&server_error| {
            let client_error = ClientError::from(server_error);
            (server_error, client_error)
        })
        .unzip();

    let response = client_error
        .map(|client_error| {
            let body = json!({
                "status": response.status().as_u16(),
                "message": client_error,
            });
            (response.status(), Json(body)).into_response()
        })
        .unwrap_or(response);

    let log_line =
        RequestLogInfo::new(&uri, &method, response.status(), server_error, client_error);
    println!("{}", json!(log_line));

    response
}
