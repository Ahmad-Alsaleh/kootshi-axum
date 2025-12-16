use anyhow::Context;
use axum::http::StatusCode;
use rand::distr::{Alphanumeric, SampleString};
use serde::Deserialize;
use serde_json::json;

// TODO: test the output (logs) of the server. ig a good way of doing it is by running the server
// in a command and capturing stdout. but ig it is better to put these tests in
// tests/server-logs.rs

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

// TODO: for the login endponit, test client cookies, etc. test cookies if we login then make another api call
// an easy and (imo) good way is to make a /protected-ping endpoint (follow restful naming conventions) for tests only that will return `pong!`

// TODO: (important) test calling a protected endpoint without logging in, to ensure auth i didn't forget to protect the endpoint with auth

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
    let request_body = json!({"username": "player_1", "password": "player_1_password"});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();
    let set_cookie_header = response.header("set-cookie").unwrap();

    // check status code
    assert_eq!(status, 200, "response body:\n{response_body:#}");

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
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = json!({"username": username, "password": password});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(status, 400, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "message": "invalid_username",
        "request_id": response_body.get("request_id").unwrap(),
        "status": 400,
    });
    assert_eq!(response_body, expected_body);

    // check headers
    assert!(response.header("set-cookie").is_none());

    // check client cookies
    assert!(client.cookie("auth-token").is_none());

    Ok(())
}

#[tokio::test]
async fn login_err_wrong_password() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = json!({"username": "admin", "password": password});
    let response = client.do_post("/auth/login", request_body).await.unwrap();

    let status = response.status();
    let content_type = response.header("Content-Type").unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "response body:\n{response_body:#}"
    );

    // check headers
    assert!(content_type.to_lowercase().starts_with("application/json"));

    // check response body
    assert_eq!(
        response_body,
        json!({
            "message": "invalid_username_or_password",
            "request_id": response_body.get("request_id").unwrap(),
            "status": 401,
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
