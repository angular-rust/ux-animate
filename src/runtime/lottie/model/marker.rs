use crate::runtime::lottie::LottieComposition;

pub struct Marker {
    composition: LottieComposition,
    name: String,
    start_frame: f64,
    duration_frames: f64,
}

impl Marker {
    pub fn new(
        composition: LottieComposition,
        name: String,
        start_frame: f64,
        duration_frames: f64,
    ) -> Self {
        Self {
            composition,
            name,
            start_frame,
            duration_frames,
        }
    }

    fn matches_name(&self, name: String) -> bool {
        self.name.to_lowercase() == name.to_lowercase()
    }

    fn get_start(&self) -> f64 {
        unimplemented!()
    }

    fn get_end(&self) -> f64 {
        unimplemented!()
    }
}
