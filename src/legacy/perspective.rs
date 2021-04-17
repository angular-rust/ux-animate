use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Perspective(Boxed<ffi::ClutterPerspective>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_perspective_get_type(), ptr as *mut _) as *mut ffi::ClutterPerspective,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_perspective_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_perspective_get_type(),
    }
}

#[doc(hidden)]
impl Uninitialized for Perspective {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}