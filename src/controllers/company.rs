use crate::models::{Company, ModelManager};
use uuid::Uuid;

pub struct CompanyController;

impl CompanyController {
    pub async fn create(model_manager: ModelManager, name: String) -> Uuid {
        sqlx::query_as::<_, (Uuid,)>(
            "INSERT INTO companies (name, fields) VALUES ($1) RETURNING id",
        )
        .bind(name)
        .fetch_one(model_manager.db())
        .await
        .unwrap()
        .0
    }

    pub async fn get_all(model_manager: ModelManager) -> Vec<Company> {
        sqlx::query_as("select * from companies")
            .fetch_all(model_manager.db())
            .await
            .unwrap()
    }

    pub async fn get_by_id(model_manager: ModelManager, id: Uuid) -> Option<Company> {
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
