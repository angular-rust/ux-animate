use primitives::Size;

use super::{FrameRate, LottieComposition, LottieDelegates};

/// A Lottie animation in the render tree.
///
/// The RenderLottie attempts to find a size for itself that fits in the given
/// constraints and preserves the composition's intrinsic aspect ratio.
pub struct RenderLottie {
    /// The lottie composition to display.
    composition: LottieComposition,
    delegates: LottieDelegates,
    enable_merge_paths: bool,
    progress: f64,
    frame_rate: FrameRate,
    /// If non-null, requires the composition to have this width.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    width: f64,
    /// If non-null, require the composition to have this height.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    height: f64,
    // /// How to inscribe the composition into the space allocated during layout.
    // fit: BoxFit,
    // /// How to align the composition within its bounds.
    // ///
    // /// If this is set to a text-direction-dependent value, `textDirection` must
    // /// not be null.
    // alignment: AlignmentGeometry,

    // drawable: LottieDrawable
}

impl RenderLottie {
    pub fn get_composition(&self) -> LottieComposition {
        unimplemented!()
    }

    pub fn set_composition(&self, composition: LottieComposition) {
        unimplemented!()
    }

    pub fn get_width(&self) -> f64 {
        unimplemented!()
    }

    /// If non-null, requires the composition to have this width.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    pub fn set_width(&self, value: f64) {
        unimplemented!()
    }

    pub fn get_height(&self) -> f64 {
        unimplemented!()
    }

    /// If non-null, require the composition to have this height.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    pub fn set_height(&self, value: f64) {
        unimplemented!()
    }

    // pub fn get_fit(&self) -> BoxFit {
    //     unimplemented!()
    // }

    // /// How to inscribe the composition into the space allocated during layout.
    // pub fn set_fit(&self, value: BoxFit) {
    //     unimplemented!()
    // }

    // pub fn get_alignment(&self) -> AlignmentGeometry {
    //     unimplemented!()
    // }

    // /// How to align the composition within its bounds.
    // ///
    // /// If this is set to a text-direction-dependent value, [textDirection] must
    // /// not be null.
    // pub fn set_alignment(&self, value: AlignmentGeometry) {
    //     unimplemented!()
    // }

    // /// Find a size for the render composition within the given constraints.
    // ///
    // ///  - The dimensions of the RenderLottie must fit within the constraints.
    // ///  - The aspect ratio of the RenderLottie matches the intrinsic aspect
    // ///    ratio of the Lottie animation.
    // ///  - The RenderLottie's dimension are maximal subject to being smaller than
    // ///    the intrinsic size of the composition.
    // pub fn size_for_constraints(constraints: BoxConstrants) -> Size {
    //     unimplemented!()
    // }

    //override
    pub fn compute_min_intrinsic_width(&self, height: f64) -> f64 {
        unimplemented!()
    }

    //override
    pub fn compute_max_intrinsic_width(&self, height: f64) -> f64 {
        unimplemented!()
    }

    //override
    pub fn compute_min_intrinsic_height(&self, width: f64) -> f64 {
        unimplemented!()
    }

    //override
    pub fn compute_max_intrinsic_height(&self, width: f64) -> f64 {
        unimplemented!()
    }

    // //override
    // pub fn hit_test_self(&self, position: Offset) -> f64 {
    //     unimplemented!()
    // }

    // //override
    // pub fn compute_dry_layout(&self, constraints: BoxConstraints) -> Size {
    //     unimplemented!()
    // }

    //override
    pub fn perform_layout(&self) {
        unimplemented!()
    }

    // //override
    // pub fn paint(&self, context: PaintingContext, offset: Offset) {
    //     unimplemented!()
    // }

    // //override
    // pub fn debug_fill_properties(&self, properties: DiagnosticPropertiesBuilder) {
    //     unimplemented!()
    // }
}
