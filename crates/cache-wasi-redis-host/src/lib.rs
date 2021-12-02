//! Impement the WASI cache interface using a Redis instance.
//! This is using a Wasmtime host implementation.

use cache::*;
use redis::{Client, Commands};

wit_bindgen_wasmtime::export!("wit/ephemeral/cache.wit");

/// Redis implementation for the WASI cache interface.
pub struct Cache {
    /// The address of the Redis instance.
    pub address: String,

    /// The Redis client.
    client: Client,
}

impl cache::Cache for Cache {
    /// Set the payload for the given key.
    /// If provided, the time-to-live argument (in seconds) will be used to set the expiration time.
    fn set(&mut self, key: &str, value: PayloadParam<'_>, ttl: Option<u32>) -> Result<(), Error> {
        self.set(key, value, ttl)?;
        Ok(())
    }

    /// Get the payload for the given key.
    fn get(&mut self, key: &str) -> Result<PayloadResult, Error> {
        Ok(self.get(key)?)
    }
}

impl Cache {
    /// Create a new instance for the cache.
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        let client = Client::open(addr)?;
        Ok(Cache {
            address: addr.to_string(),
            client,
        })
    }

    /// Set the payload in Redis using the given key and optional time-to-live (in seconds).
    pub fn set(&mut self, key: &str, value: &[u8], ttl: Option<u32>) -> anyhow::Result<()> {
        let mut conn = self.client.get_connection()?;
        conn.set(key, value)?;
        match ttl {
            Some(s) => conn.expire(key, s as usize)?,
            None => {}
        };

        Ok(())
    }

    /// Get the payload stored in Redis using the given key.
    pub fn get(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
        let mut conn = self.client.get_connection()?;
        let res: Vec<u8> = conn.get(key)?;

        Ok(res)
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}
