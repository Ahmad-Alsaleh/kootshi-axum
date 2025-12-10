use crate::errors::error_impl;

#[derive(Debug)]
pub enum CompanyControllerError {
    CompanyNameAlreadyExists,
    CompanyNotFound,
    Sqlx(sqlx::Error),
}

error_impl!(CompanyControllerError);
