use std::{sync::OnceLock, time::Duration};

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load)
}

pub struct Config {
    pub db_uri: String,
    pub server_address: String,
    pub jwt_exp_duration: Duration,
    pub jwt_secret: String,
}

impl Config {
    fn load() -> Self {
        Self {
            db_uri: std::env::var("DB_URI").unwrap(),
            server_address: std::env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| String::from("127.0.0.1:1936")),
            jwt_exp_duration: Duration::from_secs(
                std::env::var("JWT_EXP_DURATION_SECONDS")
                    .map(|x| x.parse().unwrap())
                    .unwrap_or(60 * 15),
            ),
            jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        }
    }
}
