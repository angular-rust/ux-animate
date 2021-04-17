use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct Knot(Boxed<ffi::ClutterKnot>);

    match fn {
        copy => |ptr| ffi::clutter_knot_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_knot_free(ptr),
        get_type => || ffi::clutter_knot_get_type(),
    }
}

impl Knot {
    /// Compares to knot and checks if the point to the same location.
    /// ## `knot_b`
    /// Second knot
    ///
    /// # Returns
    ///
    /// `true` if the knots point to the same location.
    fn equal(&self, knot_b: &Knot) -> bool {
        unsafe {
            from_glib(ffi::clutter_knot_equal(
                self.to_glib_none().0,
                knot_b.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for Knot {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Knot {}

#[doc(hidden)]
impl Uninitialized for Knot {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}
