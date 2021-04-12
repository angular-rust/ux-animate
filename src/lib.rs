#[macro_use]
extern crate log;

mod backend;
pub use backend::*;

pub mod easing;

pub mod tween;

pub mod motion;

pub mod physics;

pub mod runtime;

pub mod svg;

pub mod interpolate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
