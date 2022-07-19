use std::{env};


pub fn load_env_vars(key: &str) -> String {
    let value: String;

    match env::var(key) {
        Ok(val) => value = val,
        Err(e) => panic!("${} is not set ({})", key, e)
    }
    return value;
}