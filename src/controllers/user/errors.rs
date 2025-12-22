use crate::errors::error_impl;

#[derive(Debug)]
pub enum UserControllerError {
    UserNotFound,
    UsernameAlreadyExists,
    BusinessDisplayNameAlreadyExists,
    Sqlx(sqlx::Error),
}

error_impl!(UserControllerError);
