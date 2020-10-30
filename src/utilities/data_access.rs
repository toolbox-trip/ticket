use url::Url;

use crate::error::Result;

pub async fn get_private_pem(url: &str, date: &str) -> Result<bytes::Bytes> {
    let url = Url::parse(url)?
        .join("private/pem/")?
        .join(date)?;
    Ok(reqwest::get(url).await?.bytes().await?)
}
