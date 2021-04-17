use crate::{ActorMeta, Effect, OffscreenEffect};
use glib::{
    object as gobject,
    object::{Cast, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct DesaturateEffect(Object<ffi::ClutterDesaturateEffect, ffi::ClutterDesaturateEffectClass, DesaturateEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_desaturate_effect_get_type(),
    }
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
        unsafe { Effect::from_glib_none(ffi::clutter_desaturate_effect_new(factor)).unsafe_cast() }
    }

    /// Retrieves the desaturation factor of `self`
    ///
    /// # Returns
    ///
    /// the desaturation factor
    pub fn get_factor(&self) -> f64 {
        unsafe { ffi::clutter_desaturate_effect_get_factor(self.to_glib_none().0) }
    }

    /// Sets the desaturation factor for `self`, with 0.0 being "do not desaturate"
    /// and 1.0 being "fully desaturate"
    /// ## `factor`
    /// the desaturation factor, between 0.0 and 1.0
    pub fn set_factor(&self, factor: f64) {
        unsafe {
            ffi::clutter_desaturate_effect_set_factor(self.to_glib_none().0, factor);
        }
    }

    pub fn connect_property_factor_notify<F: Fn(&DesaturateEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_factor_trampoline<F: Fn(&DesaturateEffect) + 'static>(
            this: *mut ffi::ClutterDesaturateEffect,
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
}

impl fmt::Display for DesaturateEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DesaturateEffect")
    }
}
