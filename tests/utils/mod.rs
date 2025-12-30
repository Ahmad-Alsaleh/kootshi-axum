macro_rules! login {
    ($client:expr, user = $user:ident) => {{
        let login_body = ::serde_json::json!({
            "username": ::core::stringify!($user),
            "password": ::core::concat!(::core::stringify!($user), "_password"),
        });
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};

    ($client:expr, username = $username:expr, password = $password:expr) => {{
        let login_body = ::serde_json::json!({
            "username": $username,
            "password": $password,
        });
        $client.do_post("/auth/login", login_body).await.unwrap();
    }};
}

pub(crate) use login;
