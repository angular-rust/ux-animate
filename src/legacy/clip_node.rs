use crate::PaintNode;
use glib::{object::Cast, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct ClipNode(Object<ffi::ClutterClipNode, ffi::ClutterClipNodeClass, ClipNodeClass>) @extends PaintNode;

    match fn {
        get_type => || ffi::clutter_clip_node_get_type(),
    }
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
        unsafe { PaintNode::from_glib_full(ffi::clutter_clip_node_new()).unsafe_cast() }
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
