#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

pub const VERSION: &str = "18.6.4";

// pub const MAIN_GROUP: Group = Group::new();

// Convert [seconds, nanoseconds] to milliseconds.
// let now: () => number
// const now = fn() {
//     Date.getTime()
// }

mod easing;
pub use easing::*;

mod group;
pub use group::*;

mod index;
pub use index::*;

mod sequence;
pub use sequence::*;

mod tests;
pub use tests::*;

mod tween;
pub use tween::*;