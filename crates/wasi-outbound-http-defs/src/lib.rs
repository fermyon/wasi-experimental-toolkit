use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref WIT_FILES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("http-types.wit", include_str!("../wit/http-types.wit"));
        m.insert(
            "wasi-outbound-http.wit",
            include_str!("../wit/wasi-outbound-http.wit"),
        );
        m
    };
}
