use crate::{Color, InternalColor, PaintNode, PipelineNode, RgbaColor};
use glib::{object::Cast, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct ColorNode(Object<ffi::ClutterColorNode, ffi::ClutterColorNodeClass, ColorNodeClass>) @extends PipelineNode, PaintNode;

    match fn {
        get_type => || ffi::clutter_color_node_get_type(),
    }
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
        let color = match color {
            Some(value) => {
                let RgbaColor {
                    red,
                    green,
                    blue,
                    alpha,
                } = value.into();
                Some(InternalColor::new(red, green, blue, alpha))
            }
            None => None,
        };
        unsafe {
            PaintNode::from_glib_full(ffi::clutter_color_node_new(color.to_glib_none().0))
                .unsafe_cast()
        }
    }
}

impl fmt::Display for ColorNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorNode")
    }
}
