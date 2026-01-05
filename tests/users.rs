mod utils;

use rand::distr::{Alphanumeric, SampleString};
use serde_json::json;
use serial_test::serial;
use utils::login;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

// GET /users/me 200
#[tokio::test]
async fn get_personal_info_ok_player() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = player_1);

    // exec
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 200, "response_body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "id": "00000000-0000-0000-0000-000000000001",
        "username": "player_1",
        "account_type": "player",
        "profile": {
            "first_name": "player_1_first",
            "last_name": "player_1_last",
            "preferred_sports": ["football"]
        }
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}

// GET /users/me 200
#[tokio::test]
async fn get_personal_info_ok_business() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = business_1);

    // exec
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 200, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "id": "00000000-0000-0000-0000-000000000003",
        "username": "business_1",
        "account_type": "business",
        "profile": {
            "display_name": "business_1_display"
        }
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}

// GET /users/me 200
#[tokio::test]
async fn get_personal_info_ok_admin() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = admin);

    // exec
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 200, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "id": "00000000-0000-0000-0000-000000000005",
        "username": "admin",
        "account_type": "admin"
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}

#[tokio::test]
async fn get_personal_info_err_login_needed() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 401, "response body:\n{response_body:#}");

    // check response body
    let expected_body = json!({
        "message": "login_needed",
        "request_id": serde_json::from_value::<Uuid>(response_body.get("request_id").unwrap().clone()).unwrap(),
        "status": 401
    });
    assert_eq!(response_body, expected_body);

    Ok(())
}

// PATCH /users/me 204
#[tokio::test]
async fn update_personal_info_ok_player() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let first_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let last_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let preferred_sports = ["football", "basketball", "padel"];
    let request_body = ::serde_json::json!({
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
    assert_eq!(response.status(), 201);
    let response_body = response.json_body().unwrap();
    let user_id = response_body.get("user_id").unwrap().as_str().unwrap();

    login!(client, username = username, password = password);

    // exec
    let new_username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let new_last_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let payload = json!({
        "username": new_username,
        "player_profile": {
            "last_name": new_last_name,
        }
    });
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.json_body().is_err());
    assert!(response.text_body().is_err());

    // check correct excution
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body().unwrap();

    let expected_response = json!({
        "id": user_id,
        "username": new_username,
        "account_type": "player",
        "profile": {
            "first_name": first_name,
            "last_name": new_last_name,
            "preferred_sports": preferred_sports,
        }
    });
    assert_eq!(response_body, expected_response);

    Ok(())
}

// PATCH /users/me 204
#[tokio::test]
async fn update_personal_info_ok_business() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let display_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let request_body = ::serde_json::json!({
        "username": username,
        "password": password,
        "confirm_password": password,
        "account_type": "business",
        "profile": {
            "display_name": display_name,
        }
    });
    let response = client.do_post("/auth/signup", request_body).await.unwrap();
    assert_eq!(response.status(), 201);
    let response_body = response.json_body().unwrap();
    let user_id = response_body.get("user_id").unwrap().as_str().unwrap();

    login!(client, username = username, password = password);

    // exec
    let new_password = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let new_display_name = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let payload = json!({
        "password": new_password,
        "business_profile": {
            "display_name": new_display_name,
        }
    });
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.json_body().is_err());
    assert!(response.text_body().is_err());

    // check correct excution
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body().unwrap();

    let expected_response = json!({
        "id": user_id,
        "username": username,
        "account_type": "business",
        "profile": {
            "display_name": new_display_name,
        }
    });
    assert_eq!(response_body, expected_response);

    // test if password is updated by logging in
    let response = login!(client, username = username, password = new_password);
    assert_eq!(response.status(), 200);

    Ok(())
}

#[tokio::test]
#[serial(changes_admin_user)] // TODO: remove this by creating an endpoint that allows admins to create new admins
async fn update_personal_info_ok_admin() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = admin);

    // exec
    let new_username = Alphanumeric.sample_string(&mut rand::rng(), 16);
    let payload = json!({
        "password": null,
        "username": new_username,
    });
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.json_body().is_err());
    assert!(response.text_body().is_err());

    // check correct excution
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body().unwrap();

    let expected_response = json!({
        "id": "00000000-0000-0000-0000-000000000005",
        "username": new_username,
        "account_type": "admin",
    });
    assert_eq!(response_body, expected_response);

    // clean
    let payload = json!({
        "username": "admin",
    });
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    Ok(())
}

// PATCH /users/me 204
#[tokio::test]
async fn update_personal_info_ok_no_fields() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = business_2);

    // exec
    let payload = json!({});
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.json_body().is_err());
    assert!(response.text_body().is_err());

    // check correct excution
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body().unwrap();

    let expected_response = json!({
        "id": "00000000-0000-0000-0000-000000000004",
        "username": "business_2",
        "account_type": "business",
        "profile": {
            "display_name": "business_2_display",
        }
    });
    assert_eq!(response_body, expected_response);

    Ok(())
}

// PATCH /users/me 204
#[tokio::test]
async fn update_personal_info_ok_no_update() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client, user = player_2);

    // exec
    let payload = json!({
        "username": "player_2",
        "password": "player_2_password",
        "player_profile": {
            "first_name": "player_2_first",
            "last_name": "player_2_last",
        }
    });
    let response = client.do_patch("/users/me", payload).await?;

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.json_body().is_err());
    assert!(response.text_body().is_err());

    // check correct excution
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body().unwrap();

    let expected_response = json!({
        "id": "00000000-0000-0000-0000-000000000002",
        "username": "player_2",
        "account_type": "player",
        "profile": {
            "first_name": "player_2_first",
            "last_name": "player_2_last",
            "preferred_sports": ["basketball", "padel"],
        }
    });
    assert_eq!(response_body, expected_response);

    // test if password is not updated by logging in using the same credentials
    let response = login!(client, user = player_2);
    assert_eq!(response.status(), 200);

    Ok(())
}
