use crate::Interval;
use glib::{object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct Animatable(Interface<ffi::ClutterAnimatable>);

    match fn {
        get_type => || ffi::clutter_animatable_get_type(),
    }
}

/// Trait containing all `Animatable` methods.
///
/// # Implementors
///
/// [`Actor`](struct.Actor.html), [`Animatable`](struct.Animatable.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Group`](struct.Group.html), [`Rectangle`](struct.Rectangle.html), [`ScrollActor`](struct.ScrollActor.html), [`Stage`](struct.Stage.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html)
pub trait AnimatableExt: 'static {
    /// Finds the `gobject::ParamSpec` for `property_name`
    /// ## `property_name`
    /// the name of the animatable property to find
    ///
    /// # Returns
    ///
    /// The `gobject::ParamSpec` for the given property
    ///  or `None`
    fn find_property(&self, property_name: &str) -> Option<glib::ParamSpec>;

    /// Retrieves the current state of `property_name` and sets `value` with it
    /// ## `property_name`
    /// the name of the animatable property to retrieve
    /// ## `value`
    /// a `gobject::Value` initialized to the type of the property to retrieve
    fn get_initial_state(&self, property_name: &str, value: &mut glib::Value);

    /// Asks a `Animatable` implementation to interpolate a
    /// a named property between the initial and final values of
    /// a `Interval`, using `progress` as the interpolation
    /// value, and store the result inside `value`.
    ///
    /// This function should be used for every property animation
    /// involving `Animatable`<!-- -->s.
    ///
    /// This function replaces `Animatable::animate_property`.
    /// ## `property_name`
    /// the name of the property to interpolate
    /// ## `interval`
    /// a `Interval` with the animation range
    /// ## `progress`
    /// the progress to use to interpolate between the
    ///  initial and final values of the `interval`
    /// ## `value`
    /// return location for an initialized `gobject::Value`
    ///  using the same type of the `interval`
    ///
    /// # Returns
    ///
    /// `true` if the interpolation was successful,
    ///  and `false` otherwise
    fn interpolate_value<P: IsA<Interval>>(
        &self,
        property_name: &str,
        interval: &P,
        progress: f64,
    ) -> Option<glib::Value>;

    /// Sets the current state of `property_name` to `value`
    /// ## `property_name`
    /// the name of the animatable property to set
    /// ## `value`
    /// the value of the animatable property to set
    fn set_final_state(&self, property_name: &str, value: &glib::Value);
}

impl<O: IsA<Animatable>> AnimatableExt for O {
    fn find_property(&self, property_name: &str) -> Option<glib::ParamSpec> {
        unsafe {
            from_glib_none(ffi::clutter_animatable_find_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            ))
        }
    }

    fn get_initial_state(&self, property_name: &str, value: &mut glib::Value) {
        unsafe {
            ffi::clutter_animatable_get_initial_state(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
    }

    fn interpolate_value<P: IsA<Interval>>(
        &self,
        property_name: &str,
        interval: &P,
        progress: f64,
    ) -> Option<glib::Value> {
        unsafe {
            let mut value = glib::Value::uninitialized();
            let ret = from_glib(ffi::clutter_animatable_interpolate_value(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                interval.as_ref().to_glib_none().0,
                progress,
                value.to_glib_none_mut().0,
            ));
            if ret {
                Some(value)
            } else {
                None
            }
        }
    }

    fn set_final_state(&self, property_name: &str, value: &glib::Value) {
        unsafe {
            ffi::clutter_animatable_set_final_state(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for Animatable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Animatable")
    }
}
