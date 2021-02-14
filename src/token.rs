use crate::model::{Claims, ConfigContext};
use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
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

pub async fn validate_jwt(ctx: &ConfigContext, jwt: &str) -> Result<bool> {
    let pems = ctx.jwk_accessor.get_all_pem().await?;
    if let Ok(b) = verify(&pems[0].pem.public_pem, jwt) {
        Ok(true)
    } else if let Ok(b) = verify(&pems[1].pem.public_pem, jwt) {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn verify(pem: &str, jwt: &str) -> Result<bool> {
    let bytes = pem.as_bytes();
    let key = DecodingKey::from_rsa_pem(bytes)?;
    match decode::<Claims>(
        jwt,
        &key,
        &Validation {
            algorithms: vec![Algorithm::RS256],
            ..Default::default()
        },
    ) {
        Ok(ok) => Ok(true),
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}
