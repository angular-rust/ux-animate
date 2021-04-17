use crate::{BinAlignment, LayoutManager};
use glib::{object as gobject, object::Cast, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct BinLayout(Object<ffi::ClutterBinLayout, ffi::ClutterBinLayoutClass, BinLayoutClass>) @extends LayoutManager, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_bin_layout_get_type(),
    }
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
        unsafe {
            LayoutManager::from_glib_none(ffi::clutter_bin_layout_new(
                x_align.to_glib(),
                y_align.to_glib(),
            ))
            .unsafe_cast()
        }
    }
}

impl fmt::Display for BinLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BinLayout")
    }
}
