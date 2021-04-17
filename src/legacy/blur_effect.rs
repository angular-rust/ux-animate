use crate::{ActorMeta, Effect, OffscreenEffect};
use glib::{object as gobject, object::Cast, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct BlurEffect(Object<ffi::ClutterBlurEffect, ffi::ClutterBlurEffectClass, BlurEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_blur_effect_get_type(),
    }
}

impl BlurEffect {
    /// Creates a new `BlurEffect` to be used with
    /// `ActorExt::add_effect`
    ///
    /// # Returns
    ///
    /// the newly created `BlurEffect` or `None`
    pub fn new() -> BlurEffect {
        unsafe { Effect::from_glib_none(ffi::clutter_blur_effect_new()).unsafe_cast() }
    }
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BlurEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlurEffect")
    }
}
