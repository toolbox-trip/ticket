use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::error::{Error, ErrorKind, Result};

pub struct MemcacheClient {
    client: TcpStream,
}

impl MemcacheClient {
    pub fn new() -> MemcacheClientBuilder {
        MemcacheClientBuilder {}
    }

    pub async fn get(&mut self, key: &str) -> Result<Vec::<u8>> {
        let msg = format!("get {}\r\n", key);
        self.client.write_all(msg.as_bytes()).await?;
        let mut buffer = Vec::<u8>::with_capacity(4096);
        loop {
            let mut inner_buffer = Vec::<u8>::with_capacity(256);
            let res = self.client.read(&mut inner_buffer).await?;
            if res > 0 {
                buffer.append(&mut inner_buffer);
            } else {
                break;
            }
        }
        let result = Self::process_response(&mut buffer)?;
        Ok(result)
    }

    fn process_response(buffer: &mut Vec<u8>) -> Result<Vec::<u8>> {
        let response_type = String::from_utf8(buffer[0..5].to_vec())?;
        if &response_type != "VALUE" || buffer.len() < 15 {
            return Err(Error::from_kind(ErrorKind::CannotGetCache(response_type)));
        }
        let position = buffer.iter().position(|ch| *ch == 10).unwrap();
        let message_buffer = &buffer[position + 1..buffer.len() - 7];

        Ok(message_buffer.to_vec())
    }
}

pub struct MemcacheClientBuilder {}

impl MemcacheClientBuilder {
    pub async fn connect(self, addr: &str) -> Result<MemcacheClient> {
        let client = TcpStream::connect(addr).await?;
        Ok(MemcacheClient { client })
    }
}
