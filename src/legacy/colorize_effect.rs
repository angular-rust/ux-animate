use crate::{ActorMeta, Color, Effect, InternalColor, OffscreenEffect, RgbaColor};
use glib::{
    object as gobject,
    object::{Cast, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct ColorizeEffect(Object<ffi::ClutterColorizeEffect, ffi::ClutterColorizeEffectClass, ColorizeEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_colorize_effect_get_type(),
    }
}

impl ColorizeEffect {
    /// Creates a new `ColorizeEffect` to be used with
    /// `ActorExt::add_effect`
    /// ## `tint`
    /// the color to be used
    ///
    /// # Returns
    ///
    /// the newly created `ColorizeEffect` or `None`
    pub fn new(tint: Color) -> ColorizeEffect {
        let RgbaColor {
            red,
            green,
            blue,
            alpha,
        } = tint.into();
        let tint = InternalColor::new(red, green, blue, alpha);

        unsafe {
            Effect::from_glib_none(ffi::clutter_colorize_effect_new(tint.to_glib_none().0))
                .unsafe_cast()
        }
    }

    /// Retrieves the tint used by `self`
    /// ## `tint`
    /// return location for the color used
    pub fn get_tint(&self) -> InternalColor {
        unsafe {
            let mut tint = InternalColor::uninitialized();
            ffi::clutter_colorize_effect_get_tint(self.to_glib_none().0, tint.to_glib_none_mut().0);
            tint
        }
    }

    /// Sets the tint to be used when colorizing
    /// ## `tint`
    /// the color to be used
    pub fn set_tint(&self, tint: &InternalColor) {
        unsafe {
            ffi::clutter_colorize_effect_set_tint(self.to_glib_none().0, tint.to_glib_none().0);
        }
    }

    pub fn connect_property_tint_notify<F: Fn(&ColorizeEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tint_trampoline<F: Fn(&ColorizeEffect) + 'static>(
            this: *mut ffi::ClutterColorizeEffect,
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
                b"notify::tint\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tint_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ColorizeEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorizeEffect")
    }
}
