//! Implementation of the EnvVars interface
//! Exposes all available Environment variables to the WASM process

use std::env as sysenv;
use std::collections::HashMap;
use std::str::FromStr;
use wasi_envvars::*;

pub use wasi_envvars::add_to_linker;

wit_bindgen_wasmtime::export!("wit/ephemeral/wasi-envvars.wit");

pub struct EnvVars {
    env: HashMap<String, String>
}

impl EnvVars {
    /// Builds EnvVars with ALL process environment variables exposed
    /// 
    /// All environment keys are internally normalized and are therefore
    /// case insensitive
    pub fn new_from_env() -> EnvVars {
        EnvVars::from_map(sysenv::vars().collect(), |_| true)
    }

    /// Builds EnvVars from process environment variables filtered through the given predicate function
    /// 
    /// The predicate function is used AFTER all keys are normalized and lowercased.
    pub fn new_with_predicate(filter: fn(&(String, String)) -> bool) -> EnvVars {
        EnvVars::from_map(sysenv::vars().collect(), filter)
    }

    /// Builds EnvVars from the given HashMap for the utmost control
    pub fn new_from(other: HashMap<String, String>) -> EnvVars {
        EnvVars::from_map(other, |_| true)
    }

    pub fn new_empty() -> EnvVars {
        EnvVars { env: HashMap::new() }
    }

    fn from_map(other: HashMap<String, String>, filter: fn(&(String, String)) -> bool) -> EnvVars {
        let the_env = other
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.to_owned()))
            .filter(filter)
            .collect();
        EnvVars {env: the_env}
    }

    /// Utility function to modify the environment after creation. Chain it for maximum damage!
    pub fn with(&mut self, key: &str, value: &str) -> &mut Self {
        self.env.insert(key.to_lowercase(), value.to_string());
        self
    }

    fn try_get<T>(&mut self, key: &str) -> Result<T, EnvError>
    where T: FromStr {
        match self.env.get(&key.to_lowercase()) {
            Some(value) => match value.parse::<T>() {
                Ok(parse_value) => Ok(parse_value),
                Err(_) => Err(EnvError::ConversionError)
            },
            None => Err(EnvError::KeyNotFound)
        }
    }
}

impl wasi_envvars::WasiEnvvars for EnvVars {
    fn get_keys(&mut self) -> Vec<String> {
        self.env.keys().map(|k| k.to_string()).collect()
    }

    fn has_key(&mut self, key: &str) -> bool {
        self.env.keys().any(|env_key| env_key.to_lowercase() == key.to_lowercase())
    }
 
    fn get(&mut self, key: &str) -> Result<String, EnvError> {
        self.try_get(key)
    }

    fn get_u8(&mut self, key: &str) -> Result<u8, EnvError> {
        self.try_get(key)
    }

    fn get_u16(&mut self, key: &str) -> Result<u16, EnvError> {
        self.try_get(key)
    }

    fn get_u32(&mut self, key: &str) -> Result<u32, EnvError> {
        self.try_get(key)
    }

    fn get_u64(&mut self, key: &str) -> Result<u64, EnvError> {
        self.try_get(key)
    }

    fn get_s8(&mut self, key: &str) -> Result<i8, EnvError> {
        self.try_get(key)
    }

    fn get_s16(&mut self, key: &str) -> Result<i16, EnvError> {
        self.try_get(key)
    }

    fn get_s32(&mut self, key: &str) -> Result<i32, EnvError> {
        self.try_get(key)
    }

    fn get_s64(&mut self, key: &str) -> Result<i64, EnvError> {
        self.try_get(key)
    }

    fn get_f32(&mut self, key: &str) -> Result<f32, EnvError> {
        self.try_get(key)
    }

    fn get_f64(&mut self, key: &str) -> Result<f64, EnvError> {
        self.try_get(key)
    }
}