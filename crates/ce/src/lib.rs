
use std::{string, cell::RefCell, borrow::{BorrowMut, Borrow}};

use wasi_ce::*;
wit_bindgen_rust::export!("../../wit/ephemeral/wasi-ce.wit");
use wit_bindgen_rust::*;

struct WasiCe {}


#[derive(Default, Clone)]
struct Cloudevent {
    id: RefCell<String>,
    source: RefCell<String>,
    specversion: RefCell<String>,
    type_: RefCell<String>,
    datacontenttype: RefCell<String>,
    dataschema: RefCell<String>,
    subject: RefCell<String>,
    time: RefCell<String>,
    data: RefCell<Vec<u8>>,
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

    fn set_source(&self, source: String,) {
        println!("set source from {} to {}", self.source.borrow(), source);
        let mut _source = self.source.borrow_mut();
        *_source = source;
    }

    fn get_source(&self,) -> String {
        let s = self.source.borrow();
        println!("get source: {}", s);
        s.to_string()
    }

    fn set_specversion(&self,specversion: String,) {
        println!("set specversion from {} to {}", self.specversion.borrow(), specversion);
        let mut _specversion = self.specversion.borrow_mut();
        *_specversion = specversion;
    }

    fn get_specversion(&self,) -> String {
        let s = self.specversion.borrow();
        println!("get specversion: {}", s);
        s.to_string()
    }

    fn set_type(&self,type_: String,) {
        println!("set type from {} to {}", self.type_.borrow(), type_);
        let mut _type_ = self.type_.borrow_mut();
        *_type_ = type_;
    }

    fn get_type(&self,) -> String {
        let s = self.type_.borrow();
        println!("get type: {}", s);
        s.to_string()
    }

    fn set_data(&self,data: Data,) {
        let mut _data = self.data.borrow_mut();
        *_data = data;
    }

    fn get_data(&self,) -> Data {
        let s = self.data.borrow();
        s.clone()
    }

    fn set_datacontenttype(&self,datacontenttype: String,) {
        println!("set datacontenttype from {} to {}", self.datacontenttype.borrow(), datacontenttype);
        let mut _datacontenttype = self.datacontenttype.borrow_mut();
        *_datacontenttype = datacontenttype;
    }

    fn get_datacontenttype(&self,) -> String {
        let s = self.datacontenttype.borrow();
        println!("get datacontenttype: {}", s);
        s.to_string()
    }

    fn set_dataschema(&self,dataschema: String,) {
        println!("set dataschema from {} to {}", self.dataschema.borrow(), dataschema);
        let mut _dataschema = self.dataschema.borrow_mut();
        *_dataschema = dataschema;
    }

    fn get_dataschema(&self,) -> String {
        let s = self.dataschema.borrow();
        println!("get dataschema: {}", s);
        s.to_string()
    }

    fn set_subject(&self,subject: String,) {
        println!("set subject from {} to {}", self.subject.borrow(), subject);
        let mut _subject = self.subject.borrow_mut();
        *_subject = subject;
    }

    fn get_subject(&self,) -> String {
        let s = self.subject.borrow();
        println!("get subject: {}", s);
        s.to_string()
    }

    fn set_time(&self,time: String,) {
        println!("set time from {} to {}", self.time.borrow(), time);
        let mut _time = self.time.borrow_mut();
        *_time = time;
    }

    fn get_time(&self,) -> String {
        let s = self.time.borrow();
        println!("get time: {}", s);
        s.to_string()
    }
}

impl wasi_ce::WasiCe for WasiCe {
    fn ce_handler(event: Handle<Cloudevent>) -> Result<Handle<Cloudevent>,Error> {
        println!("event id is: {}", event.id.borrow());
        /*  
        check event.id, event.source, event.specversion and event.type_ are not empty
        */

        // TODO: properly handle the error
        if event.id.borrow().trim().is_empty() {
            return Err(wasi_ce::Error::Error);
        }
        if event.source.borrow().trim().is_empty() {
            return Err(wasi_ce::Error::Error);
        }
        if event.specversion.borrow().trim().is_empty() {
            return Err(wasi_ce::Error::Error);
        }
        if event.type_.borrow().trim().is_empty() {
            return Err(wasi_ce::Error::Error);
        }



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
