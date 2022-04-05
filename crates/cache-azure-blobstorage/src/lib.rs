//! Impement the WASI cache interface using the Azure Blob Storage service.

#![deny(missing_docs)]

pub mod azure;

use azure::Config;
use azure_core::{HttpClient, WasiHttpClient};
use futures::executor::block_on;
use std::sync::Arc;
use wasi_cache::*;

// The path to the cache interface is relative because this crate is in its own workspace.
wit_bindgen_rust::export!("wit/ephemeral/wasi-cache.wit");

/// Azure implementation for the cache interface.
/// The implementation assumes the storage container, service account,
/// and service account key have been passed in as environment variables.
pub struct WasiCache {}

impl wasi_cache::WasiCache for WasiCache {
    /// Set the payload for a given key as an Azure blob.
    fn set(key: String, value: Payload, _: Option<u32>) -> Result<(), Error> {
        Ok(block_on(set(key, value))?)
    }

    /// Read the payload for a given key from an Azure blob.
    fn get(key: String) -> Result<Payload, Error> {
        Ok(block_on(get(key))?)
    }

    /// Remove the payload for a given key from an Azure blob.
    fn delete(key: String) -> Result<(), Error> {
        Ok(block_on(delete(key))?)
    }
}

async fn get(name: String) -> azure::Result<Vec<u8>> {
    let (cfg, client) = cfg_client();
    let bytes = azure::read_blob(name, &cfg, client).await?.to_vec();
    Ok(bytes)
}

async fn set(name: String, bytes: Vec<u8>) -> azure::Result<()> {
    let (cfg, client) = cfg_client();
    let _ = azure::write_blob(name, bytes, &cfg, client).await?;
    Ok(())
}

async fn delete(name: String) -> azure::Result<()> {
    let (cfg, client) = cfg_client();
    azure::delete_blob(name, &cfg, client).await?;
    Ok(())
}

fn cfg_client() -> (Config, Arc<Box<dyn HttpClient>>) {
    let cfg =
        azure::config_from_env().expect("cannot get storage configuration from the environment");
    (cfg, Arc::new(Box::new(WasiHttpClient {})))
}

impl From<Box<dyn std::error::Error + Send + Sync>> for wasi_cache::Error {
    fn from(_: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self::Error
    }
}
