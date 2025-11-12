use crate::configs::config;
use sqlx::PgPool;

mod company;
mod request_log_info;
mod request_payloads;
mod user;

pub use company::Company;
pub use request_log_info::RequestLogInfo;
pub use request_payloads::{LoginPayload, SignupPayload};
pub use user::{FromUser, User, UserForInsertUser, UserForLogin, UserForUpdatePassword};

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
        sqlx::query(include_str!("../../sql/00-drop-tables.sql"))
            .execute(self.db())
            .await
            .expect("failed to drop tables");

        sqlx::raw_sql(include_str!("../../sql/01-create-tables.sql"))
            .execute(self.db())
            .await
            .expect("failed to create tables");

        #[cfg(debug_assertions)]
        sqlx::raw_sql(include_str!("../../sql/02-seed-tables.sql"))
            .execute(self.db())
            .await
            .expect("failed to seed tables");
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
