//! Impement the WASI cache interface using a TiKV client.
//! This is using a Wasmtime host implementation.

use std::sync::Arc;
use tokio::runtime::Runtime;
use wasi_cache::*;

pub use wasi_cache::add_to_linker;

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-cache.wit");

#[derive(Clone)]
pub struct TikvClient {
    /// pd-endpoint for the TiKV client.
    pub pd_endpoint: String,

    /// the Tikv rust raw client (no transaction)
    inner: Arc<tikv_client::RawClient>,

    runtime: Arc<tokio::runtime::Runtime>,
}

impl wasi_cache::WasiCache for TikvClient {
    /// Set the payload for the given key.
    fn set(&mut self, key: &str, value: PayloadParam<'_>, _ttl: Option<u32>) -> Result<(), Error> {
        log::info!("setting key {}", key);
        self.set(key, value)?;
        Ok(())
    }

    /// Get the payload for the given key.
    fn get(&mut self, key: &str) -> Result<PayloadResult, Error> {
        log::info!("getting key {}", key);
        let value = self.get(key)?;
        match value {
            Some(value) => Ok(value),
            None => Ok(vec![]),
        }
    }

    /// Delete the entry for the given key.
    fn delete(&mut self, key: &str) -> Result<(), Error> {
        log::info!("deleting key {}", key);
        Ok(self.delete(key)?)
    }
}

impl TikvClient {
    pub fn new(pd_endpoint: &str) -> anyhow::Result<Self> {
        let mut rt = Runtime::new().unwrap();
        let client: tikv_client::RawClient =
            rt.block_on(tikv_client::RawClient::new(vec![pd_endpoint], None))?;
        Ok(Self {
            pd_endpoint: pd_endpoint.to_string(),
            inner: Arc::new(client),
            runtime: Arc::new(rt),
        })
    }

    pub fn set(&mut self, key: &str, value: &[u8]) -> anyhow::Result<()> {
        Ok(self
            .runtime
            .block_on(self.inner.put(key.to_string(), value))?)
    }

    pub fn get(&mut self, key: &str) -> anyhow::Result<Option<Vec<u8>>> {
        let value = self.runtime.block_on(self.inner.get(key.to_string()))?;
        Ok(value)
    }

    pub fn delete(&mut self, key: &str) -> anyhow::Result<()> {
        Ok(self.runtime.block_on(self.inner.delete(key.to_string()))?)
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}
