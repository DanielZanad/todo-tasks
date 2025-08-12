extern crate dotenv;
use std::env;

use dotenv::dotenv;

pub fn get_env_var(var: &str) -> Option<String> {
    dotenv().ok();

    let env_var = env::var(var);
    if let Ok(result) = env_var {
        Some(result)
    } else {
        None
    }
}
