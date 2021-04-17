use crate::{Actor, ActorMeta, BindCoordinate, Constraint};
use glib::{
    object as gobject,
    object::{Cast, IsA, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct BindConstraint(Object<ffi::ClutterBindConstraint, ffi::ClutterBindConstraintClass, BindConstraintClass>) @extends Constraint, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_bind_constraint_get_type(),
    }
}

impl BindConstraint {
    /// Creates a new constraint, binding a `Actor`'s position to
    /// the given `coordinate` of the position of `source`
    /// ## `source`
    /// the `Actor` to use as the source of
    ///  the binding, or `None`
    /// ## `coordinate`
    /// the coordinate to bind
    /// ## `offset`
    /// the offset to apply to the binding, in pixels
    ///
    /// # Returns
    ///
    /// the newly created `BindConstraint`
    pub fn new<P: IsA<Actor>>(
        source: Option<&P>,
        coordinate: BindCoordinate,
        offset: f32,
    ) -> BindConstraint {
        unsafe {
            Constraint::from_glib_none(ffi::clutter_bind_constraint_new(
                source.map(|p| p.as_ref()).to_glib_none().0,
                coordinate.to_glib(),
                offset,
            ))
            .unsafe_cast()
        }
    }

    /// Retrieves the bound coordinate of the constraint
    ///
    /// # Returns
    ///
    /// the bound coordinate
    pub fn get_coordinate(&self) -> BindCoordinate {
        unsafe {
            from_glib(ffi::clutter_bind_constraint_get_coordinate(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the offset set using `BindConstraint::set_offset`
    ///
    /// # Returns
    ///
    /// the offset, in pixels
    pub fn get_offset(&self) -> f32 {
        unsafe { ffi::clutter_bind_constraint_get_offset(self.to_glib_none().0) }
    }

    /// Retrieves the `Actor` set using `BindConstraint::set_source`
    ///
    /// # Returns
    ///
    /// a pointer to the source actor
    pub fn get_source(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_bind_constraint_get_source(
                self.to_glib_none().0,
            ))
        }
    }

    /// Sets the coordinate to bind in the constraint
    /// ## `coordinate`
    /// the coordinate to bind
    pub fn set_coordinate(&self, coordinate: BindCoordinate) {
        unsafe {
            ffi::clutter_bind_constraint_set_coordinate(
                self.to_glib_none().0,
                coordinate.to_glib(),
            );
        }
    }

    /// Sets the offset to be applied to the constraint
    /// ## `offset`
    /// the offset to apply, in pixels
    pub fn set_offset(&self, offset: f32) {
        unsafe {
            ffi::clutter_bind_constraint_set_offset(self.to_glib_none().0, offset);
        }
    }

    /// Sets the source `Actor` for the constraint
    /// ## `source`
    /// a `Actor`, or `None` to unset the source
    pub fn set_source<P: IsA<Actor>>(&self, source: Option<&P>) {
        unsafe {
            ffi::clutter_bind_constraint_set_source(
                self.to_glib_none().0,
                source.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_coordinate_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_coordinate_trampoline<F: Fn(&BindConstraint) + 'static>(
            this: *mut ffi::ClutterBindConstraint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::coordinate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_coordinate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_offset_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_offset_trampoline<F: Fn(&BindConstraint) + 'static>(
            this: *mut ffi::ClutterBindConstraint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::offset\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_offset_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_source_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<F: Fn(&BindConstraint) + 'static>(
            this: *mut ffi::ClutterBindConstraint,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_source_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BindConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BindConstraint")
    }
}
