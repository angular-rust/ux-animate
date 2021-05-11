use crate::prelude::*;
use super::{PaintNode, PipelineNode};
use crate::{Color, RgbaColor};
// use glib::{object::Cast, translate::*};
use std::fmt;

// @extends PipelineNode, PaintNode
pub struct ColorNode {
}

impl ColorNode {
    /// Creates a new `PaintNode` that will paint a solid color
    /// fill using `color`.
    /// ## `color`
    /// the color to paint, or `None`
    ///
    /// # Returns
    ///
    /// the newly created `PaintNode`. Use
    ///  `PaintNodeExt::unref` when done
    pub fn new(color: Option<Color>) -> ColorNode {
        
        unimplemented!()
    }
}

impl fmt::Display for ColorNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorNode")
    }
}
