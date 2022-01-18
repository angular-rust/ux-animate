use super::{Actor, HandlerId, Path};
use crate::prelude::*;
use std::fmt;

// @extends Constraint, ActorMeta,
#[derive(Debug, Clone)]
pub struct PathConstraint {}

impl PathConstraint {
    /// Creates a new `PathConstraint` with the given `path` and `offset`
    /// ## `path`
    /// a `Path`, or `None`
    /// ## `offset`
    /// the offset along the `Path`
    ///
    /// # Returns
    ///
    /// the newly created `PathConstraint`
    pub fn new<P: Is<Path>>(path: Option<&P>, offset: f32) -> PathConstraint {
        // unsafe {
        //     Constraint::from_glib_none(ffi::path_constraint_new(
        //         path.map(|p| p.as_ref()).to_glib_none().0,
        //         offset,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }

    /// Retrieves the offset along the `Path` used by `self`.
    ///
    /// # Returns
    ///
    /// the offset
    pub fn get_offset(&self) -> f32 {
        // unsafe { ffi::path_constraint_get_offset(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves a pointer to the `Path` used by `self`.
    ///
    /// # Returns
    ///
    /// the `Path` used by the
    ///  `PathConstraint`, or `None`. The returned `Path` is owned
    ///  by the constraint and it should not be unreferenced
    pub fn get_path(&self) -> Option<Path> {
        // unsafe { from_glib_none(ffi::path_constraint_get_path(self.to_glib_none().0)) }
        unimplemented!()
    }

    /// Sets the offset along the `Path` used by `self`.
    /// ## `offset`
    /// the offset along the path
    pub fn set_offset(&self, offset: f32) {
        // unsafe {
        //     ffi::path_constraint_set_offset(self.to_glib_none().0, offset);
        // }
        unimplemented!()
    }

    /// Sets the `path` to be followed by the `PathConstraint`.
    ///
    /// The `self` will take ownership of the `Path` passed to this
    /// function.
    /// ## `path`
    /// a `Path`
    pub fn set_path<P: Is<Path>>(&self, path: Option<&P>) {
        // unsafe {
        //     ffi::path_constraint_set_path(
        //         self.to_glib_none().0,
        //         path.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The ::node-reached signal is emitted each time a
    /// `PathConstraint:offset` value results in the actor
    /// passing a `PathNode`
    /// ## `actor`
    /// the `Actor` using the `constraint`
    /// ## `index`
    /// the index of the node that has been reached
    pub fn connect_node_reached<F: Fn(&PathConstraint, &Actor, u32) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_offset_notify<F: Fn(&PathConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_path_notify<F: Fn(&PathConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for PathConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PathConstraint")
    }
}
