wit_bindgen_rust::import!("../../../wit/ephemeral/wasi-envvars.wit");
wit_bindgen_rust::export!("../../test.wit");

struct Test {}

impl test::Test for Test {
    fn test() -> Result<(), test::Error> {
        let definitely_doesnt_exist_key = "frododorf";
        let definitely_exists_key = "good_key";
        let convertable_key = "TESTCONVERT";
        let convertable_value: u8 = 1234;

        println!("envvars_rust_test:: Counting keys");
        let keys = wasi_envvars::get_keys();
        assert_neq!(0, keys.size());

        println!("envvars_rust_test:: Testing for key existence {}", definitely_exists_key);
        let res = wasi_envvars::has_key(definitely_exists_key);
        assert_eq!(true, res);

        println!("envvars_rust_test:: Reading nonexistent key {}", definitely_doesnt_exist_key);
        let res = wasi_envvars::get(definitely_doesnt_exist_key);
        match res {
            Err(wasi_envvars::EnvError::KeyNotFound) => assert!(true),
            _ => assert!(false)
        }

        println!("envvars_rust_test:: Testing successful conversion for {}", convertable_key);
        let res = wasi_envvars::get_u8(convertable_key);
        assert_eq!(true, res.is_ok());
        let res = res.unwrap();
        assert_eq!(convertable_value, res);

        let insensitive_key = convertable_key.to_lowercase();
        println!("envvars_rust_test:: Testing for key insensitivity {}", insensitive_key);
        assert_eq!(true, wasi_envvars::has_key(insensitive_key));

        println!("envvars_rust_test:: Testing failed conversion");
        let res = wasi_envvars::get_u8(definitely_exists_key);
        match res {
            Err(wasi_envvars::EnvError::ConversionError) => assert!(true),
            _ => assert!(false)
        }
    }
}