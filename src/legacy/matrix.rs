use glib::translate::*;
// use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Matrix(Boxed<ffi::ClutterMatrix>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_matrix_get_type(), ptr as *mut _) as *mut ffi::ClutterMatrix,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_matrix_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_matrix_get_type(),
    }
}

impl Matrix {
    //pub fn init_from_array(&mut self, values: /*Unimplemented*/FixedArray TypeId { ns_id: 0, id: 20 }; 16) -> Option<Matrix> {
    //    unsafe { TODO: call clutter_sys:clutter_matrix_init_from_array() }
    //}

    /// Initializes the `Matrix` `self` with the contents of the
    /// `Matrix` `b`.
    /// ## `b`
    /// the `Matrix` to copy
    ///
    /// # Returns
    ///
    /// the initialized `Matrix`
    pub fn init_from_matrix(&mut self, b: &Matrix) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::clutter_matrix_init_from_matrix(
                self.to_glib_none_mut().0,
                b.to_glib_none().0,
            ))
        }
    }

    /// Initializes `self` with the identity matrix, i.e.:
    ///
    ///
    /// ```text
    ///   .xx = 1.0, .xy = 0.0, .xz = 0.0, .xw = 0.0
    ///   .yx = 0.0, .yy = 1.0, .yz = 0.0, .yw = 0.0
    ///   .zx = 0.0, .zy = 0.0, .zz = 1.0, .zw = 0.0
    ///   .wx = 0.0, .wy = 0.0, .wz = 0.0, .ww = 1.0
    /// ```
    ///
    /// # Returns
    ///
    /// the initialized `Matrix`
    pub fn init_identity(&mut self) -> Option<Matrix> {
        unsafe { from_glib_none(ffi::clutter_matrix_init_identity(self.to_glib_none_mut().0)) }
    }

    /// Allocates enough memory to hold a `Matrix`.
    ///
    /// # Returns
    ///
    /// the newly allocated `Matrix`
    pub fn alloc() -> Option<Matrix> {
        unsafe { from_glib_full(ffi::clutter_matrix_alloc()) }
    }
}

#[doc(hidden)]
impl Uninitialized for Matrix {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc().unwrap()
    }
}
