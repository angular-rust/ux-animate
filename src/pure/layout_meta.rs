use crate::prelude::*;
use super::{ChildMeta, LayoutManager};
use std::fmt;

// @extends ChildMeta
#[derive(Debug, Clone)]
pub struct LayoutMeta {
}

impl Object for LayoutMeta {}
impl Is<LayoutMeta> for LayoutMeta {}

impl AsRef<LayoutMeta> for LayoutMeta {
    fn as_ref(&self) -> &LayoutMeta {
        self
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

impl<O: Is<LayoutMeta>> LayoutMetaExt for O {
    fn get_manager(&self) -> Option<LayoutManager> {
        // unsafe {
        //     from_glib_none(ffi::clutter_layout_meta_get_manager(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }
}

impl fmt::Display for LayoutMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LayoutMeta")
    }
}
