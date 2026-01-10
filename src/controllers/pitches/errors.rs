use crate::errors::error_impl;

#[derive(Debug)]
pub enum PitchControllerError {
    Sqlx(sqlx::Error),
}

error_impl!(PitchControllerError);
