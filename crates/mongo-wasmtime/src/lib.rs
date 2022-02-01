//! Implement the mongodb interface using the mongo-rust-driver (https://docs.rs/mongodb/latest/mongodb)
//! This is using a Wasmtime host implementation.

pub use wasi_mongo::add_to_linker;

use wasi_mongo::*;
use std::sync::Arc;

use mongodb::{
    sync::{Client,Database,Collection},
    bson::doc,
    results::InsertOneResult,
    options::CreateCollectionOptions
};

use serde_json::{
    Value,
    Map,
    from_slice,
};

const MONGO_DOCUMENT_ID_KEY: &str = "_id";

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-mongo.wit");

#[derive(Clone)]
pub struct WasiMongoClient {
    client: Arc<Client>
}

impl wasi_mongo::WasiMongo for WasiMongoClient {
    fn create_collection(&mut self, db: &str, name: &str) -> Result<(), Error> {
        self.mongo_create_collection(db, name)?;
        Ok(())
    }

    fn insert_one_document(&mut self, db: &str, collection: &str, id: &str, document: wasi_mongo::DocumentParam) -> Result<String, Error> {
        match self.mongo_insert_one_document(db, collection, id, document) {
            Ok(res) => {
                return Ok(res.inserted_id.as_str().ok_or(Error::Error)?.to_string());
            },
            Err(_) => Err(Error::Error),
        }
    }

    fn find_document(&mut self, db: &str, collection: &str, name: &str) -> Result<wasi_mongo::DocumentResult, Error> {
        let res = self.mongo_find_document(db, collection, name)?;
        Ok(res)
    }

    fn drop_database(&mut self, db: &str) -> Result<(), Error> {
        self.mongo_drop_database(db)?;
        Ok(())
    }
}

impl WasiMongoClient {
    pub fn new(addr: &str) -> anyhow::Result<Self> {
        let client = Arc::new(Client::with_uri_str(addr)?);
        Ok(WasiMongoClient {
            client
        })
    }

    fn database(&mut self, name: &str) -> anyhow::Result<Database> {
        let db = self.client.database(name);
        Ok(db)
    }

    fn mongo_create_collection(&mut self, db: &str, name: &str) -> anyhow::Result<()> {
        let database = self.database(db)?;
        database.create_collection(name, CreateCollectionOptions::default())?;
        Ok(())
    }

    fn mongo_insert_one_document(&mut self, db: &str, collection: &str, id: &str, document: wasi_mongo::DocumentParam) -> anyhow::Result<InsertOneResult> {
        let database = self.database(db)?;
        let collection: Collection<Map<String, Value>> = database.collection(collection);
        
        let parsed: Value = from_slice(document)?;
        let mut obj = parsed.as_object().unwrap().clone();
        
        // Add the id to the documents _id field
        obj.insert(MONGO_DOCUMENT_ID_KEY.to_string(), Value::String(id.to_string()));
        
        Ok(collection.insert_one(obj, None)?)
    }

    fn mongo_find_document(&mut self, db: &str, collection: &str, name: &str) -> anyhow::Result<wasi_mongo::DocumentResult> {
        let database = self.database(db)?;
        let collection: Collection<Map<String, Value>> = database.collection(collection);

        let obj: Map<String, Value> = collection.find_one(doc! {"_id": name}, None)?.unwrap();
        let res: wasi_mongo::DocumentResult = serde_json::to_string(&obj)?.into();
        Ok(res)
    }

    fn mongo_drop_database(&mut self, db: &str) -> anyhow::Result<()> {
        let database = self.database(db)?;
        database.drop(None)?;
        Ok(())
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}