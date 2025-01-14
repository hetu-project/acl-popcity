use crate::common::consts;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Result, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHandler {
    pub secret: String,
}

impl JwtHandler {
    pub fn create_token(&self, user_uid: &str, user_name: &str, user_email: &str) -> String {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(consts::JWT_EXPIRATION))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_uid.into(),
            name: user_name.into(),
            email: user_email.into(),
            exp: expiration,
        };

        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(self.secret.clone().as_ref());

        encode(&header, &claims, &encoding_key).unwrap_or_default()
    }

    pub fn decode_token(&self, token: String) -> Result<Claims> {
        let decoding_key = DecodingKey::from_secret(self.secret.clone().as_ref());
        let validation = Validation::default();

        decode::<Claims>(&token, &decoding_key, &validation).map(|data| data.claims)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub exp: usize,
}
