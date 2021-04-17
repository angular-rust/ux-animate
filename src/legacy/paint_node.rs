use crate::ActorBox;
use glib::{object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct PaintNode(Object<ffi::ClutterPaintNode, ffi::ClutterPaintNodeClass, PaintNodeClass>);

    match fn {
        get_type => || ffi::clutter_paint_node_get_type(),
    }
}

/// Trait containing all `PaintNode` methods.
///
/// # Implementors
///
/// [`ClipNode`](struct.ClipNode.html), [`PaintNode`](struct.PaintNode.html), [`PipelineNode`](struct.PipelineNode.html), [`TextNode`](struct.TextNode.html)
pub trait PaintNodeExt: 'static {
    /// Adds `child` to the list of children of `self`.
    ///
    /// This function will acquire a reference on `child`.
    /// ## `child`
    /// the child `PaintNode` to add
    fn add_child<P: IsA<PaintNode>>(&self, child: &P);

    /// Adds a rectangle region to the `self`, as described by the
    /// passed `rect`.
    /// ## `rect`
    /// a `ActorBox`
    fn add_rectangle(&self, rect: &ActorBox);

    /// Adds a rectangle region to the `self`, with texture coordinates.
    /// ## `rect`
    /// a `ActorBox`
    /// ## `x_1`
    /// the left X coordinate of the texture
    /// ## `y_1`
    /// the top Y coordinate of the texture
    /// ## `x_2`
    /// the right X coordinate of the texture
    /// ## `y_2`
    /// the bottom Y coordinate of the texture
    fn add_texture_rectangle(&self, rect: &ActorBox, x_1: f32, y_1: f32, x_2: f32, y_2: f32);

    /// Sets a user-readable `name` for `self`.
    ///
    /// The `name` will be used for debugging purposes.
    ///
    /// The `self` will copy the passed string.
    /// ## `name`
    /// a string annotating the `self`
    fn set_name(&self, name: &str);
}

impl<O: IsA<PaintNode>> PaintNodeExt for O {
    fn add_child<P: IsA<PaintNode>>(&self, child: &P) {
        unsafe {
            ffi::clutter_paint_node_add_child(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_rectangle(&self, rect: &ActorBox) {
        unsafe {
            ffi::clutter_paint_node_add_rectangle(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
            );
        }
    }

    fn add_texture_rectangle(&self, rect: &ActorBox, x_1: f32, y_1: f32, x_2: f32, y_2: f32) {
        unsafe {
            ffi::clutter_paint_node_add_texture_rectangle(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
            );
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::clutter_paint_node_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }
}

impl fmt::Display for PaintNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PaintNode")
    }
}
