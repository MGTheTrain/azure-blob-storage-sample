use colored::Colorize;
use log::info;
use std::env;

pub mod azure_connectors {
    pub mod azure_blob_handler;
}

pub fn set_env_var(env_var_name: &str) -> Option<String> {
    match env::var(env_var_name) {
        Ok(value) => Some(value),
        Err(_) => {
            info!("{} is not set.", env_var_name.blue());
            None
        }
    }
}
