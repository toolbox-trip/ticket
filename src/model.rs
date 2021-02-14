use crate::jwk::JwkPemAccessor;
use serde::{Deserialize, Serialize};

pub struct ConfigContext {
    pub jgen_url: String,
    pub redis_url: String,

    pub jwk_accessor: JwkPemAccessor,
}

impl ConfigContext {
    pub fn default() -> Self {
        let jgen_url = "http://localhost:8000";
        let redis_url = "redis://localhost";
        ConfigContext {
            jgen_url: jgen_url.to_owned(),
            redis_url: redis_url.to_owned(),
            jwk_accessor: JwkPemAccessor::new(jgen_url, redis_url),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: i64, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: i64, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn generate_claims(user: &str) -> Self {
        Claims {
            aud: "snail".to_string(),
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
            iat: chrono::Utc::now().timestamp(),
            iss: "ticket".to_string(),
            nbf: chrono::Utc::now().timestamp(),
            sub: user.to_string(),
        }
    }
}
