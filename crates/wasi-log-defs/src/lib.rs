use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref WIT_FILES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("wasi-log.wit", include_str!("../wit/wasi-log.wit"));
        m
    };
}
