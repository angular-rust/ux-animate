use super::{BinAlignment, Container};
use std::fmt;

// * @short_description: A simple layout manager
// *
// * #ClutterBinLayout is a layout manager which implements the following
// * policy:
// *
// *   - the preferred size is the maximum preferred size
// *   between all the children of the container using the
// *   layout;
// *   - each child is allocated in "layers", on on top
// *   of the other;
// *   - for each layer there are horizontal and vertical
// *   alignment policies.
// *
// * The [bin-layout example](https://git.gnome.org/browse/clutter/tree/examples/bin-layout.c?h=clutter-1.18)
// * shows how to pack actors inside a #ClutterBinLayout.
// @extends LayoutManager
pub struct BinLayout{
    x_align: BinAlignment,
    y_align: BinAlignment,
  
    container: Option<Container>
}

impl BinLayout {
    /// Creates a new `BinLayout` layout manager
    /// ## `x_align`
    /// the default alignment policy to be used on the
    ///  horizontal axis
    /// ## `y_align`
    /// the default alignment policy to be used on the
    ///  vertical axis
    ///
    /// # Returns
    ///
    /// the newly created layout manager
    pub fn new(x_align: BinAlignment, y_align: BinAlignment) -> BinLayout {
        // unsafe {
        //     LayoutManager::from_glib_none(ffi::clutter_bin_layout_new(
        //         x_align.to_glib(),
        //         y_align.to_glib(),
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl fmt::Display for BinLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BinLayout")
    }
}
