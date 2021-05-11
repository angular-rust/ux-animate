use super::{Knot, PathNode};
use crate::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

#[derive(Debug, Clone)]
pub struct Path {}

impl Path {
    /// Creates a new `Path` instance with no nodes.
    ///
    /// The object has a floating reference so if you add it to a
    /// `BehaviourPath` then you do not need to unref it.
    ///
    /// # Returns
    ///
    /// the newly created `Path`
    pub fn new() -> Path {
        // unsafe { from_glib_none(ffi::clutter_path_new()) }
        unimplemented!()
    }

    pub fn with_description(desc: &str) -> Path {
        // unsafe {
        //     from_glib_none(ffi::clutter_path_new_with_description(
        //         desc.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }
}

impl Object for Path {}
impl Is<Path> for Path {}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `Path` methods.
///
/// # Implementors
///
/// [`Path`](struct.Path.html)
pub trait PathExt: 'static {
    // /// Add the nodes of the Cairo path to the end of `self`.
    // /// ## `cpath`
    // /// a Cairo path
    // fn add_cairo_path(&self, cpath: &cairo::Path);

    /// Adds a `PathNodeType::Close` type node to the path. This creates a
    /// straight line from the last node to the last `PathNodeType::MoveTo`
    /// type node.
    fn add_close(&self);

    /// Adds a `PathNodeType::CurveTo` type node to the path. This causes
    /// the actor to follow a bezier from the last node to (`x_3`, `y_3`) using
    /// (`x_1`, `y_1`) and (`x_2`,`y_2`) as control points.
    /// ## `x_1`
    /// the x coordinate of the first control point
    /// ## `y_1`
    /// the y coordinate of the first control point
    /// ## `x_2`
    /// the x coordinate of the second control point
    /// ## `y_2`
    /// the y coordinate of the second control point
    /// ## `x_3`
    /// the x coordinate of the third control point
    /// ## `y_3`
    /// the y coordinate of the third control point
    fn add_curve_to(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32, x_3: i32, y_3: i32);

    /// Adds a `PathNodeType::LineTo` type node to the path. This causes the
    /// actor to move to the new coordinates in a straight line.
    /// ## `x`
    /// the x coordinate
    /// ## `y`
    /// the y coordinate
    fn add_line_to(&self, x: i32, y: i32);

    /// Adds a `PathNodeType::MoveTo` type node to the path. This is usually
    /// used as the first node in a path. It can also be used in the middle
    /// of the path to cause the actor to jump to the new coordinate.
    /// ## `x`
    /// the x coordinate
    /// ## `y`
    /// the y coordinate
    fn add_move_to(&self, x: i32, y: i32);

    /// Adds `node` to the end of the path.
    /// ## `node`
    /// a `PathNode`
    fn add_node(&self, node: &PathNode);

    /// Same as `PathExt::add_curve_to` except the coordinates are
    /// relative to the previous node.
    /// ## `x_1`
    /// the x coordinate of the first control point
    /// ## `y_1`
    /// the y coordinate of the first control point
    /// ## `x_2`
    /// the x coordinate of the second control point
    /// ## `y_2`
    /// the y coordinate of the second control point
    /// ## `x_3`
    /// the x coordinate of the third control point
    /// ## `y_3`
    /// the y coordinate of the third control point
    fn add_rel_curve_to(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32, x_3: i32, y_3: i32);

    /// Same as `PathExt::add_line_to` except the coordinates are
    /// relative to the previous node.
    /// ## `x`
    /// the x coordinate
    /// ## `y`
    /// the y coordinate
    fn add_rel_line_to(&self, x: i32, y: i32);

    /// Same as `PathExt::add_move_to` except the coordinates are
    /// relative to the previous node.
    /// ## `x`
    /// the x coordinate
    /// ## `y`
    /// the y coordinate
    fn add_rel_move_to(&self, x: i32, y: i32);

    /// Adds new nodes to the end of the path as described in `str`. The
    /// format is a subset of the SVG path format. Each node is represented
    /// by a letter and is followed by zero, one or three pairs of
    /// coordinates. The coordinates can be separated by spaces or a
    /// comma. The types are:
    ///
    ///  - `M`: Adds a `PathNodeType::MoveTo` node. Takes one pair of coordinates.
    ///  - `L`: Adds a `PathNodeType::LineTo` node. Takes one pair of coordinates.
    ///  - `C`: Adds a `PathNodeType::CurveTo` node. Takes three pairs of coordinates.
    ///  - `z`: Adds a `PathNodeType::Close` node. No coordinates are needed.
    ///
    /// The M, L and C commands can also be specified in lower case which
    /// means the coordinates are relative to the previous node.
    ///
    /// For example, to move an actor in a 100 by 100 pixel square centered
    /// on the point 300,300 you could use the following path:
    ///
    ///
    /// ```text
    ///   M 250,350 l 0 -100 L 350,250 l 0 100 z
    /// ```
    ///
    /// If the path description isn't valid `false` will be returned and no
    /// nodes will be added.
    /// ## `str`
    /// a string describing the new nodes
    ///
    /// # Returns
    ///
    /// `true` is the path description was valid or `false`
    /// otherwise.
    fn add_string(&self, str: &str) -> bool;

