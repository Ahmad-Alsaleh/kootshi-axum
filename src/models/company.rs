use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Clone)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    // fields: Vec<Uuid>, // TODO: add this later
}
