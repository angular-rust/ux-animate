#[derive(Debug, PartialOrd, Ord)] // Hash
pub struct PathNode {
}

impl PathNode {
    /// Compares two nodes and checks if they are the same type with the
    /// same coordinates.
    /// ## `node_b`
    /// Second node
    ///
    /// # Returns
    ///
    /// `true` if the nodes are the same.
    fn equal(&self, node_b: &PathNode) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_path_node_equal(
        //         self.to_glib_none().0,
        //         node_b.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }
}

impl PartialEq for PathNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for PathNode {}

