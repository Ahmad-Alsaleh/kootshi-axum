use sqlx::PgPool;
use std::sync::Arc;

mod company;
mod request_log_info;
mod request_payloads;

pub use company::Company;
pub use request_log_info::RequestLogInfo;
pub use request_payloads::LoginPayload;

#[derive(Clone)]
pub struct ModelManager(Arc<PgPool>);

impl ModelManager {
    pub async fn new() -> Self {
        // TODO: use a config for the db conneciton string

        let pool = PgPool::connect("postgres://postgres:dev-password@localhost:1934/postgres")
            .await
            .unwrap();
        Self(Arc::new(pool))
    }

    // TODO: use pub(in crate::controllers)
    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
