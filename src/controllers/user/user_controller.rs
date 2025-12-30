use crate::{
    configs::config,
    controllers::{
        UserControllerError,
        user::models::{
            RawAdminUser, RawBusinessUser, RawPlayerUser, UpdateUserInfoPayload,
            UpdateUserProfilePayload, UserForInsert, UserLoginInfo, UserPersonalInfo, UserProfile,
        },
    },
    models::{
        ModelManager,
        tables::{BusinessProfile, PlayerProfile, UserRole},
    },
    secrets::SecretManager,
};
use sqlx::QueryBuilder;
use uuid::Uuid;

pub struct UserController;

impl UserController {
    pub async fn get_personal_info_by_id(
        model_manager: &ModelManager,
        id: Uuid,
        user_role: UserRole,
    ) -> Result<UserPersonalInfo, UserControllerError> {
        let user = match user_role {
            UserRole::Player => {
                let raw_player_user: RawPlayerUser = sqlx::query_as(
                    r#"
                    SELECT
                        users.username,
                        player.first_name,
                        player.last_name,
                        player.preferred_sports
                    FROM
                        users
                    JOIN
                        player_profiles player
                    ON
                        player.user_id = users.id
                    WHERE
                        users.id = $1
                    AND
                        users.role = 'player'
                    "#,
                )
                .bind(id)
                .fetch_optional(model_manager.db())
                .await
                .map_err(UserControllerError::Sqlx)?
                .ok_or(UserControllerError::UserNotFound)?;
                UserPersonalInfo {
                    id,
                    username: raw_player_user.username,
                    profile: UserProfile::Player(raw_player_user.profile),
                }
            }
            UserRole::Business => {
                let raw_business_user: RawBusinessUser = sqlx::query_as(
                    r#"
                    SELECT
                        users.username,
                        business.display_name
                    FROM
                        users
                    JOIN
                        business_profiles business
                    ON
                        business.user_id = users.id
                    WHERE
                        users.id = $1
                    AND
                        users.role = 'business'
                    "#,
                )
                .bind(id)
                .fetch_optional(model_manager.db())
                .await
                .map_err(UserControllerError::Sqlx)?
                .ok_or(UserControllerError::UserNotFound)?;
                UserPersonalInfo {
                    id,
                    username: raw_business_user.username,
                    profile: UserProfile::Business(raw_business_user.profile),
                }
            }
            UserRole::Admin => {
                let raw_admin_user: RawAdminUser = sqlx::query_as(
                    r#"
                    SELECT
                        users.username
                    FROM
                        users
                    WHERE
                        users.id = $1
                    AND
                        users.role = 'admin'
                    "#,
                )
                .bind(id)
                .fetch_optional(model_manager.db())
                .await
                .map_err(UserControllerError::Sqlx)?
                .ok_or(UserControllerError::UserNotFound)?;
                UserPersonalInfo {
                    id,
                    username: raw_admin_user.username,
                    profile: UserProfile::Admin,
                }
            }
        };

        Ok(user)
    }

