use crate::{
    configs::config,
    controllers::UserControllerError,
    models::{
        ModelManager, UpdateUserPersonalInfoPayload, UserForInsertUser, UserForUpdatePassword,
        UserFromRow,
    },
    secrets::SecretManager,
};
use sqlx::{FromRow, QueryBuilder, postgres::PgRow};
use uuid::Uuid;

pub struct UserController;

impl UserController {
    pub async fn get_by_id<U>(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<U, UserControllerError>
    where
        U: UserFromRow,
        U: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(model_manager.db())
            .await
            .map_err(UserControllerError::Sqlx)?
            .ok_or(UserControllerError::UserNotFound)
    }

    pub async fn get_by_username<U>(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<U, UserControllerError>
    where
        U: UserFromRow,
        U: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    {
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .map_err(UserControllerError::Sqlx)?
            .ok_or(UserControllerError::UserNotFound)
    }

    pub async fn update_by_id(
        model_manager: &ModelManager,
        id: Uuid,
        new_user_info: UpdateUserPersonalInfoPayload,
    ) -> Result<Uuid, UserControllerError> {
        let mut query_builder = QueryBuilder::new("UPDATE users SET ");
        let mut separated_query_builder = query_builder.separated(", ");

        let mut changed = false;
        if let Some(username) = new_user_info.username {
            separated_query_builder
                .push("username = ")
                .push_bind_unseparated(username);
            changed = true;
        }
        if let Some(first_name) = new_user_info.first_name {
            separated_query_builder
                .push("first_name = ")
                .push_bind_unseparated(first_name);
            changed = true;
        }
        if let Some(last_name) = new_user_info.last_name {
            separated_query_builder
                .push("last_name = ")
                .push_bind_unseparated(last_name);
            changed = true;
        }

        if !changed {
            return Ok(id);
        }

        query_builder
            .push(" WHERE id = ")
            .push_bind(id)
            .push(" RETURNING id");

        let query = query_builder.build_query_scalar();

        query
            .fetch_optional(model_manager.db())
            .await
            .map_err(UserControllerError::Sqlx)?
            .ok_or(UserControllerError::UserNotFound)
    }

    pub async fn update_password_by_username(
        model_manager: &ModelManager,
        username: &str,
        new_password: &str,
    ) -> Result<Uuid, UserControllerError> {
        let user =
            UserController::get_by_username::<UserForUpdatePassword>(model_manager, username)
                .await?;

        let new_password_hash =
            SecretManager::hash_secret(new_password, &user.password_salt, &config().password_key);

        sqlx::query_scalar("UPDATE users SET password_hash = $1 WHERE username = $2 RETURNING id")
            .bind(new_password_hash)
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .map_err(UserControllerError::Sqlx)?
            .ok_or(UserControllerError::UserNotFound)
    }

    pub async fn insert_user(
        model_manager: &ModelManager,
        user: UserForInsertUser<'_>,
    ) -> Result<Uuid, UserControllerError> {
        let mut password_salt = [0u8; 32];
        SecretManager::generate_salt(&mut password_salt);
        let password_hash =
            SecretManager::hash_secret(user.password, &password_salt, &config().password_key);

        let result = sqlx::query_scalar(
            "INSERT INTO users
                (username, role, password_hash, password_salt)
            VALUES
                ($1, $2, $3, $4)
            RETURNING id",
        )
        .bind(user.username)
        .bind(user.role)
        .bind(password_hash)
        .bind(password_salt)
        .fetch_one(model_manager.db())
        .await;

        match result {
            Ok(id) => Ok(id),
            Err(sqlx::Error::Database(err)) if err.constraint() == Some("users_username_key") => {
                Err(UserControllerError::UsernameAlreadyExists)
            }
            Err(err) => Err(UserControllerError::Sqlx(err)),
        }
    }

    // TODO: implement tests
    pub async fn delete_by_username(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<Uuid, UserControllerError> {
        sqlx::query_scalar("DELETE FROM users WHERE username = $1 RETURNING id")
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .map_err(UserControllerError::Sqlx)?
            .ok_or(UserControllerError::UserNotFound)
    }
}

#[cfg(test)]
#[serial_test::serial] // TODO: check if any of the tests below can be run in parallel to speed up tests
mod tests {
    use crate::{
        configs::config,
        controllers::user::{errors::UserControllerError, user_controller::UserController},
        models::{ModelManager, User, UserForInsertUser, UserRole},
        secrets::SecretManager,
    };
    use anyhow::Context;
    use rand::distr::{Alphanumeric, SampleString};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_by_id_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = "player_1";
        let id = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching id")?;

        // exec
        let user = UserController::get_by_id::<User>(&model_manager, id)
            .await
            .context("failed while fetching user")?;

        // check
        assert_eq!(user.id, id);
        assert_eq!(user.username, username);
        assert!(matches!(user.role, UserRole::Player));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_by_id::<User>(&model_manager, Uuid::new_v4()).await;

        // check
        assert!(matches!(user, Err(UserControllerError::UserNotFound)));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_username_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let username = "business_1";
        let user = UserController::get_by_username::<User>(&model_manager, username)
            .await
            .context("failed while fetching user")?;

        // check
        assert_eq!(user.username, username);
        assert!(matches!(user.role, UserRole::Business));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_username_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user =
            UserController::get_by_username::<User>(&model_manager, "invalid username").await;

        // check
        assert!(matches!(user, Err(UserControllerError::UserNotFound)));

        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_by_username_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = "player_2";
        let (old_password_hash, password_salt): (Vec<u8>, Vec<u8>) =
            sqlx::query_as("SELECT password_hash, password_salt FROM users WHERE username = $1")
                .bind(username)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching user")?;

        // exec
        let new_password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let id =
            UserController::update_password_by_username(&model_manager, username, &new_password)
                .await
                .context("failed while updating password")?;

        // check
        let (expected_id, new_password_hash): (Uuid, Vec<u8>) =
            sqlx::query_as("SELECT id, password_hash FROM users WHERE username = $1")
                .bind(username)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching user")?;

        assert_eq!(id, expected_id);

        SecretManager::verify_secret(
            &new_password,
            &password_salt,
            &config().password_key,
            &new_password_hash,
        )
        .context("password was not updated correctly")?;

        // clean
        let rows_affected = sqlx::query("UPDATE users SET password_hash = $1 WHERE username = $2")
            .bind(old_password_hash)
            .bind(username)
            .execute(model_manager.db())
            .await
            .context("failed while updating password to original value")?
            .rows_affected();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_password_by_username_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let result =
            UserController::update_password_by_username(&model_manager, "invalid username", "")
                .await;

        // check
        assert!(matches!(result, Err(UserControllerError::UserNotFound)));

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_user_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let user = UserForInsertUser {
            username: &username,
            password: &password,
            role: UserRole::Admin,
        };

        // exec
        let id = UserController::insert_user(&model_manager, user)
            .await
            .context("failed while inserting user")?;

        // check
        let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching user")?;

        SecretManager::verify_secret(
            &password,
            &user.password_salt,
            &config().password_key,
            &user.password_hash,
        )
        .context("password is wrong")?;

        assert_eq!(user.id, id);
        assert_eq!(user.username, username);
        assert!(matches!(user.role, UserRole::Admin));

        // clean
        let rows_affected = sqlx::query("DELETE FROM users WHERE username = $1")
            .bind(username)
            .execute(model_manager.db())
            .await
            .context("failed while deleting user")?
            .rows_affected();
        assert_eq!(rows_affected, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_user_err_user_already_exists() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let user = UserForInsertUser {
            username: "business_2",
            password: "",
            role: UserRole::Player,
        };

        // exec
        let result = UserController::insert_user(&model_manager, user).await;

        // check
        assert!(matches!(
            result,
            Err(UserControllerError::UsernameAlreadyExists)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_username_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let inserted_user_id: Uuid = sqlx::query_scalar(
            "
            INSERT INTO
                users (username, role, password_hash, password_salt)
            VALUES
                ($1, 'admin', '\\x00', '\\x11')
            RETURNING id
            ",
        )
        .bind(&username)
        .fetch_one(model_manager.db())
        .await
        .context("failed while inserting user")?;

        // exec
        let deleted_user_id = UserController::delete_by_username(&model_manager, &username)
            .await
            .context("failed while deleting user")?;

        // check
        assert_eq!(inserted_user_id, deleted_user_id);

        let result = sqlx::query("SELECT * FROM users WHERE id = $1 OR username = $2")
            .bind(inserted_user_id)
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .context("failed while fetching user")?;
        assert!(result.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_by_username_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let result = UserController::delete_by_username(&model_manager, &username).await;

        // check
        assert!(matches!(result, Err(UserControllerError::UserNotFound)));

        let result = sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(model_manager.db())
            .await
            .context("failed while fetching user")?;
        assert!(result.is_none());

        Ok(())
    }
}
