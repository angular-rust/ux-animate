#[derive(Debug, Clone)]
pub struct Knot {
    x: i32,
    y: i32,
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
        // unsafe {
        //     from_glib(ffi::clutter_knot_equal(
        //         self.to_glib_none().0,
        //         knot_b.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }
}

impl PartialEq for Knot {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Knot {}
