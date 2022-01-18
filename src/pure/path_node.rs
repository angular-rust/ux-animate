use super::{Knot, PathNodeType};
// PathNode:
// @type: the node's type
// @points: the coordinates of the node
//
// Represents a single node of a #Path.
//
// Some of the coordinates in @points may be unused for some node
// types. %PATH_MOVE_TO and %PATH_LINE_TO use only one
// pair of coordinates, %PATH_CURVE_TO uses all three and
// %PATH_CLOSE uses none.
#[derive(Debug, PartialOrd)] // Hash
pub struct PathNode {
    kind: PathNodeType,
    points: [Knot; 3],
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
        //     from_glib(ffi::path_node_equal(
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

