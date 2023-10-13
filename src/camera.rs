use opencv::videoio::{self, VideoCapture};
use opencv::Result;
use crate::config::camera::CameraConfig;

pub struct Camera {
    index: i32,
    fourcc: i32,
    source: VideoCapture,
}

impl Camera {

    pub fn from_config(config: &CameraConfig) -> Result<Camera> {
        
        match VideoCapture::new(config.index(), videoio::CAP_V4L2) {
            Ok(source) => Ok(
                Camera {
                    index: config.index(),
                    fourcc: config.format(),
                    source: source,
                }
            ),
            Err(error) => Err(error),
        }

    }

    pub fn get_source(&self) -> &VideoCapture { &self.source }

}
