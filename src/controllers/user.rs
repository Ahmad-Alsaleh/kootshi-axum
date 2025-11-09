use crate::models::{LoginPayload, ModelManager, User};

pub struct UserController;

impl UserController {
    pub async fn get(
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
