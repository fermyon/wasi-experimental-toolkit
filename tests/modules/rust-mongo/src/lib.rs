use wasi_mongo::*;

wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-mongo.wit");
wit_bindgen_rust::export!("../../test.wit");

const TEST_DB_NAME: &str = "db";
const TEST_COLLECTION_NAME: &str = "collection";
const TEST_DOCUMENT_ID: &str = "my_id";

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {


        wasi_mongo::create_collection(TEST_DB_NAME, TEST_COLLECTION_NAME)?;
        
        let doc: &[u8] = 
            r#"{ "name": "Nitish", "age": 31, "city": "San Diego" }"#.as_bytes();
        wasi_mongo::insert_one_document(TEST_DB_NAME, TEST_COLLECTION_NAME, TEST_DOCUMENT_ID, doc.into())?;
        
        Ok(())
    }
}

impl From<wasi_mongo::Error> for test::Error {
    fn from(_: wasi_mongo::Error) -> Self {
        Self::Failure
    }
}