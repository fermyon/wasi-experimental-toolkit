//! Implement the mongodb interface using the mongo-rust-driver (https://docs.rs/mongodb/latest/mongodb)
//! This is using a Wasmtime host implementation.

pub use wasi_mongo::add_to_linker;

use wasi_mongo::*;
use std::sync::Arc;

use mongodb::{
    sync::{Client,Database,Collection,Cursor},
    bson::doc,
    results::{InsertOneResult, InsertManyResult},
    options::FindOptions
};

use serde_json::{
    Value,
    Map,
    from_slice,
};

use serde::{Deserialize, Serialize};

const MONGO_DOCUMENT_ID_KEY: &str = "_id";

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-mongo.wit");

#[derive(Clone)]
pub struct WasiMongoClient {
    client: Arc<Client>
}

impl wasi_mongo::WasiMongo for WasiMongoClient {
    fn insert_one(&mut self, db: &str, collection: &str, document: DocumentParam) -> Result<(), Error> {
        if let Ok(res) = self.mongo_insert_one(db, collection, document) {
            Ok(())
        } else {
            Err(Error::Error)
        }
    }

    fn insert_many(&mut self,db: &str,collection: &str, documents: Vec<DocumentParam>) -> Result<(),Error> {
        if let Ok(_res) = self.mongo_insert_many(db, collection, documents) {
            Ok(())
        } else {
            Err(Error::Error)
        }   
    }

    fn find(&mut self, db: &str, collection: &str, filter: DocumentParam) -> Result<Vec<DocumentResult>,Error> {
        let res: Vec<DocumentResult> = self.mongo_find(db, collection, filter)?;
        Ok(res)
    }


    fn update_one(&mut self,db: &str,collection: &str,filter: DocumentParam,doc:DocumentParam) -> Result<(),Error> {
        todo!()
    }


    fn update_many(&mut self,db: &str,collection: &str,filter: DocumentParam,docs:Vec<DocumentParam>) -> Result<(),Error> {
        todo!()
    }

    fn replace_one(&mut self,db: &str,collection: &str,filter: DocumentParam,doc:DocumentParam) -> Result<(),Error> {
        todo!()
    }

    fn delete_one(&mut self,db: &str,collection: &str,filter: DocumentParam) -> Result<(),Error> {
        todo!()
    }

    fn delete_many(&mut self,db: &str,collection: &str,filter: DocumentParam) -> Result<(),Error> {
        todo!()
    }

    fn drop_database(&mut self, db: &str) -> Result<(), Error> {
        self.mongo_drop_database(db)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DB_NAME: &str = "db";
    const TEST_COLLECTION_NAME: &str = "collection";
    const MONGO_SERVER_ADDR: &str = "mongodb://localhost:27017";

    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u8,
        city: String,
    }

    #[test]
    fn insert_one() {
        let mut client = WasiMongoClient::new(MONGO_SERVER_ADDR).unwrap();

        let person = Person {
            name: "John".to_string(),
            age: 30,
            city: "London".to_string(),
        };
        
        let doc = serde_json::to_vec(&person).unwrap();

        let res = client.mongo_insert_one(TEST_DB_NAME, TEST_COLLECTION_NAME, &doc);
        assert!(res.is_ok());
    }

    #[test]
    fn insert_many() {
        let mut client = WasiMongoClient::new(MONGO_SERVER_ADDR).unwrap();

        let person_one = Person {
            name: "John".to_string(),
            age: 30,
            city: "London".to_string(),
        };

        let person_two = Person {
    		name: "Marcus".to_string(),
    		age: 27,
    		city: "London".to_string(),
        };

        
        let doc_one = serde_json::to_vec(&person_one).unwrap();
        let doc_two: Vec<u8> = serde_json::to_vec(&person_two).unwrap();

        let docs: Vec<&[u8]> = vec![&doc_one, &doc_two];

        let res = client.mongo_insert_many(TEST_DB_NAME, TEST_COLLECTION_NAME,docs);
        assert!(res.is_ok());
    }

    #[test]
    fn find_all() {
        let mut client = WasiMongoClient::new(MONGO_SERVER_ADDR).unwrap();
        let res = client.mongo_find(TEST_DB_NAME, TEST_COLLECTION_NAME, DocumentParam::default());
        
        assert!(res.is_ok());
        
        for doc in res.unwrap() {
            let person: Person = serde_json::from_slice(&doc).unwrap();
            println!("{:?}", person);
        }
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

    fn mongo_insert_one(&mut self, db: &str, collection: &str, document: DocumentParam) -> anyhow::Result<InsertOneResult> {
        let database = self.database(db)?;
        let collection: Collection<Map<String, Value>> = database.collection(collection);
        
        let parsed: Value = from_slice(document)?;
        let obj = parsed.as_object().unwrap().clone();
        
        Ok(collection.insert_one(obj, None)?)
    }


    fn mongo_insert_many(&mut self, db: &str, collection: &str, documents: Vec<DocumentParam>) -> anyhow::Result<InsertManyResult> {
        let database = self.database(db)?;
        let collection: Collection<Map<String, Value>> = database.collection(collection);
        let mut objs: Vec<Map<String, Value>> = vec![];
        for doc in documents {
            let parsed: Value = from_slice(doc)?;
            let obj = parsed.as_object().unwrap().clone();
            objs.push(obj);
        }

        Ok(collection.insert_many(objs, None)?)
    }

    // TODO: add filter
    fn mongo_find(&mut self, db: &str, collection: &str, _filter: DocumentParam) -> anyhow::Result<Vec<wasi_mongo::DocumentResult>> {
        let database = self.database(db)?;
        let collection: Collection<Map<String, Value>> = database.collection(collection);

        // let parsed_filter: Value = from_slice(filter)?;
        // let filter_map = parsed_filter.as_object().unwrap().clone();
        // let options: FindOptions = Default::default();

        let cursor = collection.find(None, None)?;
        
        let mut docs: Vec<wasi_mongo::DocumentResult> = vec![];

        for doc in cursor {
            let doc: wasi_mongo::DocumentResult  = serde_json::to_string(&doc.unwrap())?.into();
            docs.push(doc);
        }

        Ok(docs)
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