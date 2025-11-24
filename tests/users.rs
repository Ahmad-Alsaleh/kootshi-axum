mod utils;

use anyhow::Context;
use serde::Deserialize;
use utils::login;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948/api/v1";

// GET /users/me 200
#[tokio::test]
async fn get_personal_info_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let response = client.do_get("/users/me").await?;
    let response_body = response.json_body()?;

    // check status code
    assert_eq!(response.status(), 200);

    // check response body
    #[derive(Deserialize, PartialEq, Debug)]
    struct Schema {
        id: Uuid,
        username: String,
        first_name: Option<String>,
        last_name: Option<String>,
    }
    let user = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;

    let expected_user = Schema {
        id: user.id,
        username: String::from("ahmad.alsaleh"),
        first_name: Some(String::from("Ahmad")),
        last_name: Some(String::from("Alsaleh")),
    };

    assert_eq!(user, expected_user);

    Ok(())
}
