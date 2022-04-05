use std::{fs, path::Path};

use wasi_cache_defs::WIT_FILES;

fn main() {
    for (name, contents) in WIT_FILES.iter() {
        let target = Path::new("wit").join("ephemeral").join(name);
        println!("cargo:rerun-if-changed={}", target.to_str().unwrap());
        if !target.exists() {
            fs::write(target, contents).unwrap();
        }
    }
}
