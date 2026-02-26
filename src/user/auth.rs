use axum::{extract::{Extension, FromRequestParts}, http::{header::AUTHORIZATION, request::Parts}};
use crate::user::ApiContext;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;
use crate::{error::{Error, Result}};
use std::time::Duration;
use sqlx::types::chrono::Utc;

pub struct AuthUser {
    pub user_id: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthUserClaim{
    pub user_id: Uuid,
    exp: i64,
}

impl<S> FromRequestParts<S> for AuthUserClaim
where
    S: Send + Sync,  
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let Extension(ctx) = Extension::<ApiContext>::from_request_parts(parts, state)
            .await
            .map_err(|_| Error::Unauthorized)?;

        let auth_header = parts.headers.get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        let auth_str = auth_header.to_str()
            .map_err(|_| Error::Unauthorized)?;

        let token = auth_str.strip_prefix("Bearer ")
            .ok_or(Error::Unauthorized)?;

        let validation = Validation::new(Algorithm::HS384);

        let key = DecodingKey::from_secret(ctx.config.hmac_key.as_bytes());
        let token_data = decode::<AuthUserClaim>(token, &key, &validation)
            .map_err(|_| Error::Unauthorized)?;

        Ok(token_data.claims)
    }
}

pub fn generate_token (user_id: &Uuid, hmac_key: &str) -> Result<String>{
    let claims = AuthUserClaim {
        user_id: *user_id,
        exp: (Utc::now() + Duration::from_secs(60 * 60 * 24)).timestamp() as i64,
    };

    let header = Header::new(Algorithm::HS384);
    let key = EncodingKey::from_secret(hmac_key.as_bytes());

    encode(&header, &claims, &key).map_err(|_| Error::Unauthorized)
}