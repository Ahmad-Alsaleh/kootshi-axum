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
use uuid::Uuid;

pub async fn generate_request_id(mut response: Response) -> Response {
    let request_id = Uuid::new_v4();
    response.extensions_mut().insert(request_id);
    response
}

pub async fn map_response(response: Response) -> Response {
    // change the response body if there is a server error
    if let Some(&server_error) = response.extensions().get::<ServerError>() {
        let client_error = ClientError::from(server_error);
        let request_id = response
            .extensions()
            .get::<Uuid>()
            .expect("request_id is inserted in a previous middleware");
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

pub async fn log_response(method: Method, uri: Uri, response: Response) -> Response {
    let request_id = *response
        .extensions()
        .get::<Uuid>()
        .expect("request_id is inserted in a previous middleware");

    let server_error = response.extensions().get::<ServerError>().copied();
    let client_error = response.extensions().get::<ClientError>().copied();

    let log_line = RequestLogInfo::new(
        request_id,
        &uri,
        &method,
        response.status(),
        server_error,
        client_error,
    );
    println!("{}", json!(log_line));

    response
}
