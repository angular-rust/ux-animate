#![allow(unused_imports)]
#![allow(deprecated)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]

#[macro_use]
extern crate log;

mod canvas;
pub use canvas::*;

pub mod easing;

pub mod tween;

pub mod motion;

pub mod physics;

mod runtime;
pub use runtime::*;

pub mod svg;

pub mod path;

pub mod interpolate;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate glib;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate bitflags;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod rt;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod macros;

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
mod legacy;

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::type_complexity))]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
mod pure;

#[cfg(not(target_arch = "wasm32"))]
pub mod prelude;

#[cfg(not(target_arch = "wasm32"))]
pub use legacy::*;

#[cfg(not(target_arch = "wasm32"))]
pub use self::rt::{init, quit, run, set_initialized};

pub use primitives::colorspace::{Color, RgbColor, RgbaColor};
pub use primitives::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
