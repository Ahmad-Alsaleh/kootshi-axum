use anyhow::Context;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948";

macro_rules! login {
    ($client:expr) => {
        let login_body = json!({"username":"ahmad.alsaleh", "password": "passme"});
        $client.do_post("/auth/login", login_body).await.unwrap();
    };
}

// tests geting a company without loging in
#[tokio::test]
async fn get_companies_401() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let response = client.do_get("/companies").await.unwrap();
    let response_body = response.json_body().unwrap();

    // check statuc code
    assert_eq!(response.status(), 401);

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        message: String,
        request_id: Uuid,
        status: u16,
    }
    let response = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;
    assert_eq!(response.message, "login_needed");
    assert_eq!(response.status, 401);

    Ok(())
}

#[tokio::test]
async fn get_companies_200() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let response = client.do_get("/companies").await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 200);

    // check response body
    let companies = response_body.as_array().unwrap();

    let fetched_names = companies
        .iter()
        .map(|company| company["name"].as_str().unwrap())
        .collect::<HashSet<_>>();
    assert!(fetched_names.is_superset(&HashSet::from(["Al Forsan", "Al Joker", "Al Abtal"])));
}

#[tokio::test]
async fn post_companies_200() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let request_body = json!({"name": "name of new company"});
    let response = client.do_post("/companies", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();
    dbg!(&response_body);

    // check status code
    assert_eq!(response.status(), 201);

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        company_id: Uuid,
    }
    // TODO: find a way to make deserialize strict, i.e. the Value can't have extra keys that are
    // not in the schema
    Schema::deserialize(response_body).context("response body does not match expected schema")?;

    // clean
    client
        .do_delete("/companies/name of new company")
        .await
        .unwrap();

    Ok(())
}

// tests creating a company with a name that already exists
#[tokio::test]
async fn post_companies_400() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let request_body = json!({"name": "Al Joker"});
    let response = client.do_post("/companies", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 400);

    // check resposne body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        message: String,
        request_id: Uuid,
        status: u16,
    }
    let schema = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;
    assert_eq!(schema.message, "company_name_already_exists");
    assert_eq!(schema.status, 400);

    Ok(())
}
