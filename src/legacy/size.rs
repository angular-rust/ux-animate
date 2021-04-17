#![allow(dead_code)]
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InternalSize(Boxed<ffi::ClutterSize>);

    match fn {
        copy => |ptr| ffi::clutter_size_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_size_free(ptr),
        get_type => || ffi::clutter_size_get_type(),
    }
}

impl InternalSize {
    /// Allocates a new `Size`.
    ///
    /// # Returns
    ///
    /// the newly allocated `Size`.
    ///  Use `Size::free` to free its resources.
    pub fn alloc() -> InternalSize {
        unsafe { from_glib_full(ffi::clutter_size_alloc()) }
    }

    /// Compares two `Size` for equality.
    /// ## `b`
    /// a `Size` to compare
    ///
    /// # Returns
    ///
    /// `true` if the two `Size` are equal
    pub fn equals(&self, b: &InternalSize) -> bool {
        unsafe {
            from_glib(ffi::clutter_size_equals(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    /// Initializes a `Size` with the given dimensions.
    /// ## `width`
    /// the width
    /// ## `height`
    /// the height
    ///
    /// # Returns
    ///
    /// the initialized `Size`
    pub fn init(&mut self, width: f32, height: f32) -> Option<InternalSize> {
        unsafe {
            from_glib_none(ffi::clutter_size_init(
                self.to_glib_none_mut().0,
                width,
                height,
            ))
        }
    }
}
