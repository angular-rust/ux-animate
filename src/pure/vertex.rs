#[derive(Debug, PartialOrd, Clone)] // Hash
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    /// Allocates a new, empty `Vertex`.
    ///
    /// # Returns
    ///
    /// the newly allocated `Vertex`.
    ///  Use `Vertex::free` to free its resources
    pub fn alloc() -> Vertex {
        // unsafe { from_glib_full(ffi::clutter_vertex_alloc()) }
        unimplemented!()
    }

    /// Creates a new `Vertex` for the point in 3D space
    /// identified by the 3 coordinates `x`, `y`, `z`.
    ///
    /// This function is the logical equivalent of:
    ///
    ///
    /// ```text
    ///   vertex_init (vertex_alloc (), x, y, z);
    /// ```
    /// ## `x`
    /// X coordinate
    /// ## `y`
    /// Y coordinate
    /// ## `z`
    /// Z coordinate
    ///
    /// # Returns
    ///
    /// the newly allocated `Vertex`.
    ///  Use `Vertex::free` to free the resources
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        // unsafe { from_glib_full(ffi::clutter_vertex_new(x, y, z)) }
        unimplemented!()
    }

    /// Compares `self` and `vertex_b` for equality
    /// ## `vertex_b`
    /// a `Vertex`
    ///
    /// # Returns
    ///
    /// `true` if the passed `Vertex` are equal
    fn equal(&self, vertex_b: &Vertex) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_vertex_equal(
        //         self.to_glib_none().0,
        //         vertex_b.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    /// Initializes `self` with the given coordinates.
    /// ## `x`
    /// X coordinate
    /// ## `y`
    /// Y coordinate
    /// ## `z`
    /// Z coordinate
    ///
    /// # Returns
    ///
    /// the initialized `Vertex`
    pub fn init(&mut self, x: f32, y: f32, z: f32) -> Option<Vertex> {
        // unsafe { from_glib_none(ffi::clutter_vertex_init(self.to_glib_none_mut().0, x, y, z)) }
        unimplemented!()
    }
}

impl PartialEq for Vertex {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Vertex {}
