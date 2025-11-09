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
pub struct JwtToken {
    pub user_id: Uuid,
    exp: u64,
}

impl JwtToken {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("UNIX_EPOCH is in the past")
                .add(config().jwt_exp_duration)
                .as_secs(),
        }
    }
}

impl<S> FromRequestParts<S> for JwtToken
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
            return Err(ServerError::JwtTokenNotFoundInCookies);
        };

        let jwt_encoded_token = cookie.value();

        let token_data = jsonwebtoken::decode::<JwtToken>(
            jwt_encoded_token.as_bytes(),
            &DecodingKey::from_secret(&config().jwt_secret),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(ServerError::JwtError)?;

        Ok(token_data.claims)
    }
}

impl<S> OptionalFromRequestParts<S> for JwtToken
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let jwt_token = <Self as FromRequestParts<S>>::from_request_parts(parts, state).await;
        Ok(jwt_token.ok())
    }
}
