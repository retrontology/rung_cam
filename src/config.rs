use serde::Deserialize;
use std::{fs,path::Path};


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

        let f = fs::read_to_string(&path);

        let data = match f {
            Ok(config) => config,
            Err(error) => panic!(
                "Could not open the config file {} for reading: {}",
                path.to_str().unwrap(),
                error
            ),
        };

        match serde_yaml::from_str(&data) {
            Ok(config) => config,
            Err(error) => panic!(
                "There was an issue attempting to parse the config from {}: {}",
                path.to_str().unwrap(),
                error
            ),
        }

    }
}