use crate::prelude::*;
use super::PaintNode;
// use glib::{object::Cast, translate::*};
use std::fmt;

// @extends PaintNode
pub struct ClipNode {
}

impl ClipNode {
    /// Creates a new `PaintNode` that will clip its child
    /// nodes to the 2D regions added to it.
    ///
    /// # Returns
    ///
    /// the newly created `PaintNode`.
    ///  Use `PaintNodeExt::unref` when done.
    pub fn new() -> ClipNode {
        unimplemented!()
    }
}

impl Default for ClipNode {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ClipNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClipNode")
    }
}
