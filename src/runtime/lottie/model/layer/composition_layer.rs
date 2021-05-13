// use dx::Matrix; // not available in wasm
use crate::{
    lottie::{
        animation::keyframe::BaseKeyframeAnimation, model::KeyPath, value::LottieValueCallback,
    },
    Canvas, Rect, Size,
};

use super::BaseLayer;

pub struct CompositionLayer {
    time_remapping: BaseKeyframeAnimation,
    layers: Vec<BaseLayer>,
    // layer_paint: Paint,
    has_mate: bool,
    has_masks: bool,
}

impl CompositionLayer {
    // override
    // parent_alpha ???
    // pub fn draw_layer(&self, canvas: Canvas, size: Size<f64>, parent_matrix: Matrix) {
    //     unimplemented!()
    // }

    // // override
    // fn get_bounds(&self, parent_matrix: Matrix, apply_parents: bool) -> Rect<f64> {
    //     unimplemented!()
    // }

    //override
    fn set_progress(progress: f64) {
        unimplemented!()
    }

    fn has_masks(&self) -> bool {
        unimplemented!()
    }

    fn has_matte(&self) -> bool {
        unimplemented!()
    }

    //override
    fn resolve_child_key_path(
        &self,
        key_path: KeyPath,
        depth: i32,
        accumulator: Vec<KeyPath>,
        current_partial_key_path: KeyPath,
    ) {
        unimplemented!()
    }

    // override
    fn add_value_callback<T>(property: T, callback: LottieValueCallback<T>) {
        unimplemented!()
    }
}
