use crate::{
    controllers::CompanyControllerError,
    models::{Company, ModelManager},
};
use uuid::Uuid;

pub struct CompanyController;

// TODO: (later) consider passing a DTO for create and update when the Company model becomes more complex

impl CompanyController {
    pub async fn create(
        model_manager: &ModelManager,
        name: &str,
    ) -> Result<Uuid, CompanyControllerError> {
        let result = sqlx::query_scalar("INSERT INTO companies (name) VALUES ($1) RETURNING id")
            .bind(name)
            .fetch_one(model_manager.db())
            .await;

        match result {
            Ok(id) => Ok(id),
            Err(sqlx::Error::Database(err)) if err.constraint() == Some("companies_name_key") => {
                Err(CompanyControllerError::CompanyNameAlreadyExists)
            }
            Err(err) => Err(CompanyControllerError::Sqlx(err)),
        }
    }

    pub async fn get_by_name(
        model_manager: &ModelManager,
        company_name: &str,
    ) -> Result<Company, CompanyControllerError> {
        sqlx::query_as("SELECT * FROM companies WHERE name = $1")
            .bind(company_name)
            .fetch_optional(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)?
            .ok_or(CompanyControllerError::CompanyNotFound)
    }

    pub async fn get_all(
        model_manager: &ModelManager,
    ) -> Result<Vec<Company>, CompanyControllerError> {
        sqlx::query_as("SELECT * FROM companies")
            .fetch_all(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)
    }

    pub async fn delete_by_name(
        model_manager: &ModelManager,
        name: &str,
    ) -> Result<Uuid, CompanyControllerError> {
        sqlx::query_scalar::<_, Uuid>("DELETE FROM companies WHERE name = $1 RETURNING id")
            .bind(name)
            .fetch_optional(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)?
            .ok_or(CompanyControllerError::CompanyNotFound)
    }
}

#[cfg(test)]
#[serial_test::serial] // TODO: check if any of the tests below can be run in parallel to speed up tests
mod tests {
    use crate::{
        controllers::{CompanyController, CompanyControllerError},
        models::ModelManager,
    };
    use anyhow::Context;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_create_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = CompanyController::create(&model_manager, "my-company")
            .await
            .context("failed while creating company")?;

        // check
        let name: String = sqlx::query_scalar("SELECT name FROM companies WHERE id = $1")
            .bind(id)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching the name of created company")?;
        assert_eq!(name, "my-company");

        // clean
        sqlx::query("DELETE FROM companies WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await
            .context("failed while deleting inserted company")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_create_err_name_exists() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let result = CompanyController::create(&model_manager, "Al Joker").await;

        // check
        assert!(matches!(
            result,
            Err(CompanyControllerError::CompanyNameAlreadyExists)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_name_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let company = CompanyController::get_by_name(&model_manager, "Al Joker")
            .await
            .context("failed while fetching company")?;

        // check
        assert_eq!(company.name, "Al Joker");

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_name_err_name_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let result = CompanyController::get_by_name(&model_manager, "in valid name").await;

        // check
        assert!(matches!(
            result,
            Err(CompanyControllerError::CompanyNotFound)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_all_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let companies = CompanyController::get_all(&model_manager)
            .await
            .context("failed while fetching all companies")?;

        //check
        let names = companies
            .iter()
            .map(|c| c.name.as_str())
            .collect::<HashSet<_>>();
        assert_eq!(names, HashSet::from(["Al Forsan", "Al Joker", "Al Abtal"]));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_name_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = CompanyController::delete_by_name(&model_manager, "Al Joker")
            .await
            .context("failed while deleting company")?;

        // check
        let result = sqlx::query("SELECT * FROM companies WHERE id = $1 OR name = 'Al Joker'")
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
            .context("failed while fetching deleted company")?;

        assert!(result.is_none());

        // clean
        sqlx::query("INSERT INTO companies (id, name) VALUES ($1, 'Al Joker')")
            .bind(id)
            .execute(model_manager.db())
            .await
            .context("failed while inserting deleted company")?;

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_name_err_name_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let result = CompanyController::delete_by_name(&model_manager, "name does not exist").await;

        // check
        assert!(matches!(
            result,
            Err(CompanyControllerError::CompanyNotFound)
        ));

        Ok(())
    }
}
