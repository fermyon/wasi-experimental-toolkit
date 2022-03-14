
use wasi_ce::*;
wit_bindgen_rust::export!("../../wit/ephemeral/wasi-ce.wit");

struct WasiCe {}

impl wasi_ce::WasiCe for WasiCe {
    fn ce_handler(event: Cloudevent) -> Result<Cloudevent,Error> {
        println!("cloudevent source: {}", event.source);
        Ok(event)
    }

}

// TODO
// Error handling is currently not implemented.
impl From<anyhow::Error> for Error {
    fn from(_: anyhow::Error) -> Self {
        Self::Error
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::Error
    }
}
