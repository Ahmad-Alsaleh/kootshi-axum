use std::{sync::OnceLock, time::Duration};

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load)
}

pub struct Config {
    pub db_uri: String,
    pub server_address: String,
    pub jwt_exp_duration: Duration,
    pub jwt_key: Vec<u8>,
    pub password_key: Vec<u8>,
}

impl Config {
    fn load() -> Self {
        Self {
            db_uri: read_env_var("DB_URI"),
            server_address: read_env_var("SERVER_ADDRESS"),
            jwt_exp_duration: Duration::from_secs(
                read_env_var("JWT_EXP_DURATION_SEC")
                    .parse()
                    .expect("failed to parse JWT_EXP_DURATION_SECONDS as an int"),
            ),
            // TODO: consider using base64_url::decode instead of into_bytes
            jwt_key: read_env_var("JWT_KEY").into_bytes(),
            // TODO: consider using base64_url::decode instead of into_bytes
            password_key: read_env_var("PASSWORD_KEY").into_bytes(),
        }
    }
}

fn read_env_var(env_var_name: &str) -> String {
    std::env::var(env_var_name)
        .unwrap_or_else(|err| panic!("failed to read env var `{env_var_name}`: {err}"))
}
