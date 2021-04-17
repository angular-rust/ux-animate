use crate::{ChildMeta, LayoutManager};
use glib::{object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct LayoutMeta(Object<ffi::ClutterLayoutMeta, ffi::ClutterLayoutMetaClass, LayoutMetaClass>) @extends ChildMeta;

    match fn {
        get_type => || ffi::clutter_layout_meta_get_type(),
    }
}

/// Trait containing all `LayoutMeta` methods.
///
/// # Implementors
///
/// [`LayoutMeta`](struct.LayoutMeta.html)
pub trait LayoutMetaExt: 'static {
    /// Retrieves the actor wrapped by `self`
    ///
    /// # Returns
    ///
    /// a `LayoutManager`
    fn get_manager(&self) -> Option<LayoutManager>;
}

impl<O: IsA<LayoutMeta>> LayoutMetaExt for O {
    fn get_manager(&self) -> Option<LayoutManager> {
        unsafe {
            from_glib_none(ffi::clutter_layout_meta_get_manager(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for LayoutMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayoutMeta")
    }
}
