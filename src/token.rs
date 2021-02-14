use crate::model::{Claims, ConfigContext};
use anyhow::Result;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde_json::Value;

pub async fn generate_token(ctx: &ConfigContext) -> Result<String> {
    let pem = ctx.jwk_accessor.get_current_pem().await?;

    let pem_bytes = pem.pem.private_pem.as_bytes();

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(pem.timestamp);
    let key = EncodingKey::from_rsa_pem(pem_bytes)?;

    Ok(encode(&header, &Claims::generate_claims("user"), &key)?)
}

pub async fn all_jwk(ctx: &ConfigContext) -> Result<String> {
    let pems = ctx.jwk_accessor.get_all_pem().await?;
    let pem_vec: Vec<Value> = pems.iter().map(|pem| pem.pem.public_jwk.clone()).collect();
    let json_str = serde_json::to_string(&pem_vec)?;
    Ok(json_str)
}
