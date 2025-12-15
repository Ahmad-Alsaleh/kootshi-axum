use crate::{
    configs::config,
    controllers::{
        UserControllerError,
        user::models::{
            RawUserPersonalInfo, UserForInsert, UserLoginInfo, UserPersonalInfo, UserProfile,
        },
    },
    models::{ModelManager, tables::UserRole},
    secrets::SecretManager,
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

    pub async fn get_login_info_by_username(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<UserLoginInfo, UserControllerError> {
        sqlx::query_as(
            "SELECT id, role, password_hash, password_salt FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(model_manager.db())
        .await
        .map_err(UserControllerError::Sqlx)?
        .ok_or(UserControllerError::UserNotFound)
    }

    pub async fn insert_user(
        model_manager: &ModelManager,
        user: UserForInsert<'_>,
    ) -> Result<Uuid, UserControllerError> {
        let mut password_salt = [0u8; 32];
        SecretManager::generate_salt(&mut password_salt);
        let password_hash =
            SecretManager::hash_secret(user.password, &password_salt, &config().password_key);

        let query = match user.profile {
            UserProfile::Player {
                first_name,
                last_name,
                preferred_sports,
            } => sqlx::query_scalar(
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
                    ((SELECT id FROM inserted_user), $4, $5, $6)
                RETURNING user_id
                "#,
            )
            .bind(user.username)
            .bind(password_hash)
            .bind(password_salt)
            .bind(first_name)
            .bind(last_name)
            .bind(preferred_sports),
            UserProfile::Business { display_name } => sqlx::query_scalar(
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
                    ((SELECT id FROM inserted_user), $4)
                RETURNING user_id
                "#,
            )
            .bind(user.username)
            .bind(password_hash)
            .bind(password_salt)
            .bind(display_name),
            UserProfile::Admin => sqlx::query_scalar(
                r#"
                INSERT INTO users
                    (username, password_hash, password_salt, role)
                VALUES
                    ($1, $2, $3, 'admin')
                RETURNING id
                "#,
            )
            .bind(user.username)
            .bind(password_hash)
            .bind(password_salt),
        };

        let result = query.fetch_one(model_manager.db()).await;

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
mod tests {
    use crate::{
        configs::config,
        controllers::{
            UserForInsert, UserProfile,
            user::{errors::UserControllerError, user_controller::UserController},
        },
        models::{
            ModelManager,
            tables::{BusinessProfile, PlayerProfile, Sport, User, UserRole},
        },
        secrets::SecretManager,
    };
    use anyhow::Context;
    use rand::distr::{Alphanumeric, SampleString};
    use uuid::{Uuid, uuid};

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

    #[tokio::test]
    async fn test_get_login_info_by_username_ok() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let username = "business_2";
        let user_login_info = UserController::get_login_info_by_username(&model_manager, username)
            .await
            .context("failed while fetching user")?;

        // check
        assert_eq!(
            user_login_info.id,
            uuid!("00000000-0000-0000-0000-000000000004")
        );

        assert_eq!(user_login_info.role, UserRole::Business);

        let password_hash: String = user_login_info
            .password_hash
            .iter()
            .map(|value| format!("{value:02x}"))
            .collect();
        assert_eq!(
            password_hash,
            "3e25a17318adce535c262e24895c98b6725ca123bd968e50480a275a59e671bf"
        );

        let password_salt: String = user_login_info
            .password_salt
            .iter()
            .map(|value| format!("{value:02x}"))
            .collect();
        assert_eq!(
            password_salt,
            "b24f91a3914b017bdc9e7ba00bc5c0ae160d03e87bc627511d828de58c6c65e9"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_get_login_info_by_username_err_user_not_found() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // exec
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let user_login_info =
            UserController::get_login_info_by_username(&model_manager, &username).await;

        // check
        assert!(matches!(
            user_login_info,
            Err(UserControllerError::UserNotFound)
        ));

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_user_ok_player() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let profile = UserProfile::Player {
            first_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
            last_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
            preferred_sports: vec![Sport::Football, Sport::Basketball],
        };
        let user = UserForInsert {
            username: &username,
            password: &password,
            profile: &profile,
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
        assert_eq!(user.role, UserRole::Player);

        let expected_profile: PlayerProfile =
            sqlx::query_as("SELECT * FROM player_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching player profile")?;
        let expected_profile = UserProfile::Player {
            first_name: expected_profile.first_name,
            last_name: expected_profile.last_name,
            preferred_sports: expected_profile.preferred_sports,
        };
        assert_eq!(expected_profile, profile);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM business_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching business profile")?;
        assert_eq!(count, 0);

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
    async fn test_insert_user_ok_business() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let profile = UserProfile::Business {
            display_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
        };
        let user = UserForInsert {
            username: &username,
            password: &password,
            profile: &profile,
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
        assert_eq!(user.role, UserRole::Business);

        let expected_profile: BusinessProfile =
            sqlx::query_as("SELECT * FROM business_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching business profile")?;
        let expected_profile = UserProfile::Business {
            display_name: expected_profile.display_name,
        };
        assert_eq!(expected_profile, profile);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM player_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching player profile")?;
        assert_eq!(count, 0);

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
    async fn test_insert_user_ok_admin() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let profile = UserProfile::Admin;
        let user = UserForInsert {
            username: &username,
            password: &password,
            profile: &profile,
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
        assert_eq!(user.role, UserRole::Admin);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM player_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching player profile")?;
        assert_eq!(count, 0);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM business_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching business profile")?;
        assert_eq!(count, 0);

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
        let user = UserForInsert {
            username: "business_2",
            password: "",
            profile: &UserProfile::Admin,
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
}
