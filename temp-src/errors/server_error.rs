impl From<CompanyControllerError> for ServerError {
    fn from(company_controller_error: CompanyControllerError) -> Self {
        match company_controller_error {
            CompanyControllerError::CompanyNameAlreadyExists => Self::CompanyNameAlreadyExists,
            CompanyControllerError::CompanyNotFound => Self::CompanyNotFound,
            CompanyControllerError::Sqlx(err) => Self::DataBase(err.to_string()),
        }
    }
}