    /// Removes all nodes from the path.
    fn clear(&self);

    /// Calls a function for each node of the path.
    /// ## `callback`
    /// the function to call with each node
    /// ## `user_data`
    /// user data to pass to the function
    fn foreach<P: FnMut(&PathNode)>(&self, callback: P);

    /// Returns a newly allocated string describing the path in the same
    /// format as used by `PathExt::add_string`.
    ///
    /// # Returns
    ///
    /// a string description of the path. Free with `g_free`.
    fn get_description(&self) -> Option<String>;

    /// Retrieves an approximation of the total length of the path.
    ///
    /// # Returns
    ///
    /// the length of the path.
    fn get_length(&self) -> u32;

    /// Retrieves the number of nodes in the path.
    ///
    /// # Returns
    ///
    /// the number of nodes.
    fn get_n_nodes(&self) -> u32;

    /// Retrieves the node of the path indexed by `index`.
    /// ## `index_`
    /// the node number to retrieve
    /// ## `node`
    /// a location to store a copy of the node
    fn get_node(&self, index_: u32) -> PathNode;

    /// Returns a `glib::SList` of `PathNode`<!-- -->s. The list should be
    /// freed with `glib::SList::free`. The nodes are owned by the path and
    /// should not be freed. Altering the path may cause the nodes in the
    /// list to become invalid so you should copy them if you want to keep
    /// the list.
    ///
    /// # Returns
    ///
    /// a
    ///  list of nodes in the path.
    fn get_nodes(&self) -> Vec<PathNode>;

    /// The value in `progress` represents a position along the path where
    /// 0.0 is the beginning and 1.0 is the end of the path. An
    /// interpolated position is then stored in `position`.
    /// ## `progress`
    /// a position along the path as a fraction of its length
    /// ## `position`
    /// location to store the position
    ///
    /// # Returns
    ///
    /// index of the node used to calculate the position.
    fn get_position(&self, progress: f64) -> (u32, Knot);

    /// Inserts `node` into the path before the node at the given offset. If
    /// `index_` is negative it will append the node to the end of the path.
    /// ## `index_`
    /// offset of where to insert the node
    /// ## `node`
    /// the node to insert
    fn insert_node(&self, index_: i32, node: &PathNode);

    /// Removes the node at the given offset from the path.
    /// ## `index_`
    /// index of the node to remove
    fn remove_node(&self, index_: u32);

    /// Replaces the node at offset `index_` with `node`.
    /// ## `index_`
    /// index to the existing node
    /// ## `node`
    /// the replacement node
    fn replace_node(&self, index_: u32, node: &PathNode);

    /// Replaces all of the nodes in the path with nodes described by
    /// `str`. See `PathExt::add_string` for details of the format.
    ///
    /// If the string is invalid then `false` is returned and the path is
    /// unaltered.
    /// ## `str`
    /// a string describing the path
    ///
    /// # Returns
    ///
    /// `true` is the path was valid, `false` otherwise.
    fn set_description(&self, str: &str) -> bool;

