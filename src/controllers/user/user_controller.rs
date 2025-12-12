use crate::{
    configs::config,
    controllers::UserControllerError,
    models::{
        ModelManager,
        api_schemas::ProfileInfo,
        dtos::{RawUserInfo, UserForInsert, UserFromRow, UserPersonalInfo},
        tables::UserRole,
    },
    secrets::SecretManager,
};
use sqlx::{FromRow, postgres::PgRow};
use uuid::Uuid;

pub struct UserController;

impl UserController {
    pub async fn get_personal_info_by_id(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<UserPersonalInfo, UserControllerError> {
        let raw_user: RawUserInfo = sqlx::query_as(
            r#"
            SELECT
                users.id,
                users.username,
                users.role,

                player.first_name,
                player.last_name,
                player.preferred_sports,

                business.display_name
            FROM users
            LEFT JOIN player_profiles player
                ON users.id = player.user_id
            LEFT JOIN business_profiles business
                ON users.id = business.user_id
            WHERE users.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(model_manager.db())
        .await
        .map_err(UserControllerError::Sqlx)?
        .ok_or(UserControllerError::UserNotFound)?;

        let profile_info = match raw_user.role {
            UserRole::Player => ProfileInfo::Player {
                first_name: raw_user
                    .first_name
                    .ok_or(UserControllerError::UnexpectedNullValueFetchedFromDb {
                        table_name: "player_profiles",
                        column_name: "first_name",
                        explanation: "user role is 'player' and this column is not nullable in the table definition",
                    })?,
                last_name: raw_user
                    .last_name
                    .ok_or(UserControllerError::UnexpectedNullValueFetchedFromDb {
                        table_name: "player_profiles",
                        column_name: "last_name",
                        explanation: "user role is 'player' and this column is not nullable in the table definition",
                    })?,
                preferred_sports: raw_user
                    .preferred_sports
                    .ok_or(UserControllerError::UnexpectedNullValueFetchedFromDb {
                        table_name: "player_profiles",
                        column_name: "preferred_sports",
                        explanation: "user role is 'player' and this column is not nullable in the table definition",
                    })?,
            },
            UserRole::Business => ProfileInfo::Business {
                display_name: raw_user
                    .display_name
                    .ok_or(UserControllerError::UnexpectedNullValueFetchedFromDb {
                        table_name: "business_profiles",
                        column_name: "display_name",
                        explanation: "user role is 'business' and this column is not nullable in the table definition",
                    })?,
            },
            UserRole::Admin => ProfileInfo::Admin,
        };

        let user_personal_info = UserPersonalInfo {
            id: raw_user.id,
            username: raw_user.username,
            profile_info,
        };

        Ok(user_personal_info)
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

    // TODO: update tests for this function
    pub async fn insert_user(
        model_manager: &ModelManager,
        user: UserForInsert<'_>,
    ) -> Result<Uuid, UserControllerError> {
        let mut password_salt = [0u8; 32];
        SecretManager::generate_salt(&mut password_salt);
        let password_hash =
            SecretManager::hash_secret(user.password, &password_salt, &config().password_key);

        let result = match user.account_type {
            ProfileInfo::Player {
                first_name,
                last_name,
                preferred_sports,
            } => {
                sqlx::query_scalar(
                    r#"
                    WITH inserted_user AS (
                        INSERT INTO users
                            (username, password_hash, password_salt, role)
                        VALUES
                            ($1, $2, $3, 'player')
                        RETURNING id
                    )
                    INSERT INTO player_profiles
                        (user_id, first_name, last_name, preferred_sports)
                    VALUES
                        (SELECT id FROM inserted_user, $4, $5, $6)
                    RETURNING user_id
                    "#,
                )
                .bind(user.username)
                .bind(password_hash)
                .bind(password_salt)
                .bind(first_name)
                .bind(last_name)
                .bind(preferred_sports)
                .fetch_one(model_manager.db())
                .await
            }
            ProfileInfo::Business { display_name } => {
                sqlx::query_scalar(
                    r#"
                    WITH inserted_user AS (
                        INSERT INTO users
                            (username, password_hash, password_salt, role)
                        VALUES
                            ($1, $2, $3, 'business')
                        RETURNING id
                    )
                    INSERT INTO business_profiles
                        (user_id, display_name)
                    VALUES
                        (SELECT id FROM inserted_user, $4)
                    RETURNING user_id
                    "#,
                )
                .bind(user.username)
                .bind(password_hash)
                .bind(password_salt)
                .bind(display_name)
                .fetch_one(model_manager.db())
                .await
            }
            ProfileInfo::Admin => todo!(),
        };

        match result {
            Ok(id) => Ok(id),
            Err(sqlx::Error::Database(err)) if err.constraint() == Some("users_username_key") => {
                Err(UserControllerError::UsernameAlreadyExists)
            }
            Err(err) => Err(UserControllerError::Sqlx(err)),
        }
    }
}

#[cfg(test)]
#[serial_test::serial] // TODO: check if any of the tests below can be run in parallel to speed up tests
mod tests {
    use crate::{
        controllers::user::{errors::UserControllerError, user_controller::UserController},
        models::{ModelManager, api_schemas::ProfileInfo, tables::Sport},
    };
    use anyhow::Context;
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
        let user = UserController::get_personal_info_by_id(&model_manager, id)
            .await
            .context("failed while fetching user")?;

        // check
        assert_eq!(user.id, id);
        assert_eq!(user.username, username);
        assert_eq!(
            user.profile_info,
            ProfileInfo::Player {
                first_name: String::from("player_1_first"),
                last_name: String::from("player_1_last"),
                preferred_sports: vec![Sport::Football],
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_personal_info_by_id(&model_manager, Uuid::new_v4()).await;

        // check
        assert!(matches!(user, Err(UserControllerError::UserNotFound)));

        Ok(())
    }
}
