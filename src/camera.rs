

mod camera {

    use opencv::videoio::{self, VideoCapture};
    pub struct Camera {
        index: i32,
        source: VideoCapture,
    }

    impl Camera {
        pub fn new(index: i32) -> Camera {
            Camera {
                index: index,
                source: match VideoCapture::new(index, videoio::CAP_V4L2) {
                    Some(device,) => device,
                    None => None,
                }
            }
        }
    }
}