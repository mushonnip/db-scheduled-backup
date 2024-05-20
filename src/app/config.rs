use serde_derive::Deserialize;
use toml;
use std::{fs, process::exit};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_name: String,
    pub db_username: String,
    pub db_container_name: String,
}

pub fn get_config() -> Config {
    let filename = "Config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Unable to load data from `{}` {}", filename, e);
            exit(1);
        }
    };

    config
}
