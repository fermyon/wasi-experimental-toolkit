wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-ce.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

struct Cloudevent {
    id: String,
}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let ce = wasi_ce::Cloudevent::create();
        ce.set_id("aaa");
        let res = wasi_ce::ce_handler(&ce);
        let id = res?.get_id();
        println!("ce_test:: succeed, returned event id: {}", id);
        Ok(())
    }
}

impl From<wasi_ce::Error> for test::Error {
    fn from(_: wasi_ce::Error) -> Self {
        Self::Failure
    }
}