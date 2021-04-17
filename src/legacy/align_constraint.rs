use crate::{Actor, ActorMeta, AlignAxis, Constraint};
use glib::{
    object as gobject,
    object::{Cast, IsA, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct AlignConstraint(Object<ffi::ClutterAlignConstraint, ffi::ClutterAlignConstraintClass, AlignConstraintClass>) @extends Constraint, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_align_constraint_get_type(),
    }
}

impl AlignConstraint {
    /// Creates a new constraint, aligning a `Actor`'s position with
    /// regards of the size of the actor to `source`, with the given
    /// alignment `factor`
    /// ## `source`
    /// the `Actor` to use as the source of the
    ///  alignment, or `None`
    /// ## `axis`
    /// the axis to be used to compute the alignment
    /// ## `factor`
    /// the alignment factor, between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the newly created `AlignConstraint`
    pub fn new<P: IsA<Actor>>(source: Option<&P>, axis: AlignAxis, factor: f32) -> AlignConstraint {
        unsafe {
            Constraint::from_glib_none(ffi::clutter_align_constraint_new(
                source.map(|p| p.as_ref()).to_glib_none().0,
                axis.to_glib(),
                factor,
            ))
            .unsafe_cast()
        }
    }

    /// Retrieves the value set using `AlignConstraint::set_align_axis`
    ///
    /// # Returns
    ///
    /// the alignment axis
    pub fn get_align_axis(&self) -> AlignAxis {
        unsafe {
            from_glib(ffi::clutter_align_constraint_get_align_axis(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the factor set using `AlignConstraint::set_factor`
    ///
    /// # Returns
    ///
    /// the alignment factor
    pub fn get_factor(&self) -> f32 {
        unsafe { ffi::clutter_align_constraint_get_factor(self.to_glib_none().0) }
    }

    /// Retrieves the source of the alignment
    ///
    /// # Returns
    ///
    /// the `Actor` used as the source
    ///  of the alignment
    pub fn get_source(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_align_constraint_get_source(
                self.to_glib_none().0,
            ))
        }
    }

    /// Sets the axis to which the alignment refers to
    /// ## `axis`
    /// the axis to which the alignment refers to
    pub fn set_align_axis(&self, axis: AlignAxis) {
        unsafe {
            ffi::clutter_align_constraint_set_align_axis(self.to_glib_none().0, axis.to_glib());
        }
    }

    /// Sets the alignment factor of the constraint
    ///
    /// The factor depends on the `AlignConstraint:align-axis` property
    /// and it is a value between 0.0 (meaning left, when
    /// `AlignConstraint:align-axis` is set to `AlignAxis::XAxis`; or
    /// meaning top, when `AlignConstraint:align-axis` is set to
    /// `AlignAxis::YAxis`) and 1.0 (meaning right, when
    /// `AlignConstraint:align-axis` is set to `AlignAxis::XAxis`; or
    /// meaning bottom, when `AlignConstraint:align-axis` is set to
    /// `AlignAxis::YAxis`). A value of 0.5 aligns in the middle in either
    /// cases
    /// ## `factor`
    /// the alignment factor, between 0.0 and 1.0
    pub fn set_factor(&self, factor: f32) {
        unsafe {
            ffi::clutter_align_constraint_set_factor(self.to_glib_none().0, factor);
        }
    }

    /// Sets the source of the alignment constraint
    /// ## `source`
    /// a `Actor`, or `None` to unset the source
    pub fn set_source<P: IsA<Actor>>(&self, source: Option<&P>) {
        unsafe {
            ffi::clutter_align_constraint_set_source(
                self.to_glib_none().0,
                source.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn connect_property_align_axis_notify<F: Fn(&AlignConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_align_axis_trampoline<F: Fn(&AlignConstraint) + 'static>(
            this: *mut ffi::ClutterAlignConstraint,
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
                b"notify::align-axis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_align_axis_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_factor_notify<F: Fn(&AlignConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_factor_trampoline<F: Fn(&AlignConstraint) + 'static>(
            this: *mut ffi::ClutterAlignConstraint,
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
                b"notify::factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_source_notify<F: Fn(&AlignConstraint) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<F: Fn(&AlignConstraint) + 'static>(
            this: *mut ffi::ClutterAlignConstraint,
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

impl fmt::Display for AlignConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AlignConstraint")
    }
}
