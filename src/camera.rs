use opencv::{
    core::Vector,
    prelude::{VideoCaptureTraitConst,VideoCaptureTrait},
    videoio::{self, VideoCapture, VideoWriter},
};


use opencv::Result;
use crate::config::camera::CameraConfig;

pub struct Camera {
    index: i32,
    width: i32,
    height: i32,
    fps: i32,
    source: VideoCapture,
}

impl Camera {

    pub fn from_config(config: &CameraConfig) -> Result<Camera> {

        let params = &[
            videoio::CAP_PROP_FOURCC,
            VideoWriter::fourcc('M', 'J', 'P', 'G')?,
            videoio::CAP_PROP_CONVERT_RGB,
            0i32,
            videoio::CAP_PROP_BUFFERSIZE,
            4i32,
        ];

        let mut source = VideoCapture::new_with_params(
            config.index(),
            videoio::CAP_V4L2,
            &Vector::from_slice(params),
        )?;
        
        let width: i32 = match config.resolution_width() {
            Some(w) => {
                source.set(videoio::CAP_PROP_FRAME_WIDTH, w as f64)?;
                w
            },
            None => source.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32,
        };
        let height: i32 = match config.resolution_height() {
            Some(h) => {
                source.set(videoio::CAP_PROP_FRAME_HEIGHT, h as f64)?;
                h
            },
            None => source.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32,
        };
        let fps: i32 = match config.fps() {
            Some(f) => {
                source.set(videoio::CAP_PROP_FPS, f as f64)?;
                f
            },
            None => source.get(videoio::CAP_PROP_FPS)? as i32,
        };

        let cam = Camera {
            index: config.index(),
            source: source,
            width: width,
            height: height,
            fps: fps,
        };

        Ok(cam)

    }

    fn index(&self) -> i32 { self.index }
    fn width(&self) -> i32 { self.width }
    fn height(&self) -> i32 { self.height }
    fn fps(&self) -> i32 { self.fps }
    
}
