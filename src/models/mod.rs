use crate::{configs::config, secrets::SecretManager};
use sqlx::PgPool;

mod company;
mod request_log_info;
mod request_payloads;
mod user;

#[cfg(debug_assertions)]
use crate::controllers::UserController;
pub use company::Company;
pub use request_log_info::RequestLogInfo;
pub use request_payloads::LoginPayload;
pub use user::{FromUser, User, UserForLogin};

#[derive(Clone)]
pub struct ModelManager(PgPool);

impl ModelManager {
    pub async fn new() -> Self {
        let pool = PgPool::connect(&config().db_uri)
            .await
            .expect("failed to connect to DB");
        Self(pool)
    }

    /// Creates the tables in the DB. This function will also seed the tables with sample data if
    /// the project is built in the debug mode
    pub async fn init(&self) {
        // TODO: use migrations to create the tables. once that's done, remove this method and
        // create a method to seed the data in debug mode

        #[cfg(debug_assertions)]
        sqlx::query("DROP TABLE IF EXISTS companies, users;")
            .execute(self.db())
            .await
            .expect("failed to drop tables");

        sqlx::raw_sql(
            r#"
            CREATE TABLE IF NOT EXISTS companies (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(128) NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(128) NOT NULL UNIQUE,
                first_name VARCHAR(128),
                last_name VARCHAR(128),
                password_hash BYTEA NOT NULL,
                password_salt BYTEA NOT NULL
            );
            "#,
        )
        .execute(self.db())
        .await
        .expect("failed to create tables");

        #[cfg(debug_assertions)]
        {
            sqlx::raw_sql(
                r#"
                -- companies
                INSERT INTO companies (name) VALUES ('Al Forsan');
                INSERT INTO companies (name) VALUES ('Al Joker');
                INSERT INTO companies (name) VALUES ('Al Abtal');

                -- users (passwords hash and salt are temporary)
                INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES ('ahmad.alsaleh', 'Ahmad', 'Alsaleh', '\x00', '\x00');
                INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES ('mohammed.hassan', 'Mohammed', 'Hassan', '\x00', '\x00');
                "#,
            )
            .execute(self.db())
            .await
            .expect("failed to seed tables");

            // TODO: replace this once UserController::insert_user is implemented
            for username in ["ahmad.alsaleh", "mohammed.hassan"] {
                let mut password_salt = [0; 32];
                SecretManager::generate_salt(&mut password_salt);

                let password_hash =
                    SecretManager::hash_secret("passme", &password_salt, &config().password_key);

                sqlx::query("UPDATE users SET password_salt = $1, password_hash = $2 WHERE username = $3 RETURNING 1")
                    .bind(password_salt)
                    .bind(password_hash)
                    .bind(username)
                    .fetch_one(self.db())
                    .await.expect("failed to update password hash and salt");
            }
        }
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
