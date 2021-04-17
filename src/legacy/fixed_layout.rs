use crate::LayoutManager;
use glib::{object as gobject, object::Cast, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct FixedLayout(Object<ffi::ClutterFixedLayout, ffi::ClutterFixedLayoutClass, FixedLayoutClass>) @extends LayoutManager, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_fixed_layout_get_type(),
    }
}

impl FixedLayout {
    /// Creates a new `FixedLayout`
    ///
    /// # Returns
    ///
    /// the newly created `FixedLayout`
    pub fn new() -> FixedLayout {
        unsafe { LayoutManager::from_glib_none(ffi::clutter_fixed_layout_new()).unsafe_cast() }
    }
}

impl Default for FixedLayout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FixedLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedLayout")
    }
}
