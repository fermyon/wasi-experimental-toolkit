use wasi_mongo::*;
use serde::{Deserialize, Serialize};

wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-mongo.wit");
wit_bindgen_rust::export!("../../test.wit");

const TEST_DB_NAME: &str = "db";
const TEST_COLLECTION_NAME: &str = "collection";

struct Test {}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    city: String,
}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let person_one = Person {
            name: "John".to_string(),
            age: 30,
            city: "London".to_string(),
        };
        
        // Serialize it to a JSON string.
        let doc = serde_json::to_vec(&person_one).unwrap();

        println!("Inserting document into collection {} in database {}", TEST_COLLECTION_NAME, TEST_DB_NAME);
        wasi_mongo::insert_one(TEST_DB_NAME, TEST_COLLECTION_NAME, &doc)?;
        println!("Document inserted");

        let person_two = Person {
            name: "Marcus".to_string(),
            age: 35,
            city: "New York".to_string(),
        };

        let person_three = Person {
            name: "Nitish".to_string(),
            age: 31,
            city: "San Diego".to_string(),
        };

        println!("Inserting multiple documents into collection {} in database {}", TEST_COLLECTION_NAME, TEST_DB_NAME);
        let doc_two: Vec<u8> = serde_json::to_vec(&person_two).unwrap();
        let doc_three: Vec<u8> = serde_json::to_vec(&person_three).unwrap();
        let docs: Vec<&[u8]> = vec![&doc_two, &doc_three];

        wasi_mongo::insert_many(TEST_DB_NAME, TEST_COLLECTION_NAME, &docs)?;
        println!("Documents inserted");

        println!("Finding all documents in collection {} in database {}", TEST_COLLECTION_NAME, TEST_DB_NAME);
        let res = wasi_mongo::find(TEST_DB_NAME, TEST_COLLECTION_NAME, wasi_mongo::DocumentParam::default())?;
        
        for doc in res {
            let person: Person = serde_json::from_slice(&doc).unwrap();
            println!("Found document {:?}", person);
        }

        println!("Dropping database {}", TEST_DB_NAME);
        wasi_mongo::drop_database(TEST_DB_NAME)?;
        println!("Database dropped");
        
        Ok(())
    }
}

impl From<wasi_mongo::Error> for test::Error {
    fn from(_: wasi_mongo::Error) -> Self {
        Self::Failure
    }
}