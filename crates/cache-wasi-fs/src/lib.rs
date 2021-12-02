//! Impement the WASI cache interface using the filesystem API.

#![deny(missing_docs)]

use cache::*;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
wit_bindgen_rust::export!("../../wit/ephemeral/cache.wit");

/// The WASI filesystem implementation for the cache interface.
struct Cache {}

impl cache::Cache for Cache {
    /// Set the payload for the given key.
    /// This implementation ignores the time-to-live argument, as it stores the
    /// payloads as files.
    fn set(key: String, payload: Payload, _: Option<u32>) -> Result<(), Error> {
        let mut file = File::create(path(&key)?)?;
        file.write_all(&payload)?;
        Ok(())
    }

    /// Get the payload stored in the cache for the given key.
    fn get(key: String) -> Result<Payload, Error> {
        let mut file = File::create(path(&key)?)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

/// Return the absolute path for the file corresponding to the given key.
fn path(name: &str) -> Result<PathBuf, anyhow::Error> {
    Ok(PathBuf::from("cache").join(name))
}

// TODO
// Error handling is currently not implemented.
impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Error
    }
}
