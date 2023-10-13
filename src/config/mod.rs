use serde::Deserialize;
use std::{fs,path::Path};
use crate::config::camera::CameraConfig;

pub mod camera;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    camera: CameraConfig,
}

impl Config {
    
    // Can't convert serde_yaml::Error to std::io::Error or vice versa so terminate errors here
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

        let config: Config = match serde_yaml::from_str(&data) {
            Ok(config) => config,
            Err(error) => panic!(
                "There was an issue attempting to parse the config from {}: {}",
                path.to_str().unwrap(),
                error
            ),
        };

        config.verify();

        config
        
    }

    pub fn get_camera_conf(&self) -> &CameraConfig { &self.camera }

    fn verify(&self) {
        self.camera.verify();
    }
    
}
