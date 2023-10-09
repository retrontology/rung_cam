use serde::{Deserialize,de::Error};
use std::fs;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    index: i32,
}

impl Config {
    
    pub fn from_file(path: &Path) -> Result<Config, Error> {

        match fs::read_to_string(&path) {
            Ok(config) => serde_yaml::from_str(&config),
            Err(error) => Error(error),
        }

    }
}