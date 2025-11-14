use anyhow::Context;
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

#[tokio::test]
async fn get_companies_401() {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // exec
    let response = client.do_get("/companies").await.unwrap();

    // check
    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn get_companies() {
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

    assert_eq!(companies.len(), 3);

    let fetched_names = companies
        .iter()
        .map(|company| company["name"].as_str().unwrap())
        .collect::<HashSet<_>>();
    assert!(fetched_names.is_superset(&HashSet::from(["Al Forsan", "Al Joker", "Al Abtal"])));
}

// TODO: test all response cases of POST /companies (eg company name already exists)
#[tokio::test]
async fn post_companies_200() -> anyhow::Result<()> {
    let client = httpc_test::new_client(DEV_BASE_URL).unwrap();

    // prepare
    login!(client);

    // exec
    let request_body = json!({"name": "name of new company"});
    let response = client.do_post("/companies", request_body).await.unwrap();
    let response_body = response.json_body().unwrap();

    // check status code
    assert_eq!(response.status(), 201);

    // check response body
    let keys: HashSet<_> = response_body
        .as_object()
        .context("failed while converting response body to a json object")?
        .keys()
        .map(|k| k.as_str())
        .collect();
    let expected_keys = HashSet::from(["company_id"]);
    assert_eq!(keys, expected_keys);

    let company_id = response_body
        .get("company_id")
        .context("key `company_id` is not found in resposne body")?;

    company_id
        .as_str()
        .context("failed while converting company_id to str")?
        .parse::<Uuid>()
        .context("returned company_id is not valid UUID")?;

    // clean
    client
        .do_delete("/companies/name of new company")
        .await
        .unwrap();

    Ok(())
}
