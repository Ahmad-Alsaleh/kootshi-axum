use sqlx::PgPool;

mod company;
mod request_log_info;
mod request_payloads;

use crate::configs::config;
pub use company::Company;
pub use request_log_info::RequestLogInfo;
pub use request_payloads::LoginPayload;

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

        // TODO: (imp) replace plain passwords with hashed/salted passwords
        sqlx::raw_sql(
            r#"
            CREATE TABLE IF NOT EXISTS companies (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR(128) NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS users (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                username VARCHAR(128) NOT NULL UNIQUE,
                password VARCHAR(128) NOT NULL,
                first_name VARCHAR(128),
                last_name VARCHAR(128)
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
            INSERT INTO users (username, password, first_name, last_name) VALUES ('ahmad.alsaleh', 'passme', 'Ahmad', 'Alsaleh');
            INSERT INTO users (username, password, first_name, last_name) VALUES ('mohammed.hassan', 'my password', 'Mohammed', 'Hassan');
            "#,
        )
        .execute(self.db())
        .await
        .expect("failed to seed tables");
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