    pub async fn get_login_info_by_username(
        model_manager: &ModelManager,
        username: &str,
    ) -> Result<UserLoginInfo, UserControllerError> {
        sqlx::query_as(
            r#"
            SELECT
                id, role, password_hash, password_salt
            FROM
                users
            WHERE
                username = $1
            "#,
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
            UserProfile::Player(PlayerProfile {
                first_name,
                last_name,
                preferred_sports,
            }) => sqlx::query_scalar(
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
            UserProfile::Business(BusinessProfile { display_name }) => sqlx::query_scalar(
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
            Err(sqlx::Error::Database(err))
                if err.constraint() == Some("business_profiles_display_name_key") =>
            {
                Err(UserControllerError::BusinessDisplayNameAlreadyExists)
            }
            Err(err) => Err(UserControllerError::Sqlx(err)),
        }
    }

    pub async fn update_by_id(
        model_manager: &ModelManager,
        id: Uuid,
        user_role: UserRole,
        new_user_info: UpdateUserInfoPayload,
    ) -> Result<(), UserControllerError> {
        // TODO: use helper functions to build each of the two queries

        let mut query_builder = QueryBuilder::new("UPDATE users SET ");
        let mut separated_query_builder = query_builder.separated(", ");

        let mut is_users_table_updated = false;

        if let Some(username) = new_user_info.username {
            is_users_table_updated = true;
            separated_query_builder
                .push("username = ")
                .push_bind_unseparated(username);
        }

        if let Some(password) = new_user_info.password {
            is_users_table_updated = true;
            // TODO: check if it's fine to update the salt when the password is updated.
            // updating it will make the code more effecient as i don't need to first fetch the salt
            // before updating the password
            let mut password_salt = [0u8; 32];
            SecretManager::generate_salt(&mut password_salt);
            let password_hash =
                SecretManager::hash_secret(&password, &password_salt, &config().password_key);
            separated_query_builder
                .push("password_hash = ")
                .push_bind_unseparated(password_hash)
                .push("password_salt = ")
                .push_bind_unseparated(password_salt);
        }

        let update_users_table_query = is_users_table_updated.then(|| {
            query_builder
                .push(" WHERE id = ")
                .push_bind(id)
                .push(" AND role = ")
                .push_bind(user_role)
                .build()
        });

        // ---

        let mut query_builder = QueryBuilder::new("");
        let mut separated_query_builder = query_builder.separated(", ");
        let mut is_profiles_table_updated = false;

        if let Some(profile) = new_user_info.profile {
            match (profile, user_role) {
                (UpdateUserProfilePayload::Player(profile), UserRole::Player) => {
                    separated_query_builder.push_unseparated("UPDATE player_profiles SET ");

                    if let Some(frist_name) = profile.first_name {
                        is_profiles_table_updated = true;
                        separated_query_builder
                            .push("first_name = ")
                            .push_bind_unseparated(frist_name);
                    }

                    if let Some(last_name) = profile.last_name {
                        is_profiles_table_updated = true;
                        separated_query_builder
                            .push("last_name = ")
                            .push_bind_unseparated(last_name);
                    }

                    if let Some(preferred_sports) = profile.preferred_sports {
                        is_profiles_table_updated = true;
                        separated_query_builder
                            .push("preferred_sports = ")
                            .push_bind_unseparated(preferred_sports);
                    }
                }
                (UpdateUserProfilePayload::Business(profile), UserRole::Business) => {
                    separated_query_builder.push_unseparated("UPDATE business_profiles SET ");

                    if let Some(display_name) = profile.display_name {
                        is_profiles_table_updated = true;
                        separated_query_builder
                            .push("display_name = ")
                            .push_bind_unseparated(display_name);
                    }
                }
                _ => panic!("invalid profile"), // TODO: return an error "invalid profile" or the like
            }
        }

        let update_profiles_table_query = is_profiles_table_updated.then(|| {
            query_builder
                .push(" WHERE user_id = ")
                .push_bind(id)
                .build()
        });

        // ---

        if !(is_users_table_updated || is_profiles_table_updated) {
            return Ok(());
        }

        let mut transaction = model_manager
            .db()
            .begin()
            .await
            .map_err(UserControllerError::Sqlx)?;

        if let Some(query) = update_users_table_query {
            let rows_affected = query
                .execute(&mut *transaction)
                .await
                .map_err(UserControllerError::Sqlx)?
                .rows_affected();
            if rows_affected != 1 {
                return Err(UserControllerError::UserNotFound);
            }
        }

        if let Some(query) = update_profiles_table_query {
            let rows_affected = query
                .execute(&mut *transaction)
                .await
                .map_err(UserControllerError::Sqlx)?
                .rows_affected();
            if rows_affected != 1 {
                return Err(UserControllerError::UserNotFound);
            }
        }

        transaction
            .commit()
            .await
            .map_err(UserControllerError::Sqlx)?;

        Ok(())
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
        let user_info =
            UserController::get_personal_info_by_id(&model_manager, id, UserRole::Player)
                .await
                .context("failed while fetching user info")?;

        // check
        assert_eq!(user_info.id, id);
        assert_eq!(user_info.username, username);
        assert_eq!(
            user_info.profile,
            UserProfile::Player(PlayerProfile {
                first_name: String::from("player_1_first"),
                last_name: String::from("player_1_last"),
                preferred_sports: vec![Sport::Football],
            })
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
        let user_info =
            UserController::get_personal_info_by_id(&model_manager, id, UserRole::Business)
                .await
                .context("failed while fetching user info")?;

        // check
        assert_eq!(user_info.id, id);
        assert_eq!(user_info.username, username);
        assert_eq!(
            user_info.profile,
            UserProfile::Business(BusinessProfile {
                display_name: String::from("business_2_display")
            })
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
        let user_info =
            UserController::get_personal_info_by_id(&model_manager, id, UserRole::Admin)
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
        let result = UserController::get_personal_info_by_id(
            &model_manager,
            Uuid::new_v4(),
            UserRole::Admin,
        )
        .await;

        // check
        assert!(
            matches!(result, Err(UserControllerError::UserNotFound)),
            "result: {result:?}"
        );

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
        let result = UserController::get_login_info_by_username(&model_manager, &username).await;

        // check
        assert!(matches!(result, Err(UserControllerError::UserNotFound)));

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_user_ok_player() -> anyhow::Result<()> {
        let model_manager = ModelManager::new().await;

        // prepare
        let username = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let password = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let profile = PlayerProfile {
            first_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
            last_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
            preferred_sports: vec![Sport::Football, Sport::Basketball],
        };
        let profile = UserProfile::Player(profile);
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

        let fetched_profile: PlayerProfile =
            sqlx::query_as("SELECT * FROM player_profiles WHERE user_id = $1")
                .bind(id)
                .fetch_one(model_manager.db())
                .await
                .context("failed while fetching player profile")?;
        let fetched_profile = UserProfile::Player(fetched_profile);
        assert_eq!(fetched_profile, profile);

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
        let profile = UserProfile::Business(BusinessProfile {
            display_name: Alphanumeric.sample_string(&mut rand::rng(), 16),
        });
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
        let expected_profile = UserProfile::Business(BusinessProfile {
            display_name: expected_profile.display_name,
        });
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
