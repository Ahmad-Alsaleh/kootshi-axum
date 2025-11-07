use sqlx::PgPool;
use std::sync::Arc;

mod company;
mod request_log_info;
mod request_payloads;

use crate::configs::config;
pub use company::Company;
pub use request_log_info::RequestLogInfo;
pub use request_payloads::LoginPayload;

#[derive(Clone)]
pub struct ModelManager(Arc<PgPool>);

impl ModelManager {
    pub async fn new() -> Self {
        let pool = PgPool::connect(&config().db_uri).await.unwrap();
        Self(Arc::new(pool))
    }

    pub fn db(&self) -> &PgPool {
        &self.0
    }
}
