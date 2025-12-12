mod utils;

use axum::http::StatusCode;
use utils::login;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

#[tokio::test]
async fn index() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let response = client.do_get("").await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.text_body().unwrap();

    assert_eq!(status, StatusCode::OK, "response body:\n{response_body:#}");
    assert!(content_type.starts_with("text/html"));
    assert_eq!(response_body, "<h1>Hello <i>World!</i></h1>");
}

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
    let response = client.do_get("/does-not-exist").await.unwrap();

    login!(client);

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();

    assert!(content_type.starts_with("text/plain"));
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(
        response.text_body().unwrap(),
        "The specified endpoint `GET /does-not-exist` is not found."
    );
}
