#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::prelude::*;
use ruex::foundation::patterns::builder::Builder;
use bytes::Bytes;
use ruex::prelude::*;
use std::{cell::RefCell, fs::File, rc::Rc};

pub(crate) mod animation;
pub(crate) mod model;
pub(crate) mod parser;
pub(crate) mod providers;
pub(crate) mod utils;
pub(crate) mod value;

mod composition;
pub(crate) use composition::*;

mod frame_rate;
pub(crate) use frame_rate::*;

mod l;
pub(crate) use l::*;

mod logger;
pub(crate) use logger::*;

mod lottie_builder;
pub(crate) use lottie_builder::*;

mod lottie_delegates;
pub(crate) use lottie_delegates::*;

mod lottie_drawable;
pub(crate) use lottie_drawable::*;

mod lottie_image_asset;
pub(crate) use lottie_image_asset::*;

mod lottie_property;
pub(crate) use lottie_property::*;

mod options;
pub(crate) use options::*;

mod performance_tracker;
pub(crate) use performance_tracker::*;

mod raw_lottie;
pub(crate) use raw_lottie::*;

mod render_lottie;
pub(crate) use render_lottie::*;

mod value_delegate;
pub(crate) use value_delegate::*;

pub(crate) use self::providers::LottieImageProviderFactory;
use self::providers::LottieProvider;

// Animation, AssetBundle, BoxFit, AnimatedBuilder

/// A widget to display a loaded `LottieComposition`.
/// The `controller` property allows to specify a custom AnimationController that
/// will drive the animation. If `controller` is None, the animation will play
/// automatically and the behavior could be adjusted with the properties `animate`,
/// `repeat` and `reverse`.
#[derive(Default, Clone)]
pub struct Lottie {
    // RawLottie
    composition: LottieComposition,
    // controller: Option<Animation<f64>>,
    animate: bool,
    frame_rate: FrameRate,
    repeat: bool,
    reverse: bool,
    delegates: LottieDelegates,
    options: LottieOptions,
    // onload: FnOnce(LottieComposition),
    image_provider_factory: LottieImageProviderFactory,
    key: String,
    // bundle: AssetBundle,
    frame_builder: LottieFrameBuilder,
    width: f64,
    height: f64,
    // fit: BoxFit,
    // alignment: Alignment,
    package: String,
    add_repaint_boundary: bool,
    provider: Rc<Option<Box<dyn LottieProvider>>>,
}

pub trait LottieBuilderExt {
    /// Creates a widget that displays an `LottieComposition`
    /// obtained from an `AssetBundle`.
    fn asset(&self, name: &str) -> &Self;

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from a `File`.
    fn file(&self, file: File) -> &Self;

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from a `Uint8List`.
    fn memory(&self, byf: Bytes) -> &Self;

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from the network.
    fn network(&self, url: &str) -> &Self;

    // /// The animation controller to animate the Lottie animation.
    // /// If null, a controller is automatically created by this class and is configured
    // /// with the properties `animate`, `reverse`
    // fn controller(&self, controller: Animation<f64>) -> &Self;

    /// The number of frames per second to render.
    /// Use `FrameRate.composition` to use the original frame rate of the Lottie composition (default)
    /// Use `FrameRate.max` to advance the animation progression at every frame.
    ///
    /// The advantage of using a low frame rate is to preserve the device battery
    /// by doing less rendering work.
    fn framerate(&self, framerate: FrameRate) -> &Self;

    /// If no controller is specified, this value indicate whether or not the
    /// Lottie animation should be played automatically (default to true).
    /// If there is an animation controller specified, this property has no effect.
    ///
    /// See [repeat] to control whether the animation should repeat.
    fn animate(&self, animate: bool) -> &Self;

    /// Specify that the automatic animation should repeat in a loop (default to true).
    /// The property has no effect if `animate` is false or `controller` is not null.
    fn repeat(&self, repeat: bool) -> &Self;

    /// Specify that the automatic animation should repeat in a loop in a "reverse"
    /// mode (go from start to end and then continuously from end to start).
    /// It default to false.
    /// The property has no effect if `animate` is false, `repeat` is false or `controller` is not null.
    fn reverse(&self, reverse: bool) -> &Self;

    /// A group of callbacks to further customize the lottie animation.
    /// - A `text` delegate to dynamically change some text displayed in the animation
    /// - A value callback to change the properties of the animation at runtime.
    /// - A text style factory to map between a font family specified in the animation
    ///   and the font family in your assets.
    fn delegates(&self, delegates: LottieDelegates) -> &Self;

