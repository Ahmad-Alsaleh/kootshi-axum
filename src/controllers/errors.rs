use crate::errors::error_impl;

#[derive(Debug)]
pub enum ControllerError {
    UserNotFound,
    Sqlx(sqlx::Error),
}

error_impl!(ControllerError);
