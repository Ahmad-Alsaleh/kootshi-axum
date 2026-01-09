macro_rules! login {
    ($client:expr, user = $user:ident) => {{
        let login_body = ::serde_json::json!({
            "username": ::core::stringify!($user),
            "password": ::core::concat!(::core::stringify!($user), "_password"),
        });
        let response = $client.do_post("/auth/login", login_body).await.unwrap();
        assert_eq!(response.status(), 200);
        response
    }};

    ($client:expr, username = $username:expr, password = $password:expr) => {{
        let login_body = ::serde_json::json!({
            "username": $username,
            "password": $password,
        });
        let response = $client.do_post("/auth/login", login_body).await.unwrap();
        assert_eq!(response.status(), 200);
        response
    }};
}
pub(crate) use login;

macro_rules! test_login_needed_error {
    ($test_name:ident, $endpoint_path:literal) => {
        #[::tokio::test]
        async fn $test_name() -> ::anyhow::Result<()> {
            let client = ::httpc_test::new_client(DEV_BASE_URL).unwrap();

            // exec
            let response = client.do_get($endpoint_path).await?;
            let response_body = response.json_body()?;

            // check status code
            assert_eq!(response.status(), 401, "response body:\n{response_body:#}");

            // check response body
            let expected_body = ::serde_json::json!({
                "message": "login_needed",
                "request_id": ::serde_json::from_value::<::uuid::Uuid>(response_body.get("request_id").unwrap().clone()).unwrap(),
                "status": 401
            });
            assert_eq!(response_body, expected_body);

            Ok(())
        }

    };
}
pub(crate) use test_login_needed_error;

macro_rules! test_get_ok {
    (
        test_name = $test_name:ident,
        user = $user:ident,
        path = $path:literal,
        status = $expected_status_code:literal,
        response = $expected_response_body:expr
    ) => {
        #[::tokio::test]
        async fn $test_name() -> ::anyhow::Result<()> {
            let client = ::httpc_test::new_client(DEV_BASE_URL)?;

            // prepare
            login!(client, user = $user);

            // exec
            let response = client.do_get($path).await?;
            let response_body = response.json_body()?;

            // check status code
            assert_eq!(
                response.status(),
                $expected_status_code,
                "response_body:\n{response_body:#}"
            );

            // check response body
            assert_eq!(response_body, $expected_response_body);

            Ok(())
        }
    };
}
pub(crate) use test_get_ok;

macro_rules! test_get_err {
    (
        test_name = $test_name:ident,
        user = $user:ident,
        path = $path:expr,
        status = $expected_status_code:literal,
        error_message = $error_message:literal
    ) => {
        #[::tokio::test]
        async fn $test_name() -> ::anyhow::Result<()> {
            let client = ::httpc_test::new_client(DEV_BASE_URL)?;

            // prepare
            login!(client, user = $user);

            // exec
            let response = client.do_get($path).await?;
            let response_body = response.json_body()?;

            // check status code
            assert_eq!(
                response.status(),
                $expected_status_code,
                "response_body:\n{response_body:#}"
            );

            // check response body
            let expected_body = json!({
                "message": $error_message,
                "request_id": serde_json::from_value::<Uuid>(response_body.get("request_id").unwrap().clone()).unwrap(),
                "status": $expected_status_code
            });
            assert_eq!(response_body, expected_body);

            Ok(())
        }
    };
}
pub(crate) use test_get_err;
