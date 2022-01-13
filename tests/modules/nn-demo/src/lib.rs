wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-nn.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let _ = wasi_nn::load(&[&[]], wasi_nn::GraphEncoding::Onnx, wasi_nn::ExecutionTarget::Cpu).unwrap();
        Ok(())
    }
}

impl From<wasi_nn::Error> for test::Error {
    fn from(_: wasi_nn::Error) -> Self {
        Self::Failure
    }
}
