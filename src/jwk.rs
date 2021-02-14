use crate::date;
use anyhow::Result;
use redis::{Client, Commands, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{cell::RefCell, convert::TryFrom};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct B64Pem {
    pub public_jwk: Value,
    pub private_jwk: Value,
    pub private_pem: String,
    pub public_pem: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pem {
    pub public_jwk: Value,
    pub private_jwk: Value,
    pub private_pem: String,
    pub public_pem: String,
}

impl TryFrom<B64Pem> for Pem {
    type Error = anyhow::Error;

    fn try_from(value: B64Pem) -> Result<Self, Self::Error> {
        Ok(Self {
            public_jwk: value.public_jwk,
            private_jwk: value.private_jwk,
            public_pem: String::from_utf8(base64::decode(value.public_pem)?)?,
            private_pem: String::from_utf8(base64::decode(value.private_pem)?)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct PeriodicalPem {
    pub pem: Pem,
    pub timestamp: String,
}

pub struct JwkPemAccessor {
    jgen_url: Url,
    redis_url: String,
    redis_client: RefCell<Option<Client>>,
    redis_connection: RefCell<Option<Connection>>,
}

impl JwkPemAccessor {
    pub fn new(jgen_url: &str, redis_url: &str) -> Self {
        let url = Url::parse(jgen_url)
            .expect("jgen_url should be in url format")
            .join("key")
            .unwrap();
        Self {
            jgen_url: url,
            redis_url: redis_url.to_owned(),
            redis_client: RefCell::new(None),
            redis_connection: RefCell::new(None),
        }
    }

    fn get_connection(&self) -> Result<()> {
        if self.redis_connection.borrow().is_none() {
            let client = redis::Client::open(self.redis_url.clone())?;
            let conn = client.get_connection()?;
            self.redis_client.replace(Some(client));
            self.redis_connection.replace(Some(conn));
        }
        Ok(())
    }

    fn set(&self, key: &str, value: &str) -> Result<()> {
        self.get_connection()?;

        if let Some(conn) = self.redis_connection.borrow_mut().as_mut() {
            conn.set(key, value)?;
            conn.expire(key, 24 * 60 * 60)?;
        }

        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<String>> {
        self.get_connection()?;

        if let Some(conn) = self.redis_connection.borrow_mut().as_mut() {
            let value: String = conn.get(key)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    async fn get_selected_pem(&self, offset: i64) -> Result<PeriodicalPem> {
        let d = date::get_date(offset);
        let mut periodical: Option<PeriodicalPem> = None;
        if let Ok(res) = self.get(d.as_str()) {
            if let Some(json_str) = res {
                if let Ok(pem) = serde_json::from_str(json_str.as_str()) {
                    periodical = Some(PeriodicalPem {
                        pem,
                        timestamp: d.clone(),
                    });
                }
            }
        }
        if periodical.is_none() {
            let response = reqwest::get(self.jgen_url.clone())
                .await?
                .json::<B64Pem>()
                .await?;
            let pem = Pem::try_from(response)?;
            let cache_str = serde_json::to_string(&pem)?;
            self.set(d.as_str(), cache_str.as_str())?;

            periodical = Some(PeriodicalPem { pem, timestamp: d });
        }

        Ok(periodical.unwrap())
    }

    pub async fn get_current_pem(&self) -> Result<PeriodicalPem> {
        self.get_selected_pem(0).await
    }

    pub async fn get_all_pem(&self) -> Result<Vec<PeriodicalPem>> {
        Ok(vec![
            self.get_selected_pem(0).await?,
            self.get_selected_pem(1).await?,
        ])
    }
}
