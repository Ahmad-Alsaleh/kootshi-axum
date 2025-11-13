use crate::errors::error_impl;

#[derive(Debug)]
pub enum CompanyControllerError {
    CompanyNameAlreadyExists,
    Sqlx(sqlx::Error),
}

error_impl!(CompanyControllerError);
