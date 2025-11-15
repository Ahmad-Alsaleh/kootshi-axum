mod utils;

use anyhow::Context;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashSet;
use utils::login;
use uuid::Uuid;

const DEV_BASE_URL: &str = "http://localhost:1948";

// GET /companies 200
#[tokio::test]
async fn get_all_companies_ok() {
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

// POST /companies 201
#[tokio::test]
async fn create_company_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let request_body = json!({"name": "new-company"});
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

    // check correct execution
    assert!(
        client
            .do_get("/companies/new-company")
            .await
            .unwrap()
            .status()
            .is_success()
    );

    // clean
    assert!(
        client
            .do_delete("/companies/new-company")
            .await
            .unwrap()
            .status()
            .is_success()
    );

    Ok(())
}

// POST /companies 400
#[tokio::test]
async fn create_company_err_name_exists() -> anyhow::Result<()> {
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

// DELETE companies/{name} 202
#[tokio::test]
async fn delete_company_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);
    let request_body = json!({"name": "temp-name"});
    assert!(
        client
            .do_post("/companies", request_body)
            .await
            .unwrap()
            .status()
            .is_success()
    );

    // exec
    let response = client.do_delete("/companies/temp-name").await.unwrap();

    // check status code
    assert_eq!(response.status(), 204);

    // check response body
    assert!(response.text_body().is_err());
    assert!(response.json_body().is_err());

    // check correct execution
    assert_eq!(
        client
            .do_get("/companies/temp-name")
            .await
            .unwrap()
            .status(),
        400
    );

    Ok(())
}

// DELETE /companies/{name} 400
#[tokio::test]
async fn delete_company_err_company_not_found() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let response = client.do_delete("/companies/invalid-name").await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 400);

    // TODO: make a macro that checks message and status
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
    assert_eq!(schema.message, "company_not_found");
    assert_eq!(schema.status, 400);

    Ok(())
}

// GET /companies/{name} 200
#[tokio::test]
async fn get_single_company_ok() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let response = client.do_get("/companies/Al Joker").await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 200);

    // check response body
    #[derive(Deserialize)]
    #[allow(unused)]
    struct Schema {
        id: Uuid,
        name: String,
    }
    let schema = Schema::deserialize(response_body)
        .context("response body does not match expected schema")?;
    assert_eq!(schema.name, "Al Joker");

    Ok(())
}

// GET /companies/{name} 400
#[tokio::test]
async fn get_single_company_err_company_not_found() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let response = client.do_get("/companies/invalid-name").await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 400);

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
    assert_eq!(schema.message, "company_not_found");
    assert_eq!(schema.status, 400);

    Ok(())
}

// tests geting a company without loging in
#[tokio::test]
async fn no_login() -> anyhow::Result<()> {
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
