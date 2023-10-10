use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub camera: CameraConfig,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    pub index: i32,
}

impl Config {
    
    pub fn from_file(path: &Path) -> Config {

        match fs::read_to_string(&path) {
            Ok(data) => {
                match serde_yaml::from_str(&data) {
                    Ok(config) => config,
                    Err(error) => {
                        panic!(
                            "There was an issue attempting to parse the config from {}: {}",
                            path.to_str().unwrap(),
                            error
                        );
                    },
                }
            }
            Err(error) => {
                panic!(
                    "Could not open the config file {} for reading: {}",
                    path.to_str().unwrap(),
                    error
                );
            },
        }

    }
}