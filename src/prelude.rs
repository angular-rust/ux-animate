#[doc(hidden)]
pub use gio::prelude::*;

#[doc(hidden)]
pub use glib::prelude::*;

#[doc(hidden)]
pub use pango::prelude::*;

pub use primitives::prelude::*;
pub use ruex::prelude::*;

pub use crate::lottie::LottieBuilderExt;
pub use crate::AdvancedShapesExt;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::legacy::traits::*;

pub trait Object: std::fmt::Debug + Clone + 'static {}
pub trait Is<T: Object>: AsRef<T> + 'static {}
