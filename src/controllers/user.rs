use crate::{
    configs::config,
    controllers::ControllerError,
    models::{FromUser, ModelManager, UserForUpdatePassword},
    secrets::SecretManager,
};
use sqlx::{FromRow, postgres::PgRow};

pub struct UserController;

impl UserController {
    pub async fn get_by_username<U>(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<U, ControllerError>
    where
        U: FromUser,
        U: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    {
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .map_err(ControllerError::Sqlx)?
            .ok_or(ControllerError::UserNotFound)
    }

    pub async fn update_password_hash_by_username(
        model_manager: &ModelManager,
        username: &str,
        new_password: &str,
    ) -> Result<(), ControllerError> {
        let user =
            UserController::get_by_username::<UserForUpdatePassword>(model_manager, username)
                .await?;

        let password_hash =
            SecretManager::hash_secret(new_password, &user.password_salt, &config().password_key);

        let rows_affected = sqlx::query("UPDATE users SET password_hash = $1 WHERE username = $2")
            .bind(password_hash)
            .bind(username)
            .execute(model_manager.db())
            .await
            .map_err(ControllerError::Sqlx)?
            .rows_affected();

        if rows_affected == 0 {
            return Err(ControllerError::UserNotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controllers::{ControllerError, UserController},
        models::ModelManager,
        secrets::SecretManager,
    };
    use anyhow::Context;
    use serial_test::serial;
    use uuid::Uuid;

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

    #[serial]
    #[tokio::test]
    async fn test_update_password_by_username_user_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let old_password: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE username = 'ahmad.alsaleh'")
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching user")?;

        // exec
        let salt = SecretManager::generate_salt();
        let random_password = Uuid::new_v4().to_string(); // any random value, uuid is suffecient
        UserController::update_password_hash_by_username(
            &model_manager,
            "ahmad.alsaleh",
            &random_password,
        )
        .await
        .context("failed while updating password")?;

        // check
        let new_password: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE username = 'ahmad.alsaleh'")
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching user")?;
        assert_eq!(new_password, random_password);

        // clean
        UserController::update_password_hash_by_username(
            &model_manager,
            "ahmad.alsaleh",
            &old_password,
        )
        .await
        .context("failed while updating password")?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_password_by_username_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let result = UserController::update_password_hash_by_username(
            &model_manager,
            "invalid username",
            "new password",
        )
        .await;

        // check
        assert!(matches!(result, Err(ControllerError::UserNotFound)));

        Ok(())
    }
}
