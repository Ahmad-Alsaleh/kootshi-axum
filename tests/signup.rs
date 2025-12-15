mod utils;

use crate::utils::login;
use anyhow::Context;
use rand::distr::{Alphanumeric, SampleString};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

#[tokio::test]
async fn signup_ok_player() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let first_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let last_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let preferred_sports = ["football", "basketball", "padel"];
    let request_body = json!({
        "username": username,
        "password": password,
        "confirm_password": password,
        "account_type": "player",
        "profile": {
            "first_name": first_name,
            "last_name": last_name,
            "preferred_sports": preferred_sports,
        }
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 201, "response body:\n{response_body:#}");

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        user_id: Uuid,
    }
    let user_id = Schema::deserialize(&response_body)
        .context("response body does not match expected schema")?
        .user_id;

    // check correct execution
    login!(client, username = username, password = password);
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;
    let expected_body = json!({
        "id": user_id,
        "username": username,
        "account_type": "player",
        "profile": {
            "first_name": first_name,
            "last_name": last_name,
            "preferred_sports": preferred_sports,
        }
    });
    assert_eq!(response_body, expected_body);

    // clean up
    // TODO
    // assert!(
    //     client
    //         .do_delete("/users/<id or username>")
    //         .await
    //         .unwrap()
    //         .status()
    //         .is_success(),
    //     "response body:\n{response_body:#}"
    // );

    Ok(())
}

#[tokio::test]
async fn signup_ok_business() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let display_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = json!({
        "username": username,
        "password": password,
        "confirm_password": password,
        "account_type": "business",
        "profile": {
            "display_name": display_name,
        }
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 201, "response body:\n{response_body:#}");

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        user_id: Uuid,
    }
    let user_id = Schema::deserialize(&response_body)
        .context("response body does not match expected schema")?
        .user_id;

    // check correct execution
    login!(client, username = username, password = password);
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;
    let expected_body = json!({
        "id": user_id,
        "username": username,
        "account_type": "business",
        "profile": {
            "display_name": display_name,
        }
    });
    assert_eq!(response_body, expected_body);

    // clean up
    // TODO
    // assert!(
    //     client
    //         .do_delete("/users/<id or username>")
    //         .await
    //         .unwrap()
    //         .status()
    //         .is_success(),
    //     "response body:\n{response_body:#}"
    // );

    Ok(())
}

#[tokio::test]
async fn signup_ok_admin() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = json!({
        "username": username,
        "password": password,
        "confirm_password": password,
        "account_type": "admin",
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 201, "response body:\n{response_body:#}");

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        user_id: Uuid,
    }
    let user_id = Schema::deserialize(&response_body)
        .context("response body does not match expected schema")?
        .user_id;

    // check correct execution
    login!(client, username = username, password = password);
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;
    let expected_body = json!({
        "id": user_id,
        "username": username,
        "account_type": "admin",
    });
    assert_eq!(response_body, expected_body);

    // clean up
    // TODO
    // assert!(
    //     client
    //         .do_delete("/users/<id or username>")
    //         .await
    //         .unwrap()
    //         .status()
    //         .is_success(),
    //     "response body:\n{response_body:#}"
    // );

    Ok(())
}

#[tokio::test]
async fn signup_err_password_and_confirm_password_are_different() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let confirm_password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = json!({
        "username": username,
        "password": password,
        "confirm_password": confirm_password,
        "account_type": "admin",
    });
    let response = client.do_post("/auth/signup", request_body).await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 400, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "message": "password_and_confirm_password_are_different",
        "request_id": serde_json::from_value::<Uuid>(response_body.get("request_id").unwrap().clone()).unwrap(),
        "status": 400
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}

#[tokio::test]
async fn signup_err_username_already_exists() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let request_body = json!({
        "username": "player_1",
        "password": "",
        "confirm_password": "",
        "account_type": "business",
        "profile": {
            "display_name": ""
        }
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 409, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "message": "username_already_exists",
        "request_id": serde_json::from_value::<Uuid>(response_body.get("request_id").unwrap().clone()).unwrap(),
        "status": 409
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}
