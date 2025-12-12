mod utils;

use serde_json::json;
use utils::login;

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

    println!("{response_body:#}");

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

// // PATCH /users/me 204
// #[tokio::test]
// #[serial(user_table)] // TODO: insert a dummy user before updating to remove `serial`
// async fn update_personal_info_ok_single_field() -> anyhow::Result<()> {
//     let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
//
//     // prepare
//     login!(client);
//
//     // exec
//     let new_last_name = Uuid::new_v4().to_string(); // any random value
//     let payload = json!({
//         "last_name": new_last_name
//     });
//     let response = client.do_patch("/users/me", payload).await?;
//
//     // check status code
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     // check response body
//     assert!(response.json_body().is_err());
//     assert!(response.text_body().is_err());
//
//     // check correct excution
//     let response = client.do_get("/users/me").await?;
//     let response_body = response.json_body().unwrap();
//
//     #[derive(Deserialize)]
//     #[allow(unused)]
//     struct Schema {
//         id: Uuid,
//         username: String,
//         first_name: Option<String>,
//         last_name: Option<String>,
//     }
//     let schema = Schema::deserialize(response_body)
//         .context("response body does not match expected schema")?;
//     assert_eq!(schema.username, "ahmad.alsaleh");
//     assert_eq!(schema.first_name.as_deref(), Some("Ahmad"));
//     assert_eq!(schema.last_name, Some(new_last_name));
//
//     // clean
//     let payload = json!({
//         "last_name": "Alsaleh",
//     });
//     let response = client.do_patch("/users/me", payload).await?;
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     Ok(())
// }
//
// // PATCH /users/me 204
// #[tokio::test]
// #[serial(user_table)]
// async fn update_personal_info_ok_multiple_fields_and_a_null_field() -> anyhow::Result<()> {
//     let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
//
//     // prepare
//     login!(client);
//
//     // exec
//     let new_username = Uuid::new_v4().to_string(); // any random value
//     let new_first_name = Uuid::new_v4().to_string(); // any random value
//     let payload = json!({
//         "username": new_username,
//         "first_name": new_first_name,
//         "last_name": null,
//     });
//     let response = client.do_patch("/users/me", payload).await?;
//
//     // check status code
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     // check response body
//     assert!(response.json_body().is_err());
//     assert!(response.text_body().is_err());
//
//     // check correct excution
//     let response = client.do_get("/users/me").await?;
//     let response_body = response.json_body().unwrap();
//
//     #[derive(Deserialize)]
//     #[allow(unused)]
//     struct Schema {
//         id: Uuid,
//         username: String,
//         first_name: Option<String>,
//         last_name: Option<String>,
//     }
//     let schema = Schema::deserialize(response_body)
//         .context("response body does not match expected schema")?;
//     assert_eq!(schema.username, new_username);
//     assert_eq!(schema.first_name, Some(new_first_name));
//     assert_eq!(schema.last_name, None);
//
//     // clean
//     let payload = json!({
//         "username": "ahmad.alsaleh",
//         "first_name": "Ahmad",
//         "last_name": "Alsaleh",
//     });
//     let response = client.do_patch("/users/me", payload).await?;
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     Ok(())
// }
//
// // PATCH /users/me 204
// #[tokio::test]
// #[serial(user_table)]
// async fn update_personal_info_ok_zero_fields() -> anyhow::Result<()> {
//     let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
//
//     // prepare
//     login!(client);
//
//     // exec
//     let payload = json!({});
//     let response = client.do_patch("/users/me", payload).await?;
//
//     // check status code
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     // check response body
//     assert!(response.json_body().is_err());
//     assert!(response.text_body().is_err());
//
//     // check correct excution
//     let response = client.do_get("/users/me").await?;
//     let response_body = response.json_body().unwrap();
//
//     #[derive(Deserialize)]
//     #[allow(unused)]
//     struct Schema {
//         id: Uuid,
//         username: String,
//         first_name: Option<String>,
//         last_name: Option<String>,
//     }
//     let schema = Schema::deserialize(response_body)
//         .context("response body does not match expected schema")?;
//     assert_eq!(schema.username, "ahmad.alsaleh");
//     assert_eq!(schema.first_name.as_deref(), Some("Ahmad"));
//     assert_eq!(schema.last_name.as_deref(), Some("Alsaleh"));
//
//     Ok(())
// }
//
// // PATCH /users/me 204
// #[tokio::test]
// #[serial(user_table)]
// async fn update_personal_info_ok_no_update() -> anyhow::Result<()> {
//     let client = httpc_test::new_client(DEV_BASE_URL).unwrap();
//
//     // prepare
//     login!(client);
//
//     // exec
//     let payload = json!({
//         "username": "ahmad.alsaleh",
//         "first_name": "Ahmad",
//         "last_name": "Alsaleh",
//     });
//     let response = client.do_patch("/users/me", payload).await?;
//
//     // check status code
//     assert_eq!(response.status(), 204, "response body:\n{response_body:#}");
//
//     // check response body
//     assert!(response.json_body().is_err());
//     assert!(response.text_body().is_err());
//
//     // check correct excution
//     let response = client.do_get("/users/me").await?;
//     let response_body = response.json_body().unwrap();
//
//     #[derive(Deserialize)]
//     #[allow(unused)]
//     struct Schema {
//         id: Uuid,
//         username: String,
//         first_name: Option<String>,
//         last_name: Option<String>,
//     }
//     let schema = Schema::deserialize(response_body)
//         .context("response body does not match expected schema")?;
//     assert_eq!(schema.username, "ahmad.alsaleh");
//     assert_eq!(schema.first_name.as_deref(), Some("Ahmad"));
//     assert_eq!(schema.last_name.as_deref(), Some("Alsaleh"));
//
//     Ok(())
// }
