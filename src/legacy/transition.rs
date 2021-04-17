// Scriptable
use super::{Animatable, Interval, Timeline};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// TODO: @implements Scriptable
glib_wrapper! {
    pub struct Transition(Object<ffi::ClutterTransition, ffi::ClutterTransitionClass, TransitionClass>) @extends Timeline;

    match fn {
        get_type => || ffi::clutter_transition_get_type(),
    }
}

/// Trait containing all `Transition` methods.
///
/// # Implementors
///
/// [`PropertyTransition`](struct.PropertyTransition.html), [`TransitionGroup`](struct.TransitionGroup.html), [`Transition`](struct.Transition.html)
pub trait TransitionExt: 'static {
    /// Retrieves the `Animatable` set using `TransitionExt::set_animatable`.
    ///
    /// # Returns
    ///
    /// a `Animatable`, or `None`; the returned
    ///  animatable is owned by the `Transition`, and it should not be freed
    ///  directly.
    fn get_animatable(&self) -> Option<Animatable>;

    /// Retrieves the interval set using `TransitionExt::set_interval`
    ///
    /// # Returns
    ///
    /// a `Interval`, or `None`; the returned
    ///  interval is owned by the `Transition` and it should not be freed
    ///  directly
    fn get_interval(&self) -> Option<Interval>;

    /// Retrieves the value of the `Transition:remove-on-complete` property.
    ///
    /// # Returns
    ///
    /// `true` if the `self` should be detached when complete,
    ///  and `false` otherwise
    fn get_remove_on_complete(&self) -> bool;

    /// Sets the `Transition:animatable` property.
    ///
    /// The `self` will acquire a reference to the `animatable` instance,
    /// and will call the `TransitionClass.attached`() virtual function.
    ///
    /// If an existing `Animatable` is attached to `self`, the
    /// reference will be released, and the `TransitionClass.detached`()
    /// virtual function will be called.
    /// ## `animatable`
    /// a `Animatable`, or `None`
    fn set_animatable<P: IsA<Animatable>>(&self, animatable: Option<&P>);

    //fn set_from(&self, value_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets the initial value of the transition.
    ///
    /// This is a convenience function that will either create the
    /// `Interval` used by `self`, or will update it if
    /// the `Transition:interval` is already set.
    ///
    /// This function will copy the contents of `value`, so it is
    /// safe to call `gobject::Value::unset` after it returns.
    ///
    /// If `self` already has a `Transition:interval` set,
    /// then `value` must hold the same type, or a transformable type,
    /// as the interval's `Interval:value-type` property.
    ///
    /// This function is meant to be used by language bindings.
    /// ## `value`
    /// a `gobject::Value` with the initial value of the transition
    fn set_from_value(&self, value: &glib::Value);

    /// Sets the `Transition:interval` property using `interval`.
    ///
    /// The `self` will acquire a reference on the `interval`, sinking
    /// the floating flag on it if necessary.
    /// ## `interval`
    /// a `Interval`, or `None`
    fn set_interval<P: IsA<Interval>>(&self, interval: Option<&P>);

    /// Sets whether `self` should be detached from the `Animatable`
    /// set using `TransitionExt::set_animatable` when the
    /// `Timeline::completed` signal is emitted.
    /// ## `remove_complete`
    /// whether to detach `self` when complete
    fn set_remove_on_complete(&self, remove_complete: bool);

    //fn set_to(&self, value_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets the final value of the transition.
    ///
    /// This is a convenience function that will either create the
    /// `Interval` used by `self`, or will update it if
    /// the `Transition:interval` is already set.
    ///
    /// This function will copy the contents of `value`, so it is
    /// safe to call `gobject::Value::unset` after it returns.
    ///
    /// If `self` already has a `Transition:interval` set,
    /// then `value` must hold the same type, or a transformable type,
    /// as the interval's `Interval:value-type` property.
    ///
    /// This function is meant to be used by language bindings.
    /// ## `value`
    /// a `gobject::Value` with the final value of the transition
    fn set_to_value(&self, value: &glib::Value);

    fn connect_property_animatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_remove_on_complete_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Transition>> TransitionExt for O {
    fn get_animatable(&self) -> Option<Animatable> {
        unsafe {
            from_glib_none(ffi::clutter_transition_get_animatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_interval(&self) -> Option<Interval> {
        unsafe {
            from_glib_none(ffi::clutter_transition_get_interval(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_remove_on_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_transition_get_remove_on_complete(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_animatable<P: IsA<Animatable>>(&self, animatable: Option<&P>) {
        unsafe {
            ffi::clutter_transition_set_animatable(
                self.as_ref().to_glib_none().0,
                animatable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    //fn set_from(&self, value_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_transition_set_from() }
    //}

    fn set_from_value(&self, value: &glib::Value) {
        unsafe {
            ffi::clutter_transition_set_from_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn set_interval<P: IsA<Interval>>(&self, interval: Option<&P>) {
        unsafe {
            ffi::clutter_transition_set_interval(
                self.as_ref().to_glib_none().0,
                interval.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_remove_on_complete(&self, remove_complete: bool) {
        unsafe {
            ffi::clutter_transition_set_remove_on_complete(
                self.as_ref().to_glib_none().0,
                remove_complete.to_glib(),
            );
        }
    }

    //fn set_to(&self, value_type: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_transition_set_to() }
    //}

    fn set_to_value(&self, value: &glib::Value) {
        unsafe {
            ffi::clutter_transition_set_to_value(
                self.as_ref().to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn connect_property_animatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_animatable_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTransition,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Transition>,
        {
            let f: &F = &*(f as *const F);
            f(&Transition::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::animatable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_animatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interval_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTransition,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Transition>,
        {
            let f: &F = &*(f as *const F);
            f(&Transition::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interval\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_remove_on_complete_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_remove_on_complete_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTransition,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Transition>,
        {
            let f: &F = &*(f as *const F);
            f(&Transition::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::remove-on-complete\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_remove_on_complete_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transition")
    }
}