    /// Some options to enable/disable some feature of Lottie
    /// - enableMergePaths: Enable merge path support
    fn options(&self, options: LottieDelegates) -> &Self;
    fn onloaded(&self, f: &str) -> &Self;
    fn image_provider_factory(&self, factory: LottieImageProviderFactory) -> &Self;
    fn key(&self, key: &str) -> &Self;
    // fn bundle(&self, bundle: AssetBundle) -> &Self;
    fn framebuilder(&self, bundle: LottieFrameBuilder) -> &Self;

    /// If non-null, requires the composition to have this width.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    fn width(&self, width: f64) -> &Self;

    /// If non-null, require the composition to have this height.
    ///
    /// If null, the composition will pick a size that best preserves its intrinsic
    /// aspect ratio.
    fn height(&self, height: f64) -> &Self;

    // /// How to inscribe the Lottie composition into the space allocated during layout.
    // fn fit(&self, fit: BoxFit) -> &Self;

    // /// How to align the composition within its bounds.
    // ///
    // /// The alignment aligns the given position in the image to the given position
    // /// in the layout bounds. For example, an `Alignment` alignment of (-1.0,
    // /// -1.0) aligns the image to the top-left corner of its layout bounds, while a
    // /// `Alignment` alignment of (1.0, 1.0) aligns the bottom right of the
    // /// image with the bottom right corner of its layout bounds. Similarly, an
    // /// alignment of (0.0, 1.0) aligns the bottom middle of the image with the
    // /// middle of the bottom edge of its layout bounds.
    // ///
    // /// Defaults to [Alignment.center].
    // ///
    // /// See also:
    // ///
    // ///  * `Alignment`, a class with convenient constants typically used to
    // ///    specify an `AlignmentGeometry`.
    // ///  * `AlignmentDirectional`, like `Alignment` for specifying alignments
    // ///    relative to text direction.
    // fn alignment(&self, alignment: Alignment) -> &Self;

    fn package(&self, package: &str) -> &Self;

    /// Indicate to automatically add a `RepaintBoundary` widget around the animation.
    /// This allows to optimize the app performance by isolating the animation in its
    /// own `Layer`.
    ///
    /// This property is `true` by default.
    fn add_repaint_boundary(&self, value: bool) -> &Self;
}

impl LottieBuilderExt for Builder<Lottie> {
    /// Creates a widget that displays an `LottieComposition`
    /// obtained from an `AssetBundle`.
    fn asset(&self, name: &str) -> &Self {
        unimplemented!()
    }

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from a `File`.
    fn file(&self, file: File) -> &Self {
        unimplemented!()
    }

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from a `Uint8List`.
    fn memory(&self, byf: Bytes) -> &Self {
        unimplemented!()
    }

    /// Creates a widget that displays an `LottieComposition`
    /// obtained from the network.
    fn network(&self, url: &str) -> &Self {
        unimplemented!()
    }

    // fn controller(&self, controller: Animation<f64>) -> &Self {
    //     unimplemented!()
    // }

    fn framerate(&self, framerate: FrameRate) -> &Self {
        unimplemented!()
    }

    fn animate(&self, animate: bool) -> &Self {
        unimplemented!()
    }

    fn repeat(&self, repeat: bool) -> &Self {
        unimplemented!()
    }

    fn reverse(&self, reverse: bool) -> &Self {
        unimplemented!()
    }

    fn delegates(&self, delegates: LottieDelegates) -> &Self {
        unimplemented!()
    }

    fn options(&self, options: LottieDelegates) -> &Self {
        unimplemented!()
    }

    // should be fn
    fn onloaded(&self, f: &str) -> &Self {
        unimplemented!()
    }

    fn image_provider_factory(&self, factory: LottieImageProviderFactory) -> &Self {
        unimplemented!()
    }

    fn key(&self, key: &str) -> &Self {
        unimplemented!()
    }

    // fn bundle(&self, bundle: AssetBundle) -> &Self {
    //     unimplemented!()
    // }

    fn framebuilder(&self, bundle: LottieFrameBuilder) -> &Self {
        unimplemented!()
    }

    fn width(&self, width: f64) -> &Self {
        unimplemented!()
    }

    fn height(&self, height: f64) -> &Self {
        unimplemented!()
    }

    // fn fit(&self, fit: BoxFit) -> &Self {
    //     unimplemented!()
    // }

    // fn alignment(&self, alignment: Alignment) -> &Self {
    //     unimplemented!()
    // }

    fn package(&self, package: &str) -> &Self {
        unimplemented!()
    }

    fn add_repaint_boundary(&self, value: bool) -> &Self {
        unimplemented!()
    }
}

impl Lottie {
    fn new() -> Self {
        unimplemented!()
    }
}
