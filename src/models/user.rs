use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
