use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use super::data_access::get_private_pem;
use super::date::get_date;

use crate::error::Result;
use crate::model::{Claims, ConfigContext};

pub async fn generate_token(ctx: &ConfigContext) -> Result<String> {
    let url = ctx.jgen_url();
    let date = get_date();
    let pem = get_private_pem(&url, date.as_str()).await?;
    let pem_bytes = pem.as_ref();

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some(date);
    let key = EncodingKey::from_rsa_pem(pem_bytes)?;

    Ok(encode(&header, &Claims::generate_claims("user"), &key)?)
}
