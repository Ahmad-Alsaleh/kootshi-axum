macro_rules! login {
    ($client:expr, user = player_1) => {{
        let login_body = serde_json::json!({"username": "player_1", "password": "user_1_password"});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, user = business_1) => {{
        let login_body = serde_json::json!({"username": "business_1", "password": "business_1_password"});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, user = admin) => {{
        let login_body = serde_json::json!({"username": "admin", "password": "admin"});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, $username:literal, $password:literal) => {{
        let login_body = serde_json::json!({"username": $username, "password": $password});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};
}

pub(crate) use login;
