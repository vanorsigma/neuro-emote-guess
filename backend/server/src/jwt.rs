use std::sync::Arc;

use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::twitch::TwitchUserData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JWTClaim {
    pub data: TwitchUserData,
}

#[derive(Error, Debug)]
pub enum JWTClaimError {
    #[error("JWT Error: {0}")]
    ClaimGenerationError(#[from] jwt_simple::Error),
}

#[derive(Debug)]
pub struct JWTManager<K: MACLike> {
    key: Arc<K>,
}

impl<K: MACLike> JWTManager<K> {
    pub fn new(key: K) -> Self {
        JWTManager { key: key.into() }
    }

    pub fn create_user_token(
        &self,
        data: TwitchUserData,
    ) -> Result<String, JWTClaimError> {
        let claims =
            Claims::with_custom_claims(JWTClaim { data }, Duration::from_hours(2));

        Ok(self.key.authenticate(claims)?)
    }

    pub fn verify_user_token(&self, token: String) -> Result<JWTClaim, JWTClaimError> {
        tracing::debug!("token is {}", token);
        Ok(self.key.verify_token::<JWTClaim>(&token, None)?.custom)
    }
}
