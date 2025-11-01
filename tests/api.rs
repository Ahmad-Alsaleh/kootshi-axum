use axum::http::StatusCode;
use serde_json::json;

const DEV_BASE_URL: &str = "http://127.0.0.1:1936";

#[tokio::test]
async fn index() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let response = client.do_get("/").await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.text_body().unwrap();

    assert_eq!(status, StatusCode::OK);
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

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.starts_with("text/plain"));
    assert_eq!(response_body, "pong!");
}

#[tokio::test]
async fn fallback() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let response = client.do_get("/does-not-exist").await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();

    assert!(content_type.starts_with("text/plain"));
    assert_eq!(status, StatusCode::NOT_FOUND);
}

// TODO: for the login endponit...
// TODO: test client cookies, etc.
// TODO: test cookies if we login then make another api call

#[tokio::test]
async fn login_success() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let request_body = json!({"username": "demo", "password": "password"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.json_body().unwrap();

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.starts_with("application/json"));
    assert_eq!(response_body, json!("success"));
}

#[tokio::test]
async fn login_wrong_body() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let request_body = json!("wrong-body");
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn login_wrong_credentials() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let request_body = json!({"username": "demo", "password": "wrong-password"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.json_body().unwrap();

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(content_type.starts_with("application/json"));
    assert_eq!(
        response_body,
        json!({
            "status": StatusCode::FORBIDDEN.as_u16(),
            "message": "invalid_username_or_password",
            "request_id": response_body.get("request_id"),
        })
    );
}
