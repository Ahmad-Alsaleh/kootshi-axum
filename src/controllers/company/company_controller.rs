use crate::{
    controllers::CompanyControllerError,
    models::{Company, ModelManager},
};
use uuid::Uuid;

pub struct CompanyController;

// TODO: consider passing a DTO for create and update when the Company model becomes more complex

// TODO: make a custom error CompanyControllerError to be consistent with UserControllerError

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

    pub async fn get_all(
        model_manager: &ModelManager,
    ) -> Result<Vec<Company>, CompanyControllerError> {
        sqlx::query_as("SELECT * FROM companies")
            .fetch_all(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)
    }

    pub async fn get_by_id(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<Option<Company>, CompanyControllerError> {
        sqlx::query_as("SELECT * FROM companies WHERE id = $1")
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)
    }

    pub async fn delete_by_id(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<Option<Company>, CompanyControllerError> {
        sqlx::query_as("DELETE FROM companies WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)
    }

    pub async fn update_by_id(
        model_manager: &ModelManager,
        id: Uuid,
        new_name: &str,
    ) -> Result<Option<Company>, CompanyControllerError> {
        // TODO: consider returning a CompanyNotFound error
        sqlx::query_as("UPDATE companies SET name = $1 WHERE id = $2 RETURNING *")
            .bind(new_name)
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
            .map_err(CompanyControllerError::Sqlx)
    }
}

#[cfg(test)]
mod tests {
    use crate::{controllers::CompanyController, models::ModelManager};
    use anyhow::Context;
    use serial_test::serial;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[tokio::test]
    #[serial]
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
    #[serial]
    async fn test_get_by_id_id_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let (id, name): (Uuid, String) = sqlx::query_as("SELECT id, name FROM companies LIMIT 1")
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching the id and name of a previously inserted company")?;

        // exec
        let company = CompanyController::get_by_id(&model_manager, id)
            .await
            .with_context(|| format!("failed while getting company with id: `{id}`"))?;

        // check
        assert!(company.is_some());
        assert_eq!(company.unwrap().name, name);

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_by_id_id_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = Uuid::new_v4();
        let company = CompanyController::get_by_id(&model_manager, id)
            .await
            .with_context(|| format!("failed while fetching company with id: `{id}`"))?;

        // check
        assert!(company.is_none());

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_get_all() -> anyhow::Result<()> {
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
    #[serial]
    async fn test_delete_by_id_id_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let (id, name): (Uuid, String) = sqlx::query_as("SELECT id, name FROM companies LIMIT 1")
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching the id and name of a previously inserted company")?;

        // exec
        let company = CompanyController::delete_by_id(&model_manager, id)
            .await
            .with_context(|| format!("failed while deleting company with id: `{id}`"))?;

        // check
        assert!(company.is_some());
        assert_eq!(company.unwrap().name, name);

        // clean
        sqlx::query("INSERT INTO companies (id, name) VALUES ($1, $2)")
            .bind(id)
            .bind(name)
            .execute(model_manager.db())
            .await
            .context("failed while inserting deleted company")?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_by_id_id_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = Uuid::new_v4();
        let company = CompanyController::delete_by_id(&model_manager, id)
            .await
            .with_context(|| format!("failed while deleting company with id: `{id}`"))?;

        // check
        assert!(company.is_none());

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_by_id_id_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let (id, name): (Uuid, String) = sqlx::query_as("SELECT id, name FROM companies LIMIT 1")
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching the id and name of a previously inserted company")?;

        // exec
        let company = CompanyController::update_by_id(&model_manager, id, "my new company")
            .await
            .with_context(|| format!("failed while updating company with id: `{id}`"))?;

        // check
        assert!(company.is_some());
        assert_eq!(company.unwrap().name, "my new company");

        // clean
        sqlx::query("UPDATE companies SET name = $1 WHERE id = $2 RETURNING *")
            .bind(name)
            .bind(id)
            .execute(model_manager.db())
            .await
            .context("failed while changing company name to original name")?;

        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_update_by_id_id_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = Uuid::new_v4();
        let company = CompanyController::update_by_id(&model_manager, id, "new name")
            .await
            .with_context(|| format!("failed while updating company with id: `{id}`"))?;

        // check
        assert!(company.is_none());

        Ok(())
    }
}
