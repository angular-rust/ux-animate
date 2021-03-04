mod backend;
pub use backend::*;

mod motion;
pub use motion::*;

mod physics;
pub use physics::*;

mod runtime;
pub use runtime::*;

mod svg;
pub use svg::*;

mod tween;
pub use tween::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
