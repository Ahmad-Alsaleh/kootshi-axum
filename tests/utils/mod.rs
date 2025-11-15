macro_rules! login {
    ($client:expr) => {{
        let login_body = serde_json::json!({"username": "ahmad.alsaleh", "password": "passme"});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, $username:literal, $password:literal) => {{
        let login_body = serde_json::json!({"username": $username, "password": $password});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};
}

pub(crate) use login;
