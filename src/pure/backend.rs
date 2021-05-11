use glib::{
    object::ObjectType as ObjectType_,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// * @short_description: Backend abstraction
// *
// * Clutter can be compiled against different backends. Each backend
// * has to implement a set of functions, in order to be used by Clutter.
// *
// * #ClutterBackend is the base class abstracting the various implementation;
// * it provides a basic API to query the backend for generic information
// * and settings.
#[derive(Debug, Clone)]
pub struct Backend {
    // parent_instance: GObject;

    cogl_renderer: Option<dx::Renderer>,
    cogl_display: Option<dx::Display>,
    cogl_context: Option<dx::Context>,
    // cogl_source: GSource
  
    dummy_onscreen: Option<dx::Onscreen>,
  
    // device_manager: Option<dx::DeviceManager>,
  
    font_options: Option<cairo::FontOptions>,
  
    font_name: Option<String>,
  
    units_per_em: f32,
    units_serial: i32,
  
    // event_translators: Vec<>
}

impl Backend {

    // * clutter_backend_get_cogl_context:
    // * @backend: a #ClutterBackend
    // *
    // * Retrieves the #CoglContext associated with the given clutter
    // * @backend. A #CoglContext is required when using some of the
    // * experimental 2.0 Cogl API.
    // *
    // * Since CoglContext is itself experimental API this API should
    // * be considered experimental too.
    // *
    // * This API is not yet supported on OSX because OSX still
    // * uses the stub Cogl winsys and the Clutter backend doesn't
    // * explicitly create a CoglContext.
    // *
    // * Return value: (transfer none): The #CoglContext associated with @backend.
    pub fn get_context(&self) -> Option<dx::Context> {
        // self.cogl_context
        unimplemented!()
    }

    /// Retrieves the font options for `self`.
    ///
    /// # Returns
    ///
    /// the font options of the `Backend`.
    ///  The returned `cairo::FontOptions` is owned by the backend and should
    ///  not be modified or freed
    pub fn get_font_options(&self) -> Option<cairo::FontOptions> {
        // if self.font_options.is_some() {
        //     return self.font_options;
        // }

        // let font_options = cairo::FontOptions::new();
        // font_options.set_hint_style(cairo::HintStyle::None);
        // font_options.set_subpixel_order(cairo::SubpixelOrder::Default);
        // font_options.set_antialias(cairo::Antialias::Default);

        // self.font_options = Some(font_options);

        // self.font_options
        unimplemented!()
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
        // ClutterSettings *settings;
        // gint resolution;
      
        // g_return_val_if_fail (CLUTTER_IS_BACKEND (backend), -1.0);
      
        // settings = clutter_settings_get_default ();
        // g_object_get (settings, "font-dpi", &resolution, NULL);
      
        // if (resolution < 0)
        //   return 96.0;
      
        // return resolution / 1024.0;
        unimplemented!()
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
        // g_return_if_fail (CLUTTER_IS_BACKEND (backend));

        // if (backend->font_options != options) {
        //     if (backend->font_options) {
        //         cairo_font_options_destroy (backend->font_options);
        //     }

        //     if (options) {
        //         backend->font_options = cairo_font_options_copy (options);
        //     } else {
        //         backend->font_options = NULL;
        //     }

        //     g_signal_emit (backend, backend_signals[FONT_CHANGED], 0);
        // }
        unimplemented!()
    }

    /// The ::font-changed signal is emitted each time the font options
    /// have been changed through `Settings`.
    pub fn connect_font_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    /// The ::resolution-changed signal is emitted each time the font
    /// resolutions has been changed through `Settings`.
    pub fn connect_resolution_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    /// The ::settings-changed signal is emitted each time the `Settings`
    /// properties have been changed.
    pub fn connect_settings_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl Default for Backend {
    fn default() -> Self {
        // ClutterMainContext *clutter_context;
        // clutter_context = _clutter_context_get_default ();
        // return clutter_context->backend;
        unimplemented!()
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backend")
    }
}
