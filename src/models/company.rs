use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow, Clone)]
pub struct Company {
    id: Uuid,
    name: String,
    // fields: Vec<Uuid>, // TODO: add this later
}
