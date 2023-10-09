use serde::{Deserialize,de::Error};
use std::fs;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    camera: CameraConfig,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    index: i32,
}

impl Config {
    
    pub fn from_file(path: &Path) -> Result<Config, dyn Error> {

        match fs::read_to_string(&path) {
            Ok(config) => serde_yaml::from_str(&config),
            Err(e) => Err(e),
        }

    }
}