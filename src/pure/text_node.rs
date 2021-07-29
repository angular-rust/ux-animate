use crate::{Color};
use std::fmt;

// @extends PaintNode
pub struct TextNode {
}

impl TextNode {
    // /// Creates a new `PaintNode` that will paint a `pango::Layout`
    // /// with the given color.
    // ///
    // /// This function takes a reference on the passed `layout`, so it
    // /// is safe to call `gobject::ObjectExt::unref` after it returns.
    // /// ## `layout`
    // /// a `pango::Layout`, or `None`
    // /// ## `color`
    // /// the color used to paint the layout,
    // ///  or `None`
    // ///
    // /// # Returns
    // ///
    // /// the newly created `PaintNode`.
    // ///  Use `PaintNodeExt::unref` when done
    // pub fn new(layout: Option<&pango::Layout>, color: Option<Color>) -> TextNode {
    //     // let color = match color {
    //     //     Some(value) => {
    //     //         let RgbaColor {
    //     //             red,
    //     //             green,
    //     //             blue,
    //     //             alpha,
    //     //         } = value.into();
    //     //         Some(Color::new(red, green, blue, alpha))
    //     //     }
    //     //     None => None,
    //     // };
    //     // unsafe {
    //     //     PaintNode::from_glib_full(ffi::clutter_text_node_new(
    //     //         layout.to_glib_none().0,
    //     //         color.to_glib_none().0,
    //     //     ))
    //     //     .unsafe_cast()
    //     // }
    //     unimplemented!()
    // }
}

impl fmt::Display for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextNode")
    }
}
