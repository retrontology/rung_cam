use serde::Deserialize;
use crate::config::Default;

const DEFAULT_CAMERA_INDEX: i32 = 0;

#[derive(Deserialize, PartialEq, Debug)]
pub struct CameraConfig {
    index: Option<i32>,
    fps: Option<i32>,
    resolution_width: Option<i32>,
    resolution_height: Option<i32>,
}

impl Default for CameraConfig {}

impl CameraConfig {

    pub fn index(&self) -> i32 { self.get_default(self.index, DEFAULT_CAMERA_INDEX) }
    pub fn fps(&self) -> Option<i32> { self.fps }
    pub fn resolution_width(&self) -> Option<i32> { self.resolution_width }
    pub fn resolution_height(&self) -> Option<i32> { self.resolution_height }

    pub fn verify(&self){}

}