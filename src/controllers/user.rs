use crate::models::{FromUser, ModelManager};
use sqlx::{FromRow, postgres::PgRow};

pub struct UserController;

impl UserController {
    pub async fn get_by_username<U>(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<Option<U>, sqlx::Error>
    where
        U: FromUser,
        U: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    {
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controllers::UserController,
        models::{ModelManager, User},
    };
    use anyhow::Context;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_by_username_user_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_by_username::<User>(&model_manager, "mohammed.hassan")
            .await
            .context("failed while fetching user")?
            .context("user was not found")?;

        // check
        assert_eq!(user.username, "mohammed.hassan");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_username_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_by_username::<User>(&model_manager, "invalid username")
            .await
            .context("failed while fetching user")?;

        // check
        assert!(user.is_none());

        Ok(())
    }
}
