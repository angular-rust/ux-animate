use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InternalPoint(Boxed<ffi::ClutterPoint>);

    match fn {
        copy => |ptr| ffi::clutter_point_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_point_free(ptr),
        get_type => || ffi::clutter_point_get_type(),
    }
}

impl InternalPoint {
    /// Allocates a new `Point`.
    ///
    /// # Returns
    ///
    /// the newly allocated `Point`.
    ///  Use `Point::free` to free its resources.
    pub fn alloc() -> InternalPoint {
        unsafe { from_glib_full(ffi::clutter_point_alloc()) }
    }

    /// Computes the distance between two `Point`.
    /// ## `b`
    /// a `Point`
    /// ## `x_distance`
    /// return location for the horizontal
    ///  distance between the points
    /// ## `y_distance`
    /// return location for the vertical
    ///  distance between the points
    ///
    /// # Returns
    ///
    /// the distance between the points.
    pub fn distance(&self, b: &InternalPoint) -> (f32, f32, f32) {
        unsafe {
            let mut x_distance = mem::MaybeUninit::uninit();
            let mut y_distance = mem::MaybeUninit::uninit();
            let ret = ffi::clutter_point_distance(
                self.to_glib_none().0,
                b.to_glib_none().0,
                x_distance.as_mut_ptr(),
                y_distance.as_mut_ptr(),
            );
            let x_distance = x_distance.assume_init();
            let y_distance = y_distance.assume_init();
            (ret, x_distance, y_distance)
        }
    }

    /// Compares two `Point` for equality.
    /// ## `b`
    /// the second `Point` to compare
    ///
    /// # Returns
    ///
    /// `true` if the `ClutterPoints` are equal
    pub fn equals(&self, b: &InternalPoint) -> bool {
        unsafe {
            from_glib(ffi::clutter_point_equals(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    /// Initializes `self` with the given coordinates.
    /// ## `x`
    /// the X coordinate of the point
    /// ## `y`
    /// the Y coordinate of the point
    ///
    /// # Returns
    ///
    /// the initialized `Point`
    pub fn init(&mut self, x: f32, y: f32) -> Option<InternalPoint> {
        unsafe { from_glib_none(ffi::clutter_point_init(self.to_glib_none_mut().0, x, y)) }
    }

    /// A point centered at (0, 0).
    ///
    /// The returned value can be used as a guard.
    ///
    /// # Returns
    ///
    /// a point centered in (0, 0); the returned `Point`
    ///  is owned by internals and it should not be modified or freed.
    pub fn zero() -> Option<InternalPoint> {
        unsafe { from_glib_none(ffi::clutter_point_zero()) }
    }
}

#[doc(hidden)]
impl Uninitialized for InternalPoint {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc()
    }
}