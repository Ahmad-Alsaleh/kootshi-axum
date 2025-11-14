use crate::{configs::config, errors::ServerError};
use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    ops::Add,
    time::{SystemTime, UNIX_EPOCH},
};
use tower_cookies::Cookies;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub user_id: Uuid,
    exp: u64,
}

impl AuthToken {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("UNIX_EPOCH is in the past")
                .add(config().auth_token_exp_duration)
                .as_secs(),
        }
    }
}

impl<S> FromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<Cookies>()
            .await
            .expect("`CookieManagerLayer` is enabled");

        let Some(cookie) = cookies.get("auth-token") else {
            return Err(ServerError::AuthTokenNotFoundInCookies);
        };

        let encoded_auth_token = cookie.value();

        // TODO: consider using an auth_token_salt
        let token_data = jsonwebtoken::decode::<AuthToken>(
            encoded_auth_token.as_bytes(),
            &DecodingKey::from_secret(&config().auth_token_key),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims)
    }
}

impl<S> OptionalFromRequestParts<S> for AuthToken
where
    S: Send + Sync,
{
    type Rejection = (); // using the unit type since an error will never occur

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let auth_token = <Self as FromRequestParts<S>>::from_request_parts(parts, state).await;
        Ok(auth_token.ok())
    }
}
