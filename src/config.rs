use serde::Deserialize;
use std::{fs,path::Path};


#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    camera: CameraConfig,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    index: Option<i32>,
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

        match serde_yaml::from_str(&data) {
            Ok(config) => config,
            Err(error) => panic!(
                "There was an issue attempting to parse the config from {}: {}",
                path.to_str().unwrap(),
                error
            ),
        }
        
    }

    pub fn get_camera_conf(&self) -> &CameraConfig { &self.camera }
    
}

impl CameraConfig {

    pub fn index(&self) -> i32 { self._get_default(self.index, 0) }

    fn _get_default<T>(&self, element: Option<T>, default: T) -> T {
        match element {
            Some(x) => x,
            None => default,
        }
    }

}