// use crate::{Canvas, Pattern};
// use dx::Matrix; // not available in wasm
// use crate::Image; // not available in wasm
use primitives::prelude::CanvasContext;

use crate::foundation::{Rect, Size};

use super::{
    model::{layer::CompositionLayer, KeyPath},
    LottieComposition, LottieDelegates,
};

pub struct LottieFontStyle {
    font_family: String,
    style: String,
}

impl LottieFontStyle {
    pub fn new(font_family: &str, style: &str) -> Self {
        Self {
            font_family: font_family.into(),
            style: style.into(),
        }
    }
}

pub struct LottieDrawable {
    composition: LottieComposition,
    // matrix: Matrix,// not available in wasm
    composition_layer: CompositionLayer,
    size: Size<f64>,
    delegates: LottieDelegates,
    is_dirty: bool,
    enable_merge_paths: bool,
    /// Gives a suggestion whether to paint with anti-aliasing, or not.
    /// Default is true.
    anti_aliasing_suggested: bool,

    /// Sets whether to apply opacity to the each layer instead of shape.
    ///
    /// Opacity is normally applied directly to a shape. In cases where translucent
    /// shapes overlap, applying opacity to a layer will be more accurate at the
    /// expense of performance.
    ///
    /// The default value is false.
    ///
    /// Note: This process is very expensive. The performance impact will be reduced
    /// when hardware acceleration is enabled.
    is_applying_opacity_to_layers_enabled: bool,
}

impl LottieDrawable {
    pub fn get_progress(&self) -> f64 {
        unimplemented!()
    }

    pub fn set_progress(&self, value: f64) -> f64 {
        unimplemented!()
    }

    pub fn get_delegates(&self) -> LottieDelegates {
        unimplemented!()
    }

    pub fn set_delegates(&self, delegates: LottieDelegates) -> LottieDelegates {
        unimplemented!()
    }

    pub fn get_use_text_glyphs(&self) -> bool {
        unimplemented!()
    }

    // pub fn get_image_asset(&self, key: &str) -> Image {
    //     unimplemented!()
    // }

    // pub fn get_text_style(&self, font: &str, style: &str) -> TextStyle {
    //     unimplemented!()
    // }

    // fn update_value_delegates(&self, new_delegates: Vec<ValueDelegate>) {
    //     unimplemented!()
    // }

    /// Takes a {@link KeyPath}, potentially with wildcards or globstars and resolve it to a list of
    /// zero or more actual {@link KeyPath Keypaths} that exist in the current animation.
    /// <p>
    /// If you want to set value callbacks for any of these values, it is recommend to use the
    /// returned {@link KeyPath} objects because they will be internally resolved to their content
    /// and won't trigger a tree walk of the animation contents when applied.
    fn resolve_key_path(&self, key_path: KeyPath) -> Vec<KeyPath> {
        unimplemented!()
    }

    // )))
    // should add `fit` and `alignment`
    // pub fn draw(&self, canvas: Canvas, rect: Rect<f64>) {
    //     unimplemented!()
    // }
}
