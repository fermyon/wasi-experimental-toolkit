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
        println!("ce_test:: should fail with incomplete cloudevent");
        assert!(res.is_err());

        ce.set_source("https://example.com/event-producer");
        ce.set_type("com.microsoft.steelthread.wasm");
        ce.set_specversion("1.0");
        let res = wasi_ce::ce_handler(&ce);
        assert!(res.is_ok());
        println!("ce_test:: succeed with id: {}", res?.get_id());

        Ok(())
    }
}

impl From<wasi_ce::Error> for test::Error {
    fn from(_: wasi_ce::Error) -> Self {
        Self::Failure
    }
}
