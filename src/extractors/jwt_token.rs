use crate::errors::ServerError;
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::{
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
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
                .add(Duration::from_secs(60 * 15)) // 15 minutes // TODO: make this configurable
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

        let Some(cookie) = cookies.get("token") else {
            return Err(ServerError::JwtTokenNotFoundInCookies);
        };

        let jwt_encoded_token = cookie.value();

        // TODO: use env var for secret, create a Config object
        let token_data = jsonwebtoken::decode::<JwtToken>(
            jwt_encoded_token.as_bytes(),
            &DecodingKey::from_secret(b"my-secret"),
            &Validation::default(),
        )
        .map_err(ServerError::JwtError)?;

        Ok(token_data.claims)
    }
}
