use anyhow::Context;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

#[tokio::test]
async fn signup_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({
        "username": "new.user",
        "password": "mypassword@12345!",
        "confirm_password": "mypassword@12345!",
        "last_name": "Fake Last Name"
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(status, 201, "response body:\n{response_body:#}");

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        user_id: Uuid,
    }
    Schema::deserialize(response_body).context("response body does not match expected schema")?;

    // check correct execution
    let login_body = serde_json::json!({"username": "new.user", "password": "mypassword@12345!"});
    assert!(
        client
            .do_post("/auth/login", login_body)
            .await
            .unwrap()
            .status()
            .is_success(),
        "response body:\n{response_body:#}"
    );

    // clean up
    assert!(
        client
            .do_delete("/users/new.user")
            .await
            .unwrap()
            .status()
            .is_success(),
        "response body:\n{response_body:#}"
    );

    Ok(())
}

#[tokio::test]
async fn signup_err_password_and_confirm_password_are_different() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({
        "username": "new.user",
        "password": "mypassword@12345!",
        "confirm_password": "different-password",
        "last_name": "Fake Last Name"
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(status, 400, "response body:\n{response_body:#}");

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
    assert_eq!(
        schema.message,
        "password_and_confirm_password_are_different"
    );
    assert_eq!(schema.status, 400, "response body:\n{response_body:#}");

    Ok(())
}

#[tokio::test]
async fn signup_err_username_already_exists() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({
        "username": "ahmad.alsaleh",
        "password": "mypassword@12345!",
        "confirm_password": "mypassword@12345!",
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();

    let status = response.status();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(status, 409, "response body:\n{response_body:#}");

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
    assert_eq!(schema.message, "username_already_exists");
    assert_eq!(schema.status, 409, "response body:\n{response_body:#}");

    Ok(())
}
