use crate::models::{Company, ModelManager};
use uuid::Uuid;

pub struct CompanyController;

impl CompanyController {
    pub async fn create(model_manager: &ModelManager, name: String) -> Uuid {
        sqlx::query_as::<_, (Uuid,)>("INSERT INTO companies (name) VALUES ($1) RETURNING id")
            .bind(name)
            .fetch_one(model_manager.db())
            .await
            .unwrap()
            .0
    }

    pub async fn get_all(model_manager: &ModelManager) -> Vec<Company> {
        sqlx::query_as("select * from companies")
            .fetch_all(model_manager.db())
            .await
            .unwrap()
    }

    pub async fn get_by_id(model_manager: &ModelManager, id: Uuid) -> Option<Company> {
        let row = sqlx::query_as("select * from companies where id = $1")
            .bind(id)
            .fetch_one(model_manager.db())
            .await;

        match row {
            Ok(company) => Some(company),
            Err(sqlx::Error::RowNotFound) => None,
            // TODO: replace this error by returning a Result<Option<Company>>. maybe u can replace the match with a map
            Err(_) => panic!("TODO"),
        }
    }

    // TODO: add delete and update methods (make sure to handel errors and edge cases)
}

// TODO: write test cases for CompanyController

// TODO: replace all unwraps in tests with anyhow::Result

#[cfg(test)]
mod tests {
    use crate::{controllers::CompanyController, models::ModelManager};
    use serial_test::serial;
    use std::collections::HashSet;
    use uuid::Uuid;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() {
        let model_manager = ModelManager::new().await;

        // exec
        let id = CompanyController::create(&model_manager, String::from("my-company")).await;

        // check
        let name = sqlx::query_as::<_, (String,)>("SELECT name FROM companies WHERE id = $1")
            .bind(id)
            .fetch_one(model_manager.db())
            .await
            .unwrap()
            .0;
        assert_eq!(name, "my-company");

        // clean
        sqlx::query("DELETE FROM companies WHERE id = $1")
            .bind(id)
            .execute(model_manager.db())
            .await
            .unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id() {
        let model_manager = ModelManager::new().await;

        // prepare
        let (id, name) =
            sqlx::query_as::<_, (Uuid, String)>("SELECT id, name FROM companies LIMIT 1")
                .fetch_one(model_manager.db())
                .await
                .unwrap();

        // exec
        let company = CompanyController::get_by_id(&model_manager, id)
            .await
            .unwrap();

        // check
        assert_eq!(company.name, name);
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_id_id_not_found() {
        let model_manager = ModelManager::new().await;

        // exec
        let company = CompanyController::get_by_id(&model_manager, Uuid::nil()).await;

        // check
        assert!(company.is_none());
    }

    #[serial]
    #[tokio::test]
    async fn test_get_all_ok() {
        let model_manager = ModelManager::new().await;

        // exec
        let companies = CompanyController::get_all(&model_manager).await;

        //check
        let names = companies
            .iter()
            .map(|c| c.name.as_str())
            .collect::<HashSet<_>>();
        assert_eq!(names, HashSet::from(["Al Forsan", "Al Joker", "Al Abtal",]));
    }
}
