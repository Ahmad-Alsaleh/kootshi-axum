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

// TODO: put extractors file in src/extractors.rs not src/models/extractors.rs
// then, renam Context to JwtToken

#[derive(Serialize, Deserialize, Clone)]
pub struct Context {
    user_id: Uuid,
    exp: u64,
}

impl Context {
    pub fn new(user_id: Uuid) -> Self {
        // SystemTime::now().duration_since(UNIX_EPOCH).unwrap().add(rhs)
        Self {
            user_id,
            // FIXME: the below todo doesn't work
            // TODO: set exp to 2 seconds and call /companies with and without a 2 seconds delay
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("UNIX_EPOCH is in the past")
                .add(Duration::from_secs(1)) // 15 minutes
                .as_secs(),
        }
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
}

impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = ServerError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: replace all unwraps
        let cookies = parts
            .extract::<Cookies>()
            .await
            .expect("`CookieManagerLayer` is enabled");

        let cookie = cookies.get("token").unwrap(); // TODO: return a server error here like `ServerError::TokenNotFoundInCookies` which maps to `ClientError::Unauthorized` or `ClientError::LoginNeeded`
        let jwt_encoded_token = cookie.value();

        // TODO: use env var for secret, create a Config object
        let token_data = jsonwebtoken::decode::<Context>(
            jwt_encoded_token.as_bytes(),
            &DecodingKey::from_secret(b"my-secret"),
            &Validation::default(),
        )
        .map_err(ServerError::JwtError)?;

        Ok(token_data.claims)
    }
}
