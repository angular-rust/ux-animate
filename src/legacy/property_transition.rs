// Scriptable
use crate::{Timeline, Transition};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// TODO: @implements Scriptable
glib_wrapper! {
    pub struct PropertyTransition(Object<ffi::ClutterPropertyTransition, ffi::ClutterPropertyTransitionClass, PropertyTransitionClass>) @extends Transition, Timeline;

    match fn {
        get_type => || ffi::clutter_property_transition_get_type(),
    }
}

impl PropertyTransition {
    /// Creates a new `PropertyTransition`.
    /// ## `property_name`
    /// a property of `animatable`, or `None`
    ///
    /// # Returns
    ///
    /// the newly created `PropertyTransition`.
    ///  Use `gobject::ObjectExt::unref` when done
    pub fn new(property_name: Option<&str>) -> PropertyTransition {
        unsafe {
            Transition::from_glib_full(ffi::clutter_property_transition_new(
                property_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

/// Trait containing all `PropertyTransition` methods.
///
/// # Implementors
///
/// [`KeyframeTransition`](struct.KeyframeTransition.html), [`PropertyTransition`](struct.PropertyTransition.html)
pub trait PropertyTransitionExt: 'static {
    /// Retrieves the value of the `PropertyTransition:property-name`
    /// property.
    ///
    /// # Returns
    ///
    /// the name of the property being animated, or `None` if
    ///  none is set. The returned string is owned by the `self` and
    ///  it should not be freed.
    fn get_property_name(&self) -> Option<GString>;

    /// Sets the `PropertyTransition:property-name` property of `self`.
    /// ## `property_name`
    /// a property name
    fn set_property_name(&self, property_name: Option<&str>);

    fn connect_property_property_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<PropertyTransition>> PropertyTransitionExt for O {
    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_property_transition_get_property_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_name(&self, property_name: Option<&str>) {
        unsafe {
            ffi::clutter_property_transition_set_property_name(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    fn connect_property_property_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_property_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterPropertyTransition,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PropertyTransition>,
        {
            let f: &F = &*(f as *const F);
            f(&PropertyTransition::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::property-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_property_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PropertyTransition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyTransition")
    }
}
