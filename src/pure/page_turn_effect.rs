use crate::prelude::*;
use super::{ActorMeta, DeformEffect, Effect, OffscreenEffect};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// @extends DeformEffect, OffscreenEffect, Effect, ActorMeta
#[derive(Debug, Clone)]
pub struct PageTurnEffect{
}

impl PageTurnEffect {
    /// Creates a new `PageTurnEffect` instance with the given parameters
    /// ## `period`
    /// the period of the page curl, between 0.0 and 1.0
    /// ## `angle`
    /// the angle of the page curl, between 0.0 and 360.0
    /// ## `radius`
    /// the radius of the page curl, in pixels
    ///
    /// # Returns
    ///
    /// the newly created `PageTurnEffect`
    pub fn new(period: f64, angle: f64, radius: f32) -> PageTurnEffect {
        // unsafe {
        //     Effect::from_glib_none(ffi::clutter_page_turn_effect_new(period, angle, radius))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }

    /// Retrieves the value set using `PageTurnEffect::get_angle`
    ///
    /// # Returns
    ///
    /// the angle of the page curling
    pub fn get_angle(&self) -> f64 {
        // unsafe { ffi::clutter_page_turn_effect_get_angle(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves the value set using `PageTurnEffect::get_period`
    ///
    /// # Returns
    ///
    /// the period of the page curling
    pub fn get_period(&self) -> f64 {
        // unsafe { ffi::clutter_page_turn_effect_get_period(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves the value set using `PageTurnEffect::set_radius`
    ///
    /// # Returns
    ///
    /// the radius of the page curling
    pub fn get_radius(&self) -> f32 {
        // unsafe { ffi::clutter_page_turn_effect_get_radius(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Sets the angle of the page curling, in degrees
    /// ## `angle`
    /// the angle of the page curl, in degrees
    pub fn set_angle(&self, angle: f64) {
        // unsafe {
        //     ffi::clutter_page_turn_effect_set_angle(self.to_glib_none().0, angle);
        // }
        unimplemented!()
    }

    /// Sets the period of the page curling, between 0.0 (no curling)
    /// and 1.0 (fully curled)
    /// ## `period`
    /// the period of the page curl, between 0.0 and 1.0
    pub fn set_period(&self, period: f64) {
        // unsafe {
        //     ffi::clutter_page_turn_effect_set_period(self.to_glib_none().0, period);
        // }
        unimplemented!()
    }

    /// Sets the radius of the page curling
    /// ## `radius`
    /// the radius of the page curling, in pixels
    pub fn set_radius(&self, radius: f32) {
        // unsafe {
        //     ffi::clutter_page_turn_effect_set_radius(self.to_glib_none().0, radius);
        // }
        unimplemented!()
    }

    pub fn connect_property_angle_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    pub fn connect_property_period_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    pub fn connect_property_radius_notify<F: Fn(&PageTurnEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for PageTurnEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PageTurnEffect")
    }
}
