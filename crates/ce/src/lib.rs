
use std::{string, cell::RefCell, borrow::{BorrowMut, Borrow}};

use wasi_ce::*;
wit_bindgen_rust::export!("../../wit/ephemeral/wasi-ce.wit");
use wit_bindgen_rust::*;

struct WasiCe {}


#[derive(Default, Clone)]
struct Cloudevent {
    id: RefCell<String>,
}

impl wasi_ce::Cloudevent for Cloudevent {
    
    fn create() -> Handle<Cloudevent> {
        Cloudevent::default().into()
    }

    fn set_id(&self, id: String) { 
        println!("set id from {} to {}", self.id.borrow(), id);
        let mut _id = self.id.borrow_mut();
        *_id = id;
    }

    fn get_id(&self) -> String { 
        let s = self.id.borrow();
        println!("get id: {}", s);
        s.to_string()
    }
}

impl wasi_ce::WasiCe for WasiCe {
    fn ce_handler(event: Handle<Cloudevent>) -> Result<Handle<Cloudevent>,Error> {
        println!("event id is: {}", event.id.borrow());

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
