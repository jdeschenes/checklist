use eyre::{eyre, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_hours: u64,
}

impl JwtService {
    pub fn new(secret: &str, expiration_hours: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiration_hours,
        }
    }

    pub fn generate_token(&self, user_id: i32, email: &str) -> Result<String> {
        let now = OffsetDateTime::now_utc();
        let exp = now + time::Duration::hours(self.expiration_hours as i64);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: exp.unix_timestamp(),
            iat: now.unix_timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| eyre!("Failed to generate JWT token: {}", e))
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map_err(|e| eyre!("Invalid JWT token: {}", e))?;

        Ok(token_data.claims)
    }
}
