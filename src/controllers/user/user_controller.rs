use super::models::{RawUserPersonalInfo, UserPersonalInfo};
use crate::{
    controllers::{UserControllerError, user::models::UserProfile},
    models::{ModelManager, tables::UserRole},
};
use uuid::Uuid;

pub struct UserController;

impl UserController {
    pub async fn get_personal_info_by_id(
        model_manager: &ModelManager,
        id: Uuid,
    ) -> Result<UserPersonalInfo, UserControllerError> {
        let raw_user_info: RawUserPersonalInfo = sqlx::query_as(
            r#"
            SELECT
                users.id,
                users.username,
                users.role,
                -- player profile
                player.first_name,
                player.last_name,
                player.preferred_sports,
                -- business profile
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

        macro_rules! explanation {
            ($role:literal) => {
                stringify!(user role is $role and this column is not nullable in the table definition)
            };
        }

        let profile = match raw_user_info.role {
            UserRole::Player => {
                let table_name = "player_profiles";
                let explanation = explanation!("player");
                UserProfile::Player {
                    first_name: raw_user_info.first_name.ok_or(
                        UserControllerError::UnexpectedNullValueFetchedFromDb {
                            table_name,
                            column_name: "first_name",
                            explanation,
                        },
                    )?,
                    last_name: raw_user_info.last_name.ok_or(
                        UserControllerError::UnexpectedNullValueFetchedFromDb {
                            table_name,
                            column_name: "last_name",
                            explanation,
                        },
                    )?,
                    preferred_sports: raw_user_info.preferred_sports.ok_or(
                        UserControllerError::UnexpectedNullValueFetchedFromDb {
                            table_name,
                            column_name: "preferred_sports",
                            explanation,
                        },
                    )?,
                }
            }
            UserRole::Business => UserProfile::Business {
                display_name: raw_user_info.display_name.ok_or(
                    UserControllerError::UnexpectedNullValueFetchedFromDb {
                        table_name: "business_profiles",
                        column_name: "display_name",
                        explanation: explanation!("business"),
                    },
                )?,
            },
            UserRole::Admin => UserProfile::Admin,
        };

        let user_info = UserPersonalInfo {
            id,
            username: raw_user_info.username,
            profile,
        };

        Ok(user_info)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controllers::{
            UserProfile,
            user::{errors::UserControllerError, user_controller::UserController},
        },
        models::{ModelManager, tables::Sport},
    };
    use anyhow::Context;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_personal_info_by_id_ok_player() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = "player_1";
        let id = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching id")?;

        // exec
        let user_info = UserController::get_personal_info_by_id(&model_manager, id)
            .await
            .context("failed while fetching user info")?;

        // check
        assert_eq!(user_info.id, id);
        assert_eq!(user_info.username, username);
        assert_eq!(
            user_info.profile,
            UserProfile::Player {
                first_name: String::from("player_1_first"),
                last_name: String::from("player_1_last"),
                preferred_sports: vec![Sport::Football],
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_personal_info_by_id_ok_business() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = "business_2";
        let id = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching id")?;

        // exec
        let user_info = UserController::get_personal_info_by_id(&model_manager, id)
            .await
            .context("failed while fetching user info")?;

        // check
        assert_eq!(user_info.id, id);
        assert_eq!(user_info.username, username);
        assert_eq!(
            user_info.profile,
            UserProfile::Business {
                display_name: String::from("business_2_display")
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_personal_info_by_id_ok_admin() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = "admin";
        let id = sqlx::query_scalar("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(model_manager.db())
            .await
            .context("failed while fetching id")?;

        // exec
        let user_info = UserController::get_personal_info_by_id(&model_manager, id)
            .await
            .context("failed while fetching user info")?;

        // check
        assert_eq!(user_info.id, id);
        assert_eq!(user_info.username, username);
        assert_eq!(user_info.profile, UserProfile::Admin);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_personal_info_by_id_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let user = UserController::get_personal_info_by_id(&model_manager, Uuid::new_v4()).await;

        // check
        assert!(matches!(user, Err(UserControllerError::UserNotFound)));

        Ok(())
    }
}
