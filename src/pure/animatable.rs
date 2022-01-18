#![allow(unused_imports)]
use crate::prelude::*;
use super::Interval;
use std::fmt;

// AnimatableIface:
// @animate_property: virtual function for custom interpolation of a
//   property. This virtual function is deprecated
// @find_property: virtual function for retrieving the #GParamSpec of
//   an animatable property
// @get_initial_state: virtual function for retrieving the initial
//   state of an animatable property
// @set_final_state: virtual function for setting the state of an
//   animatable property
// @interpolate_value: virtual function for interpolating the progress
//   of a property
//
// Base interface for #GObject<!-- -->s that can be animated by a
// a #Animation.
// @short_description: Interface for animatable classes
//
// #Animatable is an interface that allows a #GObject class
// to control how a #Animation will animate a property.
//
// Each #Animatable should implement the
// #AnimatableIface.interpolate_property() virtual function of the
// interface to compute the animation state between two values of an interval
// depending on a progress factor, expressed as a floating point value.
//
// If a #Animatable is animated by a #Animation
// instance, the #Animation will call
// animatable_interpolate_property() passing the name of the
// currently animated property; the values interval; and the progress factor.
// The #Animatable implementation should return the computed value for
// the animated
// property.

// Animatable ------------- IS INTERFACE ----------------
#[derive(Debug, Clone)]
pub struct Animatable {

}

impl Object for Animatable {}
impl Is<Animatable> for Animatable {}

impl AsRef<Animatable> for Animatable {
    fn as_ref(&self) -> &Animatable {
        self
    }
}

/// Trait containing all `Animatable` methods.
///
/// # Implementors
///
/// [`Actor`](struct.Actor.html), [`Animatable`](struct.Animatable.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Group`](struct.Group.html), [`Rectangle`](struct.Rectangle.html), [`ScrollActor`](struct.ScrollActor.html), [`Stage`](struct.Stage.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html)
pub trait AnimatableExt: 'static {
    // /// Finds the `gobject::ParamSpec` for `property_name`
    // /// ## `property_name`
    // /// the name of the animatable property to find
    // ///
    // /// # Returns
    // ///
    // /// The `gobject::ParamSpec` for the given property
    // ///  or `None`
    // fn find_property(&self, property_name: &str) -> Option<glib::ParamSpec>;

    // /// Retrieves the current state of `property_name` and sets `value` with it
    // /// ## `property_name`
    // /// the name of the animatable property to retrieve
    // /// ## `value`
    // /// a `gobject::Value` initialized to the type of the property to retrieve
    // fn get_initial_state(&self, property_name: &str, value: &mut glib::Value);

    // /// Asks a `Animatable` implementation to interpolate a
    // /// a named property between the initial and final values of
    // /// a `Interval`, using `progress` as the interpolation
    // /// value, and store the result inside `value`.
    // ///
    // /// This function should be used for every property animation
    // /// involving `Animatable`<!-- -->s.
    // ///
    // /// This function replaces `Animatable::animate_property`.
    // /// ## `property_name`
    // /// the name of the property to interpolate
    // /// ## `interval`
    // /// a `Interval` with the animation range
    // /// ## `progress`
    // /// the progress to use to interpolate between the
    // ///  initial and final values of the `interval`
    // /// ## `value`
    // /// return location for an initialized `gobject::Value`
    // ///  using the same type of the `interval`
    // ///
    // /// # Returns
    // ///
    // /// `true` if the interpolation was successful,
    // ///  and `false` otherwise
    // fn interpolate_value<P: Is<Interval>>(
    //     &self,
    //     property_name: &str,
    //     interval: &P,
    //     progress: f64,
    // ) -> Option<glib::Value>;

    // /// Sets the current state of `property_name` to `value`
    // /// ## `property_name`
    // /// the name of the animatable property to set
    // /// ## `value`
    // /// the value of the animatable property to set
    // fn set_final_state(&self, property_name: &str, value: &glib::Value);
}

impl<O: Is<Animatable>> AnimatableExt for O {
    // fn find_property(&self, property_name: &str) -> Option<glib::ParamSpec> {
    //     unimplemented!()
    // }

    // fn get_initial_state(&self, property_name: &str, value: &mut glib::Value) {
    //     unimplemented!()
    // }

    // fn interpolate_value<P: Is<Interval>>(
    //     &self,
    //     property_name: &str,
    //     interval: &P,
    //     progress: f64,
    // ) -> Option<glib::Value> {
    //     unimplemented!()
    // }

    // fn set_final_state(&self, property_name: &str, value: &glib::Value) {
    //     unimplemented!()
    // }
}

impl fmt::Display for Animatable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Animatable")
    }
}
