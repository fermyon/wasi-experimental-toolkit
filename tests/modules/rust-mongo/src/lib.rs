use wasi_mongo::*;

wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-mongo.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let msg: &str = "To err is human to rub it in is divine";
        wasi_mongo::collection_exists("database", "collection")?;
        Ok(())
    }
}

impl From<wasi_mongo::Error> for test::Error {
    fn from(_: wasi_mongo::Error) -> Self {
        Self::Failure
    }
}