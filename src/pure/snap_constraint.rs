use super::{Actor, HandlerId, SnapEdge};
use crate::prelude::*;
use std::fmt;

// @extends Constraint, ActorMeta
pub struct SnapConstraint {}

impl SnapConstraint {
    /// Creates a new `SnapConstraint` that will snap a `Actor`
    /// to the `edge` of `source`, with the given `offset`.
    /// ## `source`
    /// the `Actor` to use as the source of
    ///  the constraint, or `None`
    /// ## `from_edge`
    /// the edge of the actor to use in the constraint
    /// ## `to_edge`
    /// the edge of `source` to use in the constraint
    /// ## `offset`
    /// the offset to apply to the constraint, in pixels
    ///
    /// # Returns
    ///
    /// the newly created `SnapConstraint`
    pub fn new<P: Is<Actor>>(
        source: Option<&P>,
        from_edge: SnapEdge,
        to_edge: SnapEdge,
        offset: f32,
    ) -> SnapConstraint {
        // unsafe {
        //     Constraint::from_glib_none(ffi::clutter_snap_constraint_new(
        //         source.map(|p| p.as_ref()).to_glib_none().0,
        //         from_edge.to_glib(),
        //         to_edge.to_glib(),
        //         offset,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }

    /// Retrieves the edges used by the `self`
    /// ## `from_edge`
    /// return location for the actor's edge, or `None`
    /// ## `to_edge`
    /// return location for the source's edge, or `None`
    pub fn get_edges(&self) -> (SnapEdge, SnapEdge) {
        // unsafe {
        //     let mut from_edge = mem::MaybeUninit::uninit();
        //     let mut to_edge = mem::MaybeUninit::uninit();
        //     ffi::clutter_snap_constraint_get_edges(
        //         self.to_glib_none().0,
        //         from_edge.as_mut_ptr(),
        //         to_edge.as_mut_ptr(),
        //     );
        //     let from_edge = from_edge.assume_init();
        //     let to_edge = to_edge.assume_init();
        //     (from_glib(from_edge), from_glib(to_edge))
        // }
        unimplemented!()
    }

    /// Retrieves the offset set using `SnapConstraint::set_offset`
    ///
    /// # Returns
    ///
    /// the offset, in pixels
    pub fn get_offset(&self) -> f32 {
        // unsafe { ffi::clutter_snap_constraint_get_offset(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves the `Actor` set using `SnapConstraint::set_source`
    ///
    /// # Returns
    ///
    /// a pointer to the source actor
    pub fn get_source(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_snap_constraint_get_source(
        //         self.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    /// Sets the edges to be used by the `self`
    ///
    /// The `from_edge` is the edge on the `Actor` to which `self`
    /// has been added. The `to_edge` is the edge of the `Actor` inside
    /// the `SnapConstraint:source` property.
    /// ## `from_edge`
    /// the edge on the actor
    /// ## `to_edge`
    /// the edge on the source
    pub fn set_edges(&self, from_edge: SnapEdge, to_edge: SnapEdge) {
        // unsafe {
        //     ffi::clutter_snap_constraint_set_edges(
        //         self.to_glib_none().0,
        //         from_edge.to_glib(),
        //         to_edge.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// Sets the offset to be applied to the constraint
    /// ## `offset`
    /// the offset to apply, in pixels
    pub fn set_offset(&self, offset: f32) {
        // unsafe {
        //     ffi::clutter_snap_constraint_set_offset(self.to_glib_none().0, offset);
        // }
        unimplemented!()
    }

    /// Sets the source `Actor` for the constraint
    /// ## `source`
    /// a `Actor`, or `None` to unset the source
    pub fn set_source<P: Is<Actor>>(&self, source: Option<&P>) {
        // unsafe {
        //     ffi::clutter_snap_constraint_set_source(
        //         self.to_glib_none().0,
        //         source.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The edge of the `Actor` that should be snapped
    pub fn get_property_from_edge(&self) -> SnapEdge {
        // unsafe {
        //     let mut value = Value::from_type(<SnapEdge as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"from-edge\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `from-edge` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The edge of the `Actor` that should be snapped
    pub fn set_property_from_edge(&self, from_edge: SnapEdge) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"from-edge\0".as_ptr() as *const _,
        //         Value::from(&from_edge).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The edge of the `SnapConstraint:source` that should be snapped
    pub fn get_property_to_edge(&self) -> SnapEdge {
        // unsafe {
        //     let mut value = Value::from_type(<SnapEdge as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"to-edge\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `to-edge` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The edge of the `SnapConstraint:source` that should be snapped
    pub fn set_property_to_edge(&self, to_edge: SnapEdge) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"to-edge\0".as_ptr() as *const _,
        //         Value::from(&to_edge).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn connect_property_from_edge_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_offset_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_source_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_to_edge_notify<F: Fn(&SnapConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for SnapConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SnapConstraint")
    }
}
