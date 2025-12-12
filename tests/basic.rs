mod utils;

use axum::http::StatusCode;
use rand::distr::{Alphabetic, SampleString};

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

#[tokio::test]
async fn ping() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let response = client.do_get("/ping").await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.text_body().unwrap();

    assert_eq!(status, StatusCode::OK, "response body:\n{response_body:#}");
    assert!(content_type.starts_with("text/plain"));
    assert_eq!(response_body, "pong!");
}

#[tokio::test]
async fn fallback() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let endpoint = Alphabetic.sample_string(&mut rand::rng(), 16);
    let response = client.do_get(&format!("/{endpoint}")).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();

    assert!(content_type.starts_with("text/plain"));
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        response.text_body().unwrap(),
        format!("The specified endpoint `GET /{endpoint}` is not found.")
    );
}
