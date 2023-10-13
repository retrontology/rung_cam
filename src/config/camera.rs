use serde::Deserialize;
use opencv::videoio::VideoWriter;

const DEFAULT_CAMERA_INDEX: i32 = 0;
const DEFAULT_CAMERA_FORMAT: &str = "MJPG";
const DEFAULT_CAMERA_MAX_FPS: f64 = 30.0;

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    index: Option<i32>,
    format: Option<String>,
    max_fps: Option<f64>,
}

impl CameraConfig {

    pub fn index(&self) -> i32 { self.get_default(self.index, DEFAULT_CAMERA_INDEX) }

    pub fn format(&self) -> i32 {
        let fourcc_string = self.get_default_string(&self.format, &DEFAULT_CAMERA_FORMAT);
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

    pub fn max_fps(&self) -> f64 { self. get_default(self.max_fps, DEFAULT_CAMERA_MAX_FPS) }

    pub fn verify(&self){
        // format
        self.format();
    }

    fn get_default<T: Copy>(&self, element: Option<T>, default: T) -> T {
        match element {
            Some(x) => x,
            None => default,
        }
    }

    fn get_default_string(&self, element: &Option<String>, default: &str) -> String {
        match element {
            Some(x) => x.clone(),
            None => default.to_string(),
        }
    }
}