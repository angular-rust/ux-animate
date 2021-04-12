#[cfg(target_arch = "wasm32")]
mod web_canvas;
#[cfg(target_arch = "wasm32")]
pub use web_canvas::*;

#[cfg(not(target_arch = "wasm32"))]
mod cairo_canvas;
#[cfg(not(target_arch = "wasm32"))]
pub use cairo_canvas::*;