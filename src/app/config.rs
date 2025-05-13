use serde_derive::Deserialize;
use toml;
use std::{fs, process::exit};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub cron: Option<Cron>,
    pub storage: Storage,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub db_name: String,
    pub db_username: String,
    pub db_container_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Cron {
    pub expression: String,
}

#[derive(Debug, Deserialize)]
pub struct Storage {
    pub media: String, // ftp, s3
    pub ftp: Option<Ftp>,
    pub s3: Option<S3>,
}

#[derive(Debug, Deserialize)]
pub struct Ftp {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct S3 {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub path: String,
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