    // /// Add the nodes of the Path to the path in the Cairo context.
    // /// ## `cr`
    // /// a Cairo context
    // fn to_cairo_path(&self, cr: &mut cairo::Context);

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Path>> PathExt for O {
    // fn add_cairo_path(&self, cpath: &cairo::Path) {
    //     unsafe {
    //         ffi::clutter_path_add_cairo_path(
    //             self.as_ref().to_glib_none().0,
    //             cpath.to_glib_none().0,
    //         );
    //     }
    // }

    fn add_close(&self) {
        // unsafe {
        //     ffi::clutter_path_add_close(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn add_curve_to(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32, x_3: i32, y_3: i32) {
        // unsafe {
        //     ffi::clutter_path_add_curve_to(
        //         self.as_ref().to_glib_none().0,
        //         x_1,
        //         y_1,
        //         x_2,
        //         y_2,
        //         x_3,
        //         y_3,
        //     );
        // }
        unimplemented!()
    }

    fn add_line_to(&self, x: i32, y: i32) {
        // unsafe {
        //     ffi::clutter_path_add_line_to(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn add_move_to(&self, x: i32, y: i32) {
        // unsafe {
        //     ffi::clutter_path_add_move_to(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn add_node(&self, node: &PathNode) {
        // unsafe {
        //     ffi::clutter_path_add_node(self.as_ref().to_glib_none().0, node.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn add_rel_curve_to(&self, x_1: i32, y_1: i32, x_2: i32, y_2: i32, x_3: i32, y_3: i32) {
        // unsafe {
        //     ffi::clutter_path_add_rel_curve_to(
        //         self.as_ref().to_glib_none().0,
        //         x_1,
        //         y_1,
        //         x_2,
        //         y_2,
        //         x_3,
        //         y_3,
        //     );
        // }
        unimplemented!()
    }

    fn add_rel_line_to(&self, x: i32, y: i32) {
        // unsafe {
        //     ffi::clutter_path_add_rel_line_to(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn add_rel_move_to(&self, x: i32, y: i32) {
        // unsafe {
        //     ffi::clutter_path_add_rel_move_to(self.as_ref().to_glib_none().0, x, y);
        // }
    }

    fn add_string(&self, str: &str) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_path_add_string(
        //         self.as_ref().to_glib_none().0,
        //         str.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn clear(&self) {
        // unsafe {
        //     ffi::clutter_path_clear(self.as_ref().to_glib_none().0);
        // }
    }

    fn foreach<P: FnMut(&PathNode)>(&self, callback: P) {
        // let callback_data: P = callback;
        // unsafe extern "C" fn callback_func<P: FnMut(&PathNode)>(
        //     node: *const ffi::ClutterPathNode,
        //     data: glib_sys::gpointer,
        // ) {
        //     let node = from_glib_borrow(node);
        //     let callback: *mut P = data as *const _ as usize as *mut P;
        //     (*callback)(&node);
        // }
        // let callback = Some(callback_func::<P> as _);
        // let super_callback0: &P = &callback_data;
        // unsafe {
        //     ffi::clutter_path_foreach(
        //         self.as_ref().to_glib_none().0,
        //         callback,
        //         super_callback0 as *const _ as usize as *mut _,
        //     );
        // }
        unimplemented!()
    }

    fn get_description(&self) -> Option<String> {
        // unsafe {
        //     from_glib_full(ffi::clutter_path_get_description(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_length(&self) -> u32 {
        // unsafe { ffi::clutter_path_get_length(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_n_nodes(&self) -> u32 {
        // unsafe { ffi::clutter_path_get_n_nodes(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_node(&self, index_: u32) -> PathNode {
        // unsafe {
        //     let mut node = PathNode::uninitialized();
        //     ffi::clutter_path_get_node(
        //         self.as_ref().to_glib_none().0,
        //         index_,
        //         node.to_glib_none_mut().0,
        //     );
        //     node
        // }
        unimplemented!()
    }

    fn get_nodes(&self) -> Vec<PathNode> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_path_get_nodes(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_position(&self, progress: f64) -> (u32, Knot) {
        // unsafe {
        //     let mut position = Knot::uninitialized();
        //     let ret = ffi::clutter_path_get_position(
        //         self.as_ref().to_glib_none().0,
        //         progress,
        //         position.to_glib_none_mut().0,
        //     );
        //     (ret, position)
        // }
        unimplemented!()
    }

    fn insert_node(&self, index_: i32, node: &PathNode) {
        // unsafe {
        //     ffi::clutter_path_insert_node(
        //         self.as_ref().to_glib_none().0,
        //         index_,
        //         node.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_node(&self, index_: u32) {
        // unsafe {
        //     ffi::clutter_path_remove_node(self.as_ref().to_glib_none().0, index_);
        // }
        unimplemented!()
    }

    fn replace_node(&self, index_: u32, node: &PathNode) {
        // unsafe {
        //     ffi::clutter_path_replace_node(
        //         self.as_ref().to_glib_none().0,
        //         index_,
        //         node.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_description(&self, str: &str) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_path_set_description(
        //         self.as_ref().to_glib_none().0,
        //         str.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    // fn to_cairo_path(&self, cr: &mut cairo::Context) {
    //     unsafe {
    //         ffi::clutter_path_to_cairo_path(
    //             self.as_ref().to_glib_none().0,
    //             cr.to_glib_none_mut().0,
    //         );
    //     }
    // }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Path")
    }
}
