//! Implement the WASI logging interface using the rust log crate
//! This is using a Wasmtime host implementation.

pub use wasi_log::add_to_linker;

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-log.wit");

#[derive(Clone)]
pub struct WasiLogger {}

impl wasi_log::WasiLog for WasiLogger {
    fn log(&mut self, msg: &str, lvl: wasi_log::Level) {
        log::log!(lvl.into(), "{}", msg);
    }

    fn trace(&mut self,msg: &str) {
        log::trace!("{}", msg);
    }
    
    fn debug(&mut self,msg: &str) {
        log::debug!("{}", msg);
    }
    
    fn info(&mut self,msg: &str) {
        log::info!("{}", msg);
    }
    
    fn warn(&mut self,msg: &str) {
        log::warn!("{}", msg);
    }
    
    fn error(&mut self,msg: &str) {
        log::error!("{}", msg);
    }
    
    fn fatal(&mut self,msg: &str) {
        log::error!("{}", msg);
    }
}

impl From<wasi_log::Level> for log::Level {
    fn from(lvl: wasi_log::Level) -> Self {
        match lvl {
            wasi_log::Level::Trace => log::Level::Trace,
            wasi_log::Level::Debug => log::Level::Debug,
            wasi_log::Level::Info => log::Level::Info,
            wasi_log::Level::Warn => log::Level::Warn,
            wasi_log::Level::Error => log::Level::Error,
            wasi_log::Level::Fatal => log::Level::Error,
        }
    }
}
