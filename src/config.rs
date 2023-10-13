use serde::Deserialize;
use std::{fs,path::Path};
use opencv::videoio::VideoWriter;

const DEFAULT_CAMERA_INDEX: i32 = 0;
const DEFAULT_CAMERA_FORMAT: &str = "MJPG";

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

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    index: Option<i32>,
    format: Option<String>,
}

impl CameraConfig {

    pub fn index(&self) -> i32 { self._get_default(self.index, DEFAULT_CAMERA_INDEX) }

    pub fn format(&self) -> i32 {
        let fourcc_string = self._get_default_string(&self.format, &DEFAULT_CAMERA_FORMAT);
        let fourcc_bytes = fourcc_string.as_bytes();
        let fourcc = VideoWriter::fourcc(
            fourcc_bytes[0] as char, 
            fourcc_bytes[1] as char, 
            fourcc_bytes[2] as char,
            fourcc_bytes[3] as char
        );
        match fourcc {
            Ok(f) => f,
            Err(e) => panic!("Could not parse format from config. Please refer to OpenCV FourCC codes for valid values: {}", e),
        }
    }

    

    fn _get_default<T: Copy>(&self, element: Option<T>, default: T) -> T {
        match element {
            Some(x) => x,
            None => default,
        }
    }

    fn _get_default_string(&self, element: &Option<String>, default: &str) -> String {
        match element {
            Some(x) => x.clone(),
            None => default.to_string(),
        }
    }
}
