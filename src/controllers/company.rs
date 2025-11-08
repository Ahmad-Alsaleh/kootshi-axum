// TODO: remove me
#![allow(unused)]

use crate::models::{Company, ModelManager};
use uuid::Uuid;

pub struct CompanyController;

impl CompanyController {
    pub async fn create(model_manager: &ModelManager, name: String) -> Result<Uuid, sqlx::Error> {
        sqlx::query_scalar("INSERT INTO companies (name) VALUES ($1) RETURNING id")
            .bind(name)
            .fetch_one(model_manager.db())
            .await
    }

    pub async fn get_all(model_manager: &ModelManager) -> Result<Vec<Company>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM companies")
            .fetch_all(model_manager.db())
            .await
    }

    pub async fn get_by_id(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<Option<Company>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM companies WHERE id = $1")
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
    }

    // TODO: add delete and update methods (make sure to handel errors and edge cases)
}

#[cfg(test)]
mod tests {
    use crate::{controllers::CompanyController, models::ModelManager};
    use anyhow::Context;
    use serial_test::serial;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let id = CompanyController::create(&model_manager, String::from("my-company"))
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
            .context("failed while cleaning inserted company")?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let (id, expected_name): (Uuid, String) =
            sqlx::query_as("SELECT id, name FROM companies LIMIT 1")
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching a the id and name of an inserted company")?;

        // exec
        let company = CompanyController::get_by_id(&model_manager, id)
            .await
            .with_context(|| format!("failed while getting company with id: `{id}`"))?;

        // check
        assert_eq!(company.map(|c| c.name), Some(expected_name));

        Ok(())
    }

    #[serial]
    #[tokio::test]
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

    #[serial]
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
}
