use crate::configs::config;
use sqlx::PgPool;

pub mod api_schemas;
pub mod tables;

#[derive(Clone)]
pub struct ModelManager(PgPool);

impl ModelManager {
    pub async fn new() -> Self {
        let pool = PgPool::connect(&config().database_url)
            .await
            .expect("failed to connect to DB");
        Self(pool)
    }

    #[cfg(debug_assertions)]
    pub async fn seed_fake_data(&self) {
        self.unseed_fake_data().await;
        sqlx::raw_sql(include_str!("../../sql/seed-fake-data.sql"))
            .execute(self.db())
            .await
            .expect("failed to seed tables");
    }

    #[cfg(debug_assertions)]
    pub async fn unseed_fake_data(&self) {
        sqlx::raw_sql(include_str!("../../sql/unseed-fake-data.sql"))
            .execute(self.db())
            .await
            .expect("failed to seed tables");
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
