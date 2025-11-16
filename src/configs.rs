use std::{str::FromStr, sync::OnceLock, time::Duration};

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(Config::load)
}

pub struct Config {
    pub database_url: String,
    pub server_address: String,
    pub auth_token_exp_duration: Duration,
    pub auth_token_key: Vec<u8>,
    pub password_key: Vec<u8>,
}

impl Config {
    fn load() -> Self {
        Self {
            database_url: read_env_var("DATABASE_URL"),
            server_address: read_env_var("SERVER_ADDRESS"),
            auth_token_exp_duration: Duration::from_secs(read_env_var_parsed(
                "AUTH_TOKEN_EXP_DURATION_SEC",
            )),
            // TODO: consider using base64_url::decode instead of into_bytes
            auth_token_key: read_env_var("AUTH_TOKEN_KEY").into_bytes(),
            // TODO: consider using base64_url::decode instead of into_bytes
            password_key: read_env_var("PASSWORD_KEY").into_bytes(),
        }
    }
}

fn read_env_var(env_var_name: &str) -> String {
    std::env::var(env_var_name)
        .unwrap_or_else(|err| panic!("failed to read env var `{env_var_name}`: {err}"))
}

fn read_env_var_parsed<T: FromStr>(env_var_name: &str) -> T {
    read_env_var(env_var_name)
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse {env_var_name}"))
}
