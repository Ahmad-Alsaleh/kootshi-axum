use crate::errors::{ClientError, ServerError};
use axum::http::{Method, StatusCode, Uri};
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Serialize)]
#[skip_serializing_none]
pub struct RequestLogInfo<'a> {
    pub request_id: Uuid,
    // pub user_id // TODO
    // TODO: find a better, idiomatic way to represent this
    timestamp: u128, // millis since epoch
    path: &'a str,
    method: &'a str,
    status_code: u16,
    server_error: Option<ServerError>,
    client_error: Option<ClientError>,
}

impl<'r> RequestLogInfo<'r> {
    pub fn new(
        uri: &'r Uri,
        method: &'r Method,
        status_code: StatusCode,
        server_error: Option<ServerError>,
        client_error: Option<ClientError>,
    ) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            timestamp: get_millis_since_epoch(),
            path: uri.path(),
            method: method.as_str(),
            status_code: status_code.as_u16(),
            server_error,
            client_error,
        }
    }
}

fn get_millis_since_epoch() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("UNIX_EPOCH is earlier than `now`")
        .as_millis()
}
