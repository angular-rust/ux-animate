use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Margin(Boxed<ffi::ClutterMargin>);

    match fn {
        copy => |ptr| ffi::clutter_margin_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_margin_free(ptr),
        get_type => || ffi::clutter_margin_get_type(),
    }
}

impl Margin {
    /// Creates a new `Margin`.
    ///
    /// # Returns
    ///
    /// a newly allocated `Margin`. Use
    ///  `Margin::free` to free the resources associated with it when
    ///  done.
    pub fn new() -> Margin {
        unsafe { from_glib_full(ffi::clutter_margin_new()) }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
impl Uninitialized for Margin {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::new()
    }
}
