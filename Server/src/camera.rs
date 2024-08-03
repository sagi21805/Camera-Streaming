use opencv::core::*;
use opencv::prelude::*;
use opencv::videoio::{
    VideoCapture, VideoWriter, CAP_ANY, CAP_PROP_AUTO_EXPOSURE, CAP_PROP_BRIGHTNESS,
    CAP_PROP_CONTRAST, CAP_PROP_EXPOSURE, CAP_PROP_FOURCC, CAP_PROP_FRAME_HEIGHT,
    CAP_PROP_FRAME_WIDTH, CAP_PROP_GAIN, CAP_PROP_SATURATION,
};
pub struct CamSettings {
    pub auto_exposure: bool,
    pub exposure: i32,
    pub brightness: i32,
    pub contrast: i32,
    pub saturation: i32,
    pub gain: i32,
    pub frame_width: i32,
    pub frame_height: i32,
}

impl CamSettings {
    pub fn initialize_cap(&self, index: i32) -> Result<VideoCapture, &str> {
        let mut cap = VideoCapture::new(index, CAP_ANY).unwrap();

        if !VideoCapture::is_opened(&cap).unwrap() {
            return Err("Couldn't open cam");
        }

        let fourcc = VideoWriter::fourcc('M', 'J', 'P', 'G').unwrap() as f64;
        cap.set(CAP_PROP_FOURCC, fourcc).unwrap();
        cap.set(
            CAP_PROP_AUTO_EXPOSURE,
            (3 - (!self.auto_exposure as i32 * 2)) as f64,
        )
        .unwrap();
        cap.set(CAP_PROP_EXPOSURE, self.exposure as f64).unwrap();
        cap.set(CAP_PROP_BRIGHTNESS, self.brightness as f64)
            .unwrap();
        cap.set(CAP_PROP_CONTRAST, self.contrast as f64).unwrap();
        cap.set(CAP_PROP_SATURATION, self.saturation as f64)
            .unwrap();
        cap.set(CAP_PROP_GAIN, self.gain as f64).unwrap();
        cap.set(CAP_PROP_FRAME_WIDTH, self.frame_width as f64)
            .unwrap();
        cap.set(CAP_PROP_FRAME_HEIGHT, self.frame_height as f64)
            .unwrap();

        Ok(cap)
    }
}

impl Default for CamSettings {
    fn default() -> Self {
        Self {
            auto_exposure: true,
            exposure: 157,
            brightness: 0,
            contrast: 32,
            saturation: 90,
            gain: 0,
            frame_width: 1280,
            frame_height: 800,
        }
    }
}


pub struct Camera<'a> {
    index: i32,
    cap: VideoCapture,
    settings: &'a CamSettings,
    pub frame: Mat,
}

impl<'a> Camera<'a> {
    pub fn new(index: i32, settings: &'a CamSettings) -> Self {
        let mut camera = Camera {
            index,
            cap: settings.initialize_cap(index).unwrap(),
            settings,
            frame: Mat::default()
        };

        camera.warm_up();

        return camera;
    }

    pub fn update_frame(&mut self) {
        self.cap.read(&mut self.frame).expect("Couldn't read frame");
    }

    fn warm_up(&mut self) {
        for _ in 0..30 {
            self.update_frame();
        }
    }
}