use crate::Rect;
use bytes::Bytes;
use std::{collections::HashMap, fmt, time::Duration};

use super::{
    model::{layer::Layer, Font, FontCharacter, Marker},
    LottieImageAsset,
};

#[derive(Default, Clone)]
pub struct CompositionParameters;

#[derive(Default, Clone)]
pub struct LottieComposition {
    name: Option<String>,
    performance_tracker: Option<String>, // Some trash
    /// This is stored as a set to avoid duplicates.
    warnings: Vec<String>,
    /// Map of font names to fonts
    parameters: CompositionParameters,
    /// Used to determine if an animation can be drawn with hardware acceleration.
    has_dash_pattern: bool, // false
    /// Counts the number of mattes and masks. Before Android switched to SKIA
    /// for drawing in Oreo (API 28), using hardware acceleration with mattes and masks
    /// was only faster until you had ~4 masks after which it would actually become slower.
    mask_and_matte_count: u32,
}

impl LottieComposition {
    fn from_byte_data(data: &Bytes) -> Self {
        unimplemented!()
    }

    fn from_bytes(data: &Bytes) -> Self {
        unimplemented!()
    }

    fn add_warning(&self, warning: &str) {
        unimplemented!()
    }

    fn increment_matte_or_mask_count(&self, amount: u32) {
        unimplemented!()
    }

    fn get_mask_and_matte_count(&self) -> u32 {
        unimplemented!()
    }

    fn get_warnings(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get_performance_tracking_enabled(&self) -> bool {
        unimplemented!()
    }

    fn set_performance_tracking_enabled(enabled: bool) {
        unimplemented!()
    }

    fn get_performance_tracker(&self) -> Option<String> {
        unimplemented!()
    }

    fn layer_model_for_id(&self, id: u32) {
        unimplemented!()
    }

    fn get_bounds(&self) -> Rect<i32> {
        unimplemented!()
    }

    fn get_duration(&self) -> Duration {
        unimplemented!()
    }

    fn get_seconds(&self) -> f64 {
        unimplemented!()
    }

    fn get_start_frame(&self) -> f64 {
        unimplemented!()
    }

    fn get_end_frame(&self) -> f64 {
        unimplemented!()
    }

    fn get_layers(&self) -> Vec<Layer> {
        unimplemented!()
    }

    fn get_precomps(id: String) -> Vec<Layer> {
        unimplemented!()
    }

    fn get_characters(&self) -> HashMap<String, FontCharacter> {
        unimplemented!()
    }

    fn get_fonts(&self) -> HashMap<String, Font> {
        unimplemented!()
    }

    fn get_marker(&self, marker_name: &str) -> Option<Marker> {
        unimplemented!()
    }

    fn has_images(&self) {
        unimplemented!()
    }

    fn images(&self) -> HashMap<String, LottieImageAsset> {
        unimplemented!()
    }

    /// Returns a "rounded" progress value according to the frameRate
    fn round_progress(&self, progress: f64) {
        unimplemented!()
    }
}

impl fmt::Display for LottieComposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "LottieComposition")
    }
}
