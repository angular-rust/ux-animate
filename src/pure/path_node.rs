use super::{Knot, PathNodeType};
// * ClutterPathNode:
// * @type: the node's type
// * @points: the coordinates of the node
// *
// * Represents a single node of a #ClutterPath.
// *
// * Some of the coordinates in @points may be unused for some node
// * types. %CLUTTER_PATH_MOVE_TO and %CLUTTER_PATH_LINE_TO use only one
// * pair of coordinates, %CLUTTER_PATH_CURVE_TO uses all three and
// * %CLUTTER_PATH_CLOSE uses none.
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

