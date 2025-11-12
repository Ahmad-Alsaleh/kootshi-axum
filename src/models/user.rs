use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserForLogin {
    pub id: Uuid,
    pub password_hash: Vec<u8>,
    pub password_salt: Vec<u8>,
}

#[derive(FromRow)]
pub struct UserForUpdatePassword {
    pub password_salt: Vec<u8>,
}

pub trait FromUser {}
impl FromUser for UserForLogin {}
impl FromUser for UserForUpdatePassword {}
