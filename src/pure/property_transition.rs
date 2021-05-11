use crate::prelude::*;
use super::{Timeline, Transition};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// TODO: @implements Scriptable
// @extends Transition, Timeline
#[derive(Debug, Clone)]
pub struct PropertyTransition {
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
        // unsafe {
        //     Transition::from_glib_full(ffi::clutter_property_transition_new(
        //         property_name.to_glib_none().0,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Object for PropertyTransition {}
impl Is<PropertyTransition> for PropertyTransition {}

impl AsRef<PropertyTransition> for PropertyTransition {
    fn as_ref(&self) -> &PropertyTransition {
        self
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
    fn get_property_name(&self) -> Option<String>;

    /// Sets the `PropertyTransition:property-name` property of `self`.
    /// ## `property_name`
    /// a property name
    fn set_property_name(&self, property_name: Option<&str>);

    fn connect_property_property_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<PropertyTransition>> PropertyTransitionExt for O {
    fn get_property_name(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::clutter_property_transition_get_property_name(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_property_name(&self, property_name: Option<&str>) {
        // unsafe {
        //     ffi::clutter_property_transition_set_property_name(
        //         self.as_ref().to_glib_none().0,
        //         property_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_property_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for PropertyTransition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyTransition")
    }
}
