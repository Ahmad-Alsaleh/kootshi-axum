use anyhow::Context;
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

// TODO: test the output (logs) of the server. ig a good way of doing it is by running the server
// in a command and capturing stdout. but ig it is better to put these tests in
// tests/server-logs.rs

const DEV_BASE_URL: &str = "http://localhost:1948";

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
async fn login_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({"username": "ahmad.alsaleh", "password": "passme"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();
    let set_cookie_header = response.header("set-cookie").unwrap();

    // check status code
    assert_eq!(status, 200);

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        auth_token: String,
    }
    let schema = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;
    assert_eq!(schema.auth_token.split('.').count(), 3);

    // check headers
    assert!(set_cookie_header.starts_with("auth-token="));
    assert!(client.cookie("auth-token").is_some());

    Ok(())
}

#[tokio::test]
async fn login_err_username_not_found() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({"username": "invalid_username", "password": "passme"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(status, 400);

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        message: String,
        request_id: Uuid,
        status: u16,
    }
    let schema = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;
    assert_eq!(schema.message, "invalid_username");
    assert_eq!(schema.status, 400);

    // check headers
    assert!(response.header("set-cookie").is_none());

    // check client cookies
    assert!(client.cookie("auth-token").is_none());

    Ok(())
}

#[tokio::test]
async fn login_err_wrong_password() {
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

#[tokio::test]
async fn login_err_wrong_body() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!("wrong-body");
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    // check
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
