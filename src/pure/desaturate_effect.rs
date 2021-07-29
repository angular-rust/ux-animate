use super::{HandlerId, OffscreenEffect};
use std::fmt;

// SECTION:clutter-desaturate-effect
// @short_description: A desaturation effect
// @see_also: #Effect, #OffscreenEffect
//
// #DesaturateEffect is a sub-class of #Effect that
// desaturates the color of an actor and its contents. The strenght
// of the desaturation effect is controllable and animatable through
// the #DesaturateEffect:factor property.

// @extends OffscreenEffect, Effect, ActorMeta
pub struct DesaturateEffect {
    parent_instance: OffscreenEffect,

    /* the desaturation factor, also known as "strength" */
    factor: f64,

    factor_uniform: i32,

    tex_width: i32,
    tex_height: i32,

    pipeline: Option<dx::core::Pipeline>,
}

impl DesaturateEffect {
    /// Creates a new `DesaturateEffect` to be used with
    /// `ActorExt::add_effect`
    /// ## `factor`
    /// the desaturation factor, between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the newly created `DesaturateEffect` or `None`
    pub fn new(factor: f64) -> DesaturateEffect {
        unimplemented!()
    }

    /// Retrieves the desaturation factor of `self`
    ///
    /// # Returns
    ///
    /// the desaturation factor
    pub fn get_factor(&self) -> f64 {
        unimplemented!()
    }

    /// Sets the desaturation factor for `self`, with 0.0 being "do not desaturate"
    /// and 1.0 being "fully desaturate"
    /// ## `factor`
    /// the desaturation factor, between 0.0 and 1.0
    pub fn set_factor(&self, factor: f64) {
        unimplemented!()
    }

    pub fn connect_property_factor_notify<F: Fn(&DesaturateEffect) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DesaturateEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DesaturateEffect")
    }
}
