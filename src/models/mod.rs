use crate::configs::config;
use sqlx::PgPool;

mod company;
mod request_log_info;
mod request_payloads;
mod user;

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
                password_hash VARCHAR(256) NOT NULL,
                password_salt VARCHAR(64) NOT NULL
            );
            "#,
        )
        .execute(self.db())
        .await
        .expect("failed to create tables");

        #[cfg(debug_assertions)]
        sqlx::raw_sql(
            r#"
            -- companies
            INSERT INTO companies (name) VALUES ('Al Forsan');
            INSERT INTO companies (name) VALUES ('Al Joker');
            INSERT INTO companies (name) VALUES ('Al Abtal');

            -- users
            INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES ('ahmad.alsaleh', 'Ahmad', 'Alsaleh', 'temp', 'temp');
            INSERT INTO users (username, first_name, last_name, password_hash, password_salt) VALUES ('mohammed.hassan', 'Mohammed', 'Hassan', 'temp', 'temp');
            "#,
        )
        .execute(self.db())
        .await
        .expect("failed to seed tables");
        // TODO: insert passwords to the seeded users, once the salting logic or UserController::update_password is implemented
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
