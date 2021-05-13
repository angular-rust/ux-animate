use super::{FrameRate, LottieComposition, LottieDelegates, LottieOptions, RenderLottie};

/// A widget that displays a `LottieDrawable` directly.
///
/// This widget is rarely used directly. Instead, consider
/// using `Lottie` or `LottieAnimation`.
pub struct RawLottie {
    key: String,
    /// The Lottie composition to display.
    composition: LottieComposition,
    /// Allows to modify the Lottie animation at runtime
    delegates: LottieDelegates,
    options: LottieOptions,
    /// The progress of the Lottie animation (between 0.0 and 1.0).
    progress: f64,
    /// The number of frames per second to render.
    /// Use `FrameRate.composition` to use the original frame rate of the Lottie composition (default)
    /// Use `FrameRate.max` to advance the animation progression at every frame.
    frame_rate: FrameRate,
    /// If non-null, require the Lottie composition to have this width.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    width: f64,
    /// If non-null, require the Lottie composition to have this height.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    height: f64,
    // /// How to inscribe the Lottie composition into the space allocated during layout.
    // fit: BoxFit
    // /// How to align the composition within its bounds.
    // ///
    // /// The alignment aligns the given position in the image to the given position
    // /// in the layout bounds. For example, an [Alignment] alignment of (-1.0,
    // /// -1.0) aligns the image to the top-left corner of its layout bounds, while a
    // /// [Alignment] alignment of (1.0, 1.0) aligns the bottom right of the
    // /// image with the bottom right corner of its layout bounds. Similarly, an
    // /// alignment of (0.0, 1.0) aligns the bottom middle of the image with the
    // /// middle of the bottom edge of its layout bounds.
    // ///
    // /// Defaults to [Alignment.center].
    // ///
    // /// See also:
    // ///
    // ///  * [Alignment], a class with convenient constants typically used to
    // ///    specify an [AlignmentGeometry].
    // ///  * [AlignmentDirectional], like [Alignment] for specifying alignments
    // ///    relative to text direction.
    // alignment: AlignmentGeometry,
}

impl RawLottie {
    // override
    // seems i need some abstraction like a BuildContext )))
    fn create_render_object() -> Self {
        unimplemented!()
    }

    // override
    fn update_render_object(render_object: RenderLottie) {
        unimplemented!()
    }

    // override
    // fn debug_fill_properties(properties: DiagnosticPropertiesBuilder) {

    // }
}
