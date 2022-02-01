use wasi_mongo::*;
use serde::{Deserialize, Serialize};

wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-mongo.wit");
wit_bindgen_rust::export!("../../test.wit");

const TEST_DB_NAME: &str = "db";
const TEST_COLLECTION_NAME: &str = "collection";
const TEST_DOCUMENT_ID: &str = "my_id";

struct Test {}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    city: String,
}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        wasi_mongo::create_collection(TEST_DB_NAME, TEST_COLLECTION_NAME)?;
        
        let person = Person {
            name: "John".to_string(),
            age: 30,
            city: "London".to_string(),
        };
        
        // Serialize it to a JSON string.
        let doc = serde_json::to_vec(&person).unwrap();
        
        wasi_mongo::insert_one_document(TEST_DB_NAME, TEST_COLLECTION_NAME, TEST_DOCUMENT_ID, &doc)?;
        
        wasi_mongo::find_document(TEST_DB_NAME, TEST_COLLECTION_NAME, TEST_DOCUMENT_ID)
            .map(|res| {
                let out: Person = serde_json::from_slice::<Person>(&res).unwrap();
                println!("{:?}", out);
            })
            .map_err(|err| {
                println!("{:?}", err);
            });

        Ok(())
    }
}

impl From<wasi_mongo::Error> for test::Error {
    fn from(_: wasi_mongo::Error) -> Self {
        Self::Failure
    }
}