use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct PathNode(Boxed<ffi::ClutterPathNode>);

    match fn {
        copy => |ptr| ffi::clutter_path_node_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_path_node_free(ptr),
        get_type => || ffi::clutter_path_node_get_type(),
    }
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
        unsafe {
            from_glib(ffi::clutter_path_node_equal(
                self.to_glib_none().0,
                node_b.to_glib_none().0,
            ))
        }
    }
}

impl PartialEq for PathNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for PathNode {}

#[doc(hidden)]
impl Uninitialized for PathNode {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}
