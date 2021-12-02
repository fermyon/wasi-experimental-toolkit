//! Implements the low level access to Azure storage.

use azure_core::HttpClient;
use azure_storage::{blob::prelude::*, core::prelude::*};
use bytes::Bytes;
use std::{error::Error, result, sync::Arc};

/// The storage account name;
pub const STORAGE_ACCOUNT: &str = "STORAGE_ACCOUNT";
/// The main storage account access key.
pub const STORAGE_MASTER_KEY: &str = "STORAGE_MASTER_KEY";
/// The container name;
pub const CONTAINER: &str = "CONTAINER";

/// Type alias for the result used throughout this crate.
pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync>>;

/// Write a byte array into a blob, given the storage account, access, and container name.
pub async fn write_blob(
    blob: String,
    bytes: Vec<u8>,
    config: &Config,
    http_client: Arc<Box<dyn HttpClient>>,
) -> Result<u8> {
    let blob_client = StorageAccountClient::new_access_key(
        http_client.clone(),
        &config.storage_account,
        &config.storage_account_key,
    )
    .as_storage_client()
    .as_container_client(&config.container)
    .as_blob_client(blob);

    let len = bytes.len() as u8;
    println!("Writing {} bytes.", len);

    blob_client
        .put_block_blob(bytes)
        .content_type("text/plain")
        .execute()
        .await?;

    Ok(len)
}

/// Read a byte array from a blob, given the the storage account, access, container, and blob name.
pub async fn read_blob(
    blob: String,
    config: &Config,
    http_client: Arc<Box<dyn HttpClient>>,
) -> Result<Bytes> {
    let blob_client = StorageAccountClient::new_access_key(
        http_client.clone(),
        &config.storage_account,
        &config.storage_account_key,
    )
    .as_storage_client()
    .as_container_client(&config.container)
    .as_blob_client(blob);

    Ok(Bytes::from(
        blob_client.get().execute().await?.data.to_vec(),
    ))
}

/// Configuration for accessing a storage account and container.
pub struct Config {
    /// The storage account name.
    pub storage_account: String,
    /// The main storage account access key.
    pub storage_account_key: String,
    /// The container name;
    pub container: String,
}

/// Get the configuration for accessing the object storage from environment variables.
pub fn config_from_env() -> Result<Config> {
    Ok(Config {
        storage_account: std::env::var(STORAGE_ACCOUNT)?,
        storage_account_key: std::env::var(STORAGE_MASTER_KEY)?,
        container: std::env::var(CONTAINER)?,
    })
}
