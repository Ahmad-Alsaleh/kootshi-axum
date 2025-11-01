use anyhow::Result;
use axum::http::StatusCode;
use serde_json::json;

const DEV_BASE_URL: &str = "http://127.0.0.1:1936";

#[tokio::test]
async fn index() -> Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL)?;
    let response = client.do_get("/").await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .header("Content-Type")
            .unwrap()
            .starts_with("text/html")
    );
    assert_eq!(
        response.text_body().unwrap(),
        "<h1>Hello <i>World!</i></h1>"
    );
    Ok(())
}

#[tokio::test]
async fn ping() -> Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL)?;
    let response = client.do_get("/ping").await?;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .header("Content-Type")
            .unwrap()
            .starts_with("text/plain")
    );
    assert_eq!(response.text_body().unwrap(), "pong!");
    Ok(())
}

#[tokio::test]
async fn fallback() -> Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL)?;
    let response = client.do_get("/does-not-exist").await?;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    Ok(())
}

#[tokio::test]
async fn login() -> Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL)?;
    const ENDPOINT: &str = "/auth/login";

    let body = json!({"username": "demo", "password": "password"});
    let response = client.do_post(ENDPOINT, body).await?;
    // TODO: test client cookies, etc.
    // TODO: test cookies if we login then make another api call
    assert_eq!(response.status(), StatusCode::OK);

    let body = json!("wrong-body");
    let response = client.do_post(ENDPOINT, body).await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = json!({"username": "demo", "password": "wrong-password"});
    let response = client.do_post(ENDPOINT, body).await?;
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(
        response.json_body().unwrap(),
        json!({
            "status": StatusCode::FORBIDDEN.as_u16(),
            "message": "invalid_username_or_password",
        })
    );

    Ok(())
}
