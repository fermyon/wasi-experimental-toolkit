wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-ce.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let event = wasi_ce::CloudeventParam {
            id: "aaa",
            source: "/hello",
            specversion: "1.0",
            type_: "PUT",
            data: Some(b"hello world"),
        };

        println!("ce_test:: let wasm module handle the event id: {}", event.id);
        let res: Result<wasi_ce::CloudeventResult, _> = wasi_ce::ce_handler(event);
        println!("ce_test:: succeed, returned event id: {}", res.unwrap().id);
        Ok(())
    }
}

impl From<wasi_ce::Error> for test::Error {
    fn from(_: wasi_ce::Error) -> Self {
        Self::Failure
    }
}