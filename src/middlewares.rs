use crate::{
    MyUuid,
    errors::{ClientError, ServerError},
};
use axum::{
    Json,
    http::{Method, Uri},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

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

    let log_line = RequestLogInfo {
        request_id: MyUuid::new_v4(),
        timestamp: get_millis_since_epoch(),
        path: uri.path(),
        method: method.as_str(),
        status_code: response.status().as_u16(),
        server_error,
        client_error,
    };
    println!("{}", json!(log_line));

    response
}

// TODO: avoid serializing nones
#[derive(Serialize)]
pub struct RequestLogInfo<'a> {
    request_id: MyUuid, // TODO: use uuid
    // user_id // TODO
    timestamp: u128, // millis since epoch // TODO: find a better, idiomatic way to represent this
    path: &'a str,
    method: &'a str,
    status_code: u16,
    server_error: Option<ServerError>,
    client_error: Option<ClientError>,
}

fn get_millis_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("UNIX_EPOCH is earlier than `now`")
        .as_millis()
}
