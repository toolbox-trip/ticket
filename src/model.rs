use crate::jwk::JwkPemAccessor;
use serde::{Deserialize, Serialize};

pub struct GlobalContext {
    pub jgen_url: String,
    pub redis_url: String,

    pub jwk_accessor: JwkPemAccessor,
}

impl GlobalContext {
    pub fn default() -> Self {
        let jgen_url = std::env::var("JGEN_URL").unwrap();
        let redis_url = std::env::var("REDIS_URL").unwrap();
        let jwk_accessor = JwkPemAccessor::new(&jgen_url, &redis_url);
        GlobalContext {
            jgen_url,
            redis_url,
            jwk_accessor,
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
