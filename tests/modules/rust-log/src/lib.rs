use wasi_log::*;

wit_bindgen_rust::import!("wit/ephemeral/wasi-log.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let msg: &str = "To err is human to rub it in is divine";

        wasi_log::error(msg);

        Ok(())
    }
}
