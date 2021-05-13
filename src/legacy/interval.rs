use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};
// StaticType, Value,
// use Scriptable;

// TODO: , @implements Scriptable
glib_wrapper! {
    pub struct Interval(Object<ffi::ClutterInterval, ffi::ClutterIntervalClass, IntervalClass>) @extends gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_interval_get_type(),
    }
}

impl Interval {
    //pub fn new(gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Interval {
    //    unsafe { TODO: call clutter_sys:clutter_interval_new() }
    //}

    pub fn with_values(
        gtype: glib::types::Type,
        initial: Option<&glib::Value>,
        final_: Option<&glib::Value>,
    ) -> Interval {
        unsafe {
            from_glib_none(ffi::clutter_interval_new_with_values(
                gtype.to_glib(),
                initial.to_glib_none().0,
                final_.to_glib_none().0,
            ))
        }
    }

    //pub fn register_progress_func<P: Fn(&glib::Value, &glib::Value, f64, &glib::Value) -> bool + 'static>(value_type: glib::types::Type, func: P) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_register_progress_func() }
    //}
}

/// Trait containing all `Interval` methods.
///
/// # Implementors
///
/// [`Interval`](struct.Interval.html)
pub trait IntervalExt: 'static {
    /// Creates a copy of `self`.
    ///
    /// # Returns
    ///
    /// the newly created `Interval`
    fn clone(&self) -> Option<Interval>;

    /// Computes the value between the `self` boundaries given the
    /// progress `factor`
    ///
    /// Unlike `IntervalExt::compute_value`, this function will
    /// return a const pointer to the computed value
    ///
    /// You should use this function if you immediately pass the computed
    /// value to another function that makes a copy of it, like
    /// `gobject::ObjectExt::set_property`
    /// ## `factor`
    /// the progress factor, between 0 and 1
    ///
    /// # Returns
    ///
    /// a pointer to the computed value,
    ///  or `None` if the computation was not successfull
    fn compute(&self, factor: f64) -> Option<glib::Value>;

    /// Computes the value between the `self` boundaries given the
    /// progress `factor` and copies it into `value`.
    /// ## `factor`
    /// the progress factor, between 0 and 1
    /// ## `value`
    /// return location for an initialized `gobject::Value`
    ///
    /// # Returns
    ///
    /// `true` if the operation was successful
    fn compute_value(&self, factor: f64) -> Option<glib::Value>;

    /// Retrieves the final value of `self` and copies
    /// it into `value`.
    ///
    /// The passed `gobject::Value` must be initialized to the value held by
    /// the `Interval`.
    /// ## `value`
    /// a `gobject::Value`
    fn get_final_value(&self) -> glib::Value;

    /// Retrieves the initial value of `self` and copies
    /// it into `value`.
    ///
    /// The passed `gobject::Value` must be initialized to the value held by
    /// the `Interval`.
    /// ## `value`
    /// a `gobject::Value`
    fn get_initial_value(&self) -> glib::Value;

    //fn get_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Retrieves the `glib::Type` of the values inside `self`.
    ///
    /// # Returns
    ///
    /// the type of the value, or G_TYPE_INVALID
    fn get_value_type(&self) -> glib::types::Type;

    /// Checks if the `self` has a valid initial and final values.
    ///
    /// # Returns
    ///
    /// `true` if the `Interval` has an initial and
    ///  final values, and `false` otherwise
    fn is_valid(&self) -> bool;

    /// Gets the pointer to the final value of `self`
    ///
    /// # Returns
    ///
    /// the final value of the interval.
    ///  The value is owned by the `Interval` and it should not be
    ///  modified or freed
    fn peek_final_value(&self) -> Option<glib::Value>;

    /// Gets the pointer to the initial value of `self`
    ///
    /// # Returns
    ///
    /// the initial value of the interval.
    ///  The value is owned by the `Interval` and it should not be
    ///  modified or freed
    fn peek_initial_value(&self) -> Option<glib::Value>;

    //fn set_final(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets the final value of `self` to `value`. The value is
    /// copied inside the `Interval`.
    /// ## `value`
    /// a `gobject::Value`
    fn set_final_value(&self, value: &glib::Value);

    //fn set_initial(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets the initial value of `self` to `value`. The value is copied
    /// inside the `Interval`.
    /// ## `value`
    /// a `gobject::Value`
    fn set_initial_value(&self, value: &glib::Value);

    //fn set_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    // /// Validates the initial and final values of `self` against
    // /// a `gobject::ParamSpec`.
    // /// ## `pspec`
    // /// a `gobject::ParamSpec`
    // ///
    // /// # Returns
    // ///
    // /// `true` if the `Interval` is valid, `false` otherwise
    // fn validate<P: IsA<glib::ParamSpec>>(&self, pspec: &P) -> bool;

    // /// The final value of the interval.
    // fn get_property_final(&self) -> Option<glib::Value>;

    // /// The initial value of the interval.
    // fn get_property_initial(&self) -> Option<glib::Value>;

    fn connect_property_final_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_initial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Interval>> IntervalExt for O {
    fn clone(&self) -> Option<Interval> {
        unsafe { from_glib_full(ffi::clutter_interval_clone(self.as_ref().to_glib_none().0)) }
    }

    fn compute(&self, factor: f64) -> Option<glib::Value> {
        unsafe {
            from_glib_none(ffi::clutter_interval_compute(
                self.as_ref().to_glib_none().0,
                factor,
            ))
        }
    }

    fn compute_value(&self, factor: f64) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::clutter_interval_compute_value(
                self.as_ref().to_glib_none().0,
                factor,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn get_final_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::clutter_interval_get_final_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    fn get_initial_value(&self) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::clutter_interval_get_initial_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    //fn get_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_get_interval() }
    //}

    fn get_value_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::clutter_interval_get_value_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_interval_is_valid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_final_value(&self) -> Option<glib::Value> {
        unsafe {
            from_glib_none(ffi::clutter_interval_peek_final_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_initial_value(&self) -> Option<glib::Value> {
        unsafe {
            from_glib_none(ffi::clutter_interval_peek_initial_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn set_final(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_final() }
    //}

    fn set_final_value(&self, value: &glib::Value) {
        unsafe {
            ffi::clutter_interval_set_final_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    //fn set_initial(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_initial() }
    //}

    fn set_initial_value(&self, value: &glib::Value) {
        unsafe {
            ffi::clutter_interval_set_initial_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    //fn set_interval(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_interval_set_interval() }
    //}

    // fn validate<P: IsA<glib::ParamSpec>>(&self, pspec: &P) -> bool {
    //     unsafe {
    //         from_glib(ffi::clutter_interval_validate(
    //             self.as_ref().to_glib_none().0,
    //             pspec.as_ref().to_glib_none().0,
    //         ))
    //     }
    // }

    // fn get_property_final(&self) -> Option<glib::Value> {
    //     unsafe {
    //         let mut value = Value::from_type(<glib::Value as StaticType>::static_type());
    //         gobject_sys::g_object_get_property(
    //             self.to_glib_none().0 as *mut gobject_sys::GObject,
    //             b"final\0".as_ptr() as *const _,
    //             value.to_glib_none_mut().0,
    //         );
    //         value
    //             .get()
    //             .expect("Return Value for property `final` getter")
    //     }
    // }

    // fn get_property_initial(&self) -> Option<glib::Value> {
    //     unsafe {
    //         let mut value = Value::from_type(<glib::Value as StaticType>::static_type());
    //         gobject_sys::g_object_get_property(
    //             self.to_glib_none().0 as *mut gobject_sys::GObject,
    //             b"initial\0".as_ptr() as *const _,
    //             value.to_glib_none_mut().0,
    //         );
    //         value
    //             .get()
    //             .expect("Return Value for property `initial` getter")
    //     }
    // }

    fn connect_property_final_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_final_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterInterval,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Interval>,
        {
            let f: &F = &*(f as *const F);
            f(&Interval::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::final\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_final_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_initial_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_initial_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterInterval,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Interval>,
        {
            let f: &F = &*(f as *const F);
            f(&Interval::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::initial\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_initial_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interval")
    }
}
