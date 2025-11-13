use axum::http::StatusCode;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

// TODO: test the output (logs) of the server. ig a good way of doing it is by running the server
// in a command and capturing stdout. but ig it is better to put these tests in
// tests/server-logs.rs

const DEV_BASE_URL: &str = "http://localhost:1948";

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
    assert_eq!(
        response.text_body().unwrap(),
        "The specified endpoint `GET /does-not-exist` is not found."
    );
}

// TODO: for the login endponit, test client cookies, etc. test cookies if we login then make another api call
// an easy and (imo) good way is to make a /protected-ping endpoint (follow restful naming conventions) for tests only that will return `pong!`

// TODO: test calling a protected endpoint without auth

// TODO: test if the user hits an auth-needed endpoint after the jwt token expires
// ig an easy way to do this is to make exp configurable and have two configs, prod and dev
// or to set the value of env var at run time. or, ig .cargo/config.toml allows u to set env for a specific profile (test in this case)

// TODO: test something like `curl localhost:1948/companies -H 'Cookie: token=<JWT_TOKEN>'` (where the jwt token can be obtained by hitting /login)
// i.e. pass the cookie manually, instead of using the client's cached cookies. one easy wat to do
// this is to hit /login, save the cookie, create a new client, hit a /protected-ping and test if
// it passes

#[tokio::test]
async fn login_success() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
    let request_body = json!({"username": "ahmad.alsaleh", "password": "passme"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.json_body().unwrap();
    let set_cookie_header = response.header("set-cookie").unwrap();

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.starts_with("application/json"));
    assert!(
        response_body
            .get("token")
            .map(|token| Uuid::from_str(token.as_str().unwrap()))
            .is_some()
    );
    assert!(set_cookie_header.starts_with("auth-token="));
    assert!(client.cookie("auth-token").is_some());
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
    let request_body = json!({"username": "ahmad.alsaleh", "password": "wrong-password"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.json_body().unwrap();

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(content_type.starts_with("application/json"));
    assert_eq!(
        response_body,
        json!({
            "status": StatusCode::UNAUTHORIZED.as_u16(),
            "message": "invalid_username_or_password",
            "request_id": response_body.get("request_id").unwrap(),
        })
    );
}
