wit_bindgen_rust::import!("wit/ephemeral/wasi-cache.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let key = "where_are_they_taking_the_hobbits_to";
        let value = "Isengard";

        println!("cache_rust_test:: writing value {} to key {}", value, key);
        wasi_cache::set(key, &value.as_bytes(), None)?;

        println!("cache_rust_test:: reading from key {}", key);
        let res = wasi_cache::get(key)?;
        assert_eq!(value.as_bytes(), res);

        println!(
            "cache_rust_test:: read value {}",
            std::str::from_utf8(&res).unwrap()
        );

        println!("cache_rust_test:: deleting key {}", key);
        wasi_cache::delete(key)?;

        println!("cache_rust_test:: reading from key {} after deletion", key);
        let res = wasi_cache::get(key)?;
        let empty: &[u8] = &[];
        assert_eq!(empty, res.as_slice());

        Ok(())
    }
}

impl From<wasi_cache::Error> for test::Error {
    fn from(_: wasi_cache::Error) -> Self {
        Self::Failure
    }
}
