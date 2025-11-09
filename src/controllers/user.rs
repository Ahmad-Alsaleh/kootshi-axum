use crate::models::{LoginPayload, ModelManager, User};

pub struct UserController;

impl UserController {
    pub async fn get_by_login_payload(
        model_manager: &ModelManager,
        login_payload: LoginPayload,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM users WHERE username = $1 AND password = $2")
            .bind(login_payload.username)
            .bind(login_payload.password)
            .fetch_optional(model_manager.db())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controllers::UserController,
        models::{LoginPayload, ModelManager},
    };
    use anyhow::Context;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_by_login_payload_user_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_by_login_payload(
            &model_manager,
            LoginPayload {
                username: String::from("ahmad.alsaleh"),
                password: String::from("passme"),
            },
        )
        .await
        .context("failed while getting user")?;

        // check
        assert!(user.is_some());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_login_payload_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_by_login_payload(
            &model_manager,
            LoginPayload {
                username: String::from("ahmad.alsaleh"),
                password: String::from("wrong password"),
            },
        )
        .await
        .context("failed while getting user")?;

        // check
        assert!(user.is_none());

        Ok(())
    }
}
