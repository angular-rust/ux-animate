use super::PaintNode;
use std::fmt;

#[derive(Default)]
pub struct ClipNode {
    inner: PaintNode,
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
        Self{
            inner: Default::default()
        }
    }
}

impl fmt::Display for ClipNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClipNode")
    }
}
