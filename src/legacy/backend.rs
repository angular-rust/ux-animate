use glib::{
    object::ObjectType as ObjectType_,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct Backend(Object<ffi::ClutterBackend, ffi::ClutterBackendClass, BackendClass>);

    match fn {
        get_type => || ffi::clutter_backend_get_type(),
    }
}

impl Backend {
    /// Retrieves the font options for `self`.
    ///
    /// # Returns
    ///
    /// the font options of the `Backend`.
    ///  The returned `cairo::FontOptions` is owned by the backend and should
    ///  not be modified or freed
    pub fn get_font_options(&self) -> Option<cairo::FontOptions> {
        unsafe { from_glib_none(ffi::clutter_backend_get_font_options(self.to_glib_none().0)) }
    }

    /// Gets the resolution for font handling on the screen.
    ///
    /// The resolution is a scale factor between points specified in a
    /// `pango::FontDescription` and cairo units. The default value is 96.0,
    /// meaning that a 10 point font will be 13 units
    /// high (10 * 96. / 72. = 13.3).
    ///
    /// will set the resolution using the current backend when
    /// initializing; the resolution is also stored in the
    /// `Settings:font-dpi` property.
    ///
    /// # Returns
    ///
    /// the current resolution, or -1 if no resolution
    ///  has been set.
    pub fn get_resolution(&self) -> f64 {
        unsafe { ffi::clutter_backend_get_resolution(self.to_glib_none().0) }
    }

    /// Sets the new font options for `self`. The `Backend` will
    /// copy the `cairo::FontOptions`.
    ///
    /// If `options` is `None`, the first following call to
    /// `Backend::get_font_options` will return the default font
    /// options for `self`.
    ///
    /// This function is intended for actors creating a Pango layout
    /// using the PangoCairo API.
    /// ## `options`
    /// Cairo font options for the backend, or `None`
    pub fn set_font_options(&self, options: &cairo::FontOptions) {
        unsafe {
            ffi::clutter_backend_set_font_options(self.to_glib_none().0, options.to_glib_none().0);
        }
    }

    /// The ::font-changed signal is emitted each time the font options
    /// have been changed through `Settings`.
    pub fn connect_font_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn font_changed_trampoline<F: Fn(&Backend) + 'static>(
            this: *mut ffi::ClutterBackend,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"font-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    font_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// The ::resolution-changed signal is emitted each time the font
    /// resolutions has been changed through `Settings`.
    pub fn connect_resolution_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resolution_changed_trampoline<F: Fn(&Backend) + 'static>(
            this: *mut ffi::ClutterBackend,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resolution-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resolution_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// The ::settings-changed signal is emitted each time the `Settings`
    /// properties have been changed.
    pub fn connect_settings_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn settings_changed_trampoline<F: Fn(&Backend) + 'static>(
            this: *mut ffi::ClutterBackend,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"settings-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    settings_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backend")
    }
}
