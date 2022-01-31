//! Implement the mongodb interface using the mongo-rust-driver (https://docs.rs/mongodb/latest/mongodb)
//! This is using a Wasmtime host implementation.

pub use wasi_mongo::add_to_linker;

use wasi_mongo::*;
use std::sync::Arc;

use mongodb::{
    sync::{Client,Database,Collection},
    options::CreateCollectionOptions, bson::{Bson, document::Values, oid::ObjectId}, results::InsertOneResult,
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
                println!("{:?}", res);
                return Ok(res.inserted_id.as_str().ok_or_else(|| Error::Error)?.to_string());
            },
            Err(_) => Err(Error::Error),
        }
    }

    fn get_document(&mut self, db: &str, collection: &str, name: &str) -> Result<wasi_mongo::DocumentResult, Error> {
        self.mongo_get_document(db, collection, name)?;
        Ok(wasi_mongo::DocumentResult::new())
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

    fn mongo_get_document(&mut self, db: &str, collection: &str, _name: &str) -> anyhow::Result<wasi_mongo::DocumentResult> {
        let database = self.database(db)?;
        let _collection: Collection<Map<String, Value>> = database.collection(collection);

        // let obj: Map<String, Value> = collection.find_one(doc! {"_id": name}, None)?.unwrap();

        Ok(wasi_mongo::DocumentResult::new())
    }
}

impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}