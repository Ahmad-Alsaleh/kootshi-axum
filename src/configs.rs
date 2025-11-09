use std::{sync::OnceLock, time::Duration};

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load)
}

pub struct Config {
    pub db_uri: String,
    pub server_address: String,
    pub jwt_exp_duration: Duration,
    pub jwt_secret: Vec<u8>,
}

impl Config {
    fn load() -> Self {
        Self {
            db_uri: read_env_var("DB_URI"),
            server_address: read_env_var("SERVER_ADDRESS"),
            jwt_exp_duration: Duration::from_secs(
                read_env_var("JWT_EXP_DURATION_SECONDS")
                    .parse()
                    .expect("failed to parse JWT_EXP_DURATION_SECONDS as an int"),
            ),
            jwt_secret: read_env_var("JWT_SECRET").into_bytes(),
        }
    }
}

fn read_env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|err| panic!("failed to read env var `{key}`: {err}"))
}
