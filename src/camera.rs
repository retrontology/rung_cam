use opencv::videoio::{self, VideoCapture};
use opencv::Result;

pub struct Camera {
    index: i32,
    source: VideoCapture,
}

impl Camera {

    pub fn new(index: i32) -> Result<Camera> {

        match VideoCapture::new(index, videoio::CAP_V4L2) {
            Ok(source) => Ok(Camera { index: index, source: source }),
            Err(error) => Err(error),
        }

    }

}
