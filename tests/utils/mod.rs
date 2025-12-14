macro_rules! login {
    ($client:expr, user = $user:ident) => {{
        let login_body = serde_json::json!({"username": stringify!($user), "password": concat!(stringify!($user), "_password")});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, username = $username:literal, password = $password:literal) => {{
        let login_body = serde_json::json!({"username": $username, "password": $password});
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};
}

#[allow(unused)]
pub(crate) use login;
