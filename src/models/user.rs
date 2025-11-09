use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    // pub password: String, // TODO: make sure this is not serialized, maybe use a secret crate to wrap the password, or add a serder attr to skip serializing this field
    pub first_name: String,
    pub last_name: String,
}
