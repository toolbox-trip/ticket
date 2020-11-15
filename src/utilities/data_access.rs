use url::Url;

use crate::error::Result;
use crate::memcache::MemcacheClient;

pub async fn get_private_pem(url: &str, date: &str) -> Result<bytes::Bytes> {
    // TODO: read config from env
    let mut client = MemcacheClient::new().connect("127.0.0.1:11211").await?;
    let result = client.get(date).await;
    match result {
        Ok(r) => {
            return Ok(bytes::Bytes::from(r))
        }
        Err(_) => {}
    }

    let url = Url::parse(url)?.join("private/pem/")?.join(date)?;
    Ok(reqwest::get(url).await?.bytes().await?)
}
