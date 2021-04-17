use crate::{ActorMeta, Effect, OffscreenEffect};
use glib::{
    object as gobject,
    object::{Cast, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{fmt, mem};

glib_wrapper! {
    pub struct BrightnessContrastEffect(Object<ffi::ClutterBrightnessContrastEffect, ffi::ClutterBrightnessContrastEffectClass, BrightnessContrastEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_brightness_contrast_effect_get_type(),
    }
}

impl BrightnessContrastEffect {
    /// Creates a new `BrightnessContrastEffect` to be used with
    /// `ActorExt::add_effect`
    ///
    /// # Returns
    ///
    /// the newly created
    ///  `BrightnessContrastEffect` or `None`. Use `gobject::ObjectExt::unref` when
    ///  done.
    pub fn new() -> BrightnessContrastEffect {
        unsafe {
            Effect::from_glib_full(ffi::clutter_brightness_contrast_effect_new()).unsafe_cast()
        }
    }

    /// Retrieves the change in brightness used by `self`.
    /// ## `red`
    /// return location for red component of the
    ///  change in brightness
    /// ## `green`
    /// return location for green component of the
    ///  change in brightness
    /// ## `blue`
    /// return location for blue component of the
    ///  change in brightness
    pub fn get_brightness(&self) -> (f32, f32, f32) {
        unsafe {
            let mut red = mem::MaybeUninit::uninit();
            let mut green = mem::MaybeUninit::uninit();
            let mut blue = mem::MaybeUninit::uninit();
            ffi::clutter_brightness_contrast_effect_get_brightness(
                self.to_glib_none().0,
                red.as_mut_ptr(),
                green.as_mut_ptr(),
                blue.as_mut_ptr(),
            );
            let red = red.assume_init();
            let green = green.assume_init();
            let blue = blue.assume_init();
            (red, green, blue)
        }
    }

    /// Retrieves the contrast value used by `self`.
    /// ## `red`
    /// return location for red component of the
    ///  change in contrast
    /// ## `green`
    /// return location for green component of the
    ///  change in contrast
    /// ## `blue`
    /// return location for blue component of the
    ///  change in contrast
    pub fn get_contrast(&self) -> (f32, f32, f32) {
        unsafe {
            let mut red = mem::MaybeUninit::uninit();
            let mut green = mem::MaybeUninit::uninit();
            let mut blue = mem::MaybeUninit::uninit();
            ffi::clutter_brightness_contrast_effect_get_contrast(
                self.to_glib_none().0,
                red.as_mut_ptr(),
                green.as_mut_ptr(),
                blue.as_mut_ptr(),
            );
            let red = red.assume_init();
            let green = green.assume_init();
            let blue = blue.assume_init();
            (red, green, blue)
        }
    }

    /// The range of `brightness` is [-1.0, 1.0], where 0.0 designates no change;
    /// a value below 0.0 indicates a decrease in brightness; and a value
    /// above 0.0 indicates an increase of brightness.
    /// ## `brightness`
    /// the brightness change for all three components (r, g, b)
    pub fn set_brightness(&self, brightness: f32) {
        unsafe {
            ffi::clutter_brightness_contrast_effect_set_brightness(
                self.to_glib_none().0,
                brightness,
            );
        }
    }

    /// The range for each component is [-1.0, 1.0] where 0.0 designates no change,
    /// values below 0.0 mean a decrease in brightness, and values above indicate
    /// an increase.
    /// ## `red`
    /// red component of the change in brightness
    /// ## `green`
    /// green component of the change in brightness
    /// ## `blue`
    /// blue component of the change in brightness
    pub fn set_brightness_full(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            ffi::clutter_brightness_contrast_effect_set_brightness_full(
                self.to_glib_none().0,
                red,
                green,
                blue,
            );
        }
    }

    /// The range for `contrast` is [-1.0, 1.0], where 0.0 designates no change;
    /// a value below 0.0 indicates a decrease in contrast; and a value above
    /// 0.0 indicates an increase.
    /// ## `contrast`
    /// contrast change for all three channels
    pub fn set_contrast(&self, contrast: f32) {
        unsafe {
            ffi::clutter_brightness_contrast_effect_set_contrast(self.to_glib_none().0, contrast);
        }
    }

    /// The range for each component is [-1.0, 1.0] where 0.0 designates no change,
    /// values below 0.0 mean a decrease in contrast, and values above indicate
    /// an increase.
    /// ## `red`
    /// red component of the change in contrast
    /// ## `green`
    /// green component of the change in contrast
    /// ## `blue`
    /// blue component of the change in contrast
    pub fn set_contrast_full(&self, red: f32, green: f32, blue: f32) {
        unsafe {
            ffi::clutter_brightness_contrast_effect_set_contrast_full(
                self.to_glib_none().0,
                red,
                green,
                blue,
            );
        }
    }

    pub fn connect_property_brightness_notify<F: Fn(&BrightnessContrastEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_brightness_trampoline<
            F: Fn(&BrightnessContrastEffect) + 'static,
        >(
            this: *mut ffi::ClutterBrightnessContrastEffect,
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
                b"notify::brightness\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_brightness_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_contrast_notify<F: Fn(&BrightnessContrastEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_contrast_trampoline<
            F: Fn(&BrightnessContrastEffect) + 'static,
        >(
            this: *mut ffi::ClutterBrightnessContrastEffect,
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
                b"notify::contrast\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_contrast_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for BrightnessContrastEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BrightnessContrastEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BrightnessContrastEffect")
    }
}
