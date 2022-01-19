//! Implement the mongodb interface using the mongo-rust-driver (https://docs.rs/mongodb/latest/mongodb)
//! This is using a Wasmtime host implementation.

pub use wasi_mongo::add_to_linker;

use wasi_mongo::*;
use std::sync::Arc;

use mongodb::sync::{Client,Database};

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-mongo.wit");

#[derive(Clone)]
pub struct WasiMongoClient {
    address: String,
    client: Arc<Client>
}

impl wasi_mongo::WasiMongo for WasiMongoClient {
    fn collection_exists(&mut self, db: &str, name: &str) -> Result<bool, Error>{
        if let Ok(()) = self.collection_exists(db, name) {
            return Ok(true)
        } 
        Err(Error::Error) 
    }
}

impl WasiMongoClient {
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        let client = Arc::new(Client::with_uri_str(addr)?);
        Ok(WasiMongoClient {
            address: addr.to_string(),
            client
        })
    }

    fn collection_exists(&mut self, db: &str, _name: &str) -> anyhow::Result<()> {
        if let Ok(database) = self.database(db) {
            if let Ok(_doc) = database.run_command(mongodb::bson::doc! {"ping": 1}, None) {
                return Ok(())
            } else {
               return Err(anyhow::anyhow!("Database {} does not exist", db)) 
            }
        }
        Err(anyhow::Error::msg("database does not exist"))
    }
    
    fn database(&mut self, name: &str) -> anyhow::Result<Database> {
        let db = self.client.database(name);
        Ok(db)
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}