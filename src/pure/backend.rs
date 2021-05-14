use super::{HandlerId, MainContext, Settings};
use std::{cell::RefCell, fmt};

struct BackendProps {
    dx_renderer: Option<dx::Renderer>,
    dx_display: Option<dx::Display>,
    dx_context: Option<dx::Context>,
    // dx_source: GSource
    dummy_onscreen: Option<dx::Onscreen>,

    // device_manager: Option<dx::DeviceManager>,
    font_options: Option<cairo::FontOptions>,

    font_name: Option<String>,

    units_per_em: f32,
    units_serial: i32,
    // event_translators: Vec<>
}
// * @short_description: Backend abstraction
// *
// *  can be compiled against different backends. Each backend
// * has to implement a set of functions, in order to be used by .
// *
// * #Backend is the base class abstracting the various implementation;
// * it provides a basic API to query the backend for generic information
// * and settings.
pub struct Backend {
    props: RefCell<BackendProps>,
}

impl Backend {
    // * clutter_backend_get_dx_context:
    // * @backend: a #Backend
    // *
    // * Retrieves the #CoglContext associated with the given clutter
    // * @backend. A #CoglContext is required when using some of the
    // * experimental 2.0 Cogl API.
    // *
    // * Since CoglContext is itself experimental API this API should
    // * be considered experimental too.
    // *
    // * This API is not yet supported on OSX because OSX still
    // * uses the stub Cogl winsys and the  backend doesn't
    // * explicitly create a CoglContext.
    // *
    // * Return value: (transfer none): The #CoglContext associated with @backend.
    pub fn get_context(&self) -> Option<dx::Context> {
        let props = self.props.borrow();
        props.dx_context.clone()
    }

    pub fn create_context(&self) -> bool {
        // GError *internal_error = NULL;
        // const char *drivers_list;
        // char **known_drivers;
        // gboolean allow_any;
        // int i;

        // if backend.dx_context != NULL {
        //     return true;
        // }

        // if allowed_drivers == NULL {
        //   allowed_drivers = CLUTTER_DRIVERS;
        // }

        // allow_any = strstr(allowed_drivers, "*") != NULL;

        // drivers_list = g_getenv("CLUTTER_DRIVER");
        // if drivers_list == NULL {
        //   drivers_list = allowed_drivers;
        // }

        // known_drivers = g_strsplit(drivers_list, ",", 0);

        // for (i = 0; backend.dx_context == NULL && known_drivers[i] != NULL; i++) {
        //     const char *driver_name = known_drivers[i];
        //     gboolean is_any = g_str_equal(driver_name, "*");
        //     int j;

        //     for (j = 0; j < G_N_ELEMENTS (all_known_drivers); j++)
        //       {
        //         if (!allow_any && !is_any && !strstr(driver_name, all_known_drivers[j].driver_name))
        //           continue;

        //         if (allow_any && is_any) ||
        //             (is_any && strstr(allowed_drivers, all_known_drivers[j].driver_name)) ||
        //             g_str_equal(all_known_drivers[j].driver_name, driver_name)
        //         {
        //             CLUTTER_NOTE (BACKEND, "Checking for the %s driver", all_known_drivers[j].driver_desc);

        //             if (clutter_backend_do_real_create_context(backend, all_known_drivers[j].driver_id, &internal_error))
        //               break;

        //             if (internal_error)
        //               {
        //                 CLUTTER_NOTE(BACKEND, "Unable to use the %s driver: %s",
        //                               all_known_drivers[j].driver_desc,
        //                               internal_error.message);
        //                 g_clear_error(&internal_error);
        //               }
        //         }
        //     }
        // }

        // g_strfreev (known_drivers);

        // if backend.dx_context == NULL {
        //     if internal_error != NULL {
        //       g_propagate_error(error, internal_error);
        //     } else {
        //       g_set_error_literal(error, CLUTTER_INIT_ERROR,
        //                            CLUTTER_INIT_ERROR_BACKEND,
        //                           _("Unable to initialize the  backend: no available drivers found."));
        //     }
        //     return false;
        // }

        // backend.dx_source = dx_glib_source_new(backend.dx_context, G_PRIORITY_DEFAULT);
        // g_source_attach(backend.dx_source, NULL);

        true
    }

    fn do_real_create_context(&self, driver_id: dx::Driver) -> bool {
        // fn error() -> bool {
        //     if backend.dx_display.is_some() {
        //         dx_object_unref(backend.dx_display);
        //         backend.dx_display = NULL;
        //     }

        //     if backend.dx_renderer.is_some() {
        //         dx_object_unref(backend.dx_renderer);
        //         backend.dx_renderer = NULL;
        //     }

        //     if swap_chain.is_some() {
        //         dx_object_unref(swap_chain);
        //     }

        //     false
        // }

        // BackendClass *klass;
        // CoglSwapChain *swap_chain;
        // GError *internal_error;

        // klass = CLUTTER_BACKEND_GET_CLASS(backend);

        // swap_chain = NULL;
        // internal_error = NULL;

        // info!("Creating Cogl renderer");

        // if klass.get_renderer.is_some() {
        //     backend.dx_renderer = klass.get_renderer(backend, &internal_error);
        // } else {
        //     backend.dx_renderer = dx_renderer_new();
        // }

        // if backend.dx_renderer.is_none() {
        //     return error();
        // }

        // // If the application is trying to act as a Wayland compositor then
        // // it needs to have an EGL-based renderer backend
        // #[cfg(feature = "wayland")]
        // if _wayland_compositor_display {
        //     dx_renderer_add_constraint(backend.dx_renderer,
        //                                 COGL_RENDERER_CONSTRAINT_USES_EGL);
        // }

        // info!("Connecting the renderer");
        // dx_renderer_set_driver(backend.dx_renderer, driver_id);
        // if !dx_renderer_connect(backend.dx_renderer, &internal_error) {
        //     return error();
        // }

        // info!("Creating Cogl swap chain");
        // swap_chain = dx_swap_chain_new ();

        // info!("Creating Cogl display");
        // if klass.get_display.is_some() {
        //     backend.dx_display = klass.get_display(backend,
        //                                                 backend.dx_renderer,
        //                                                 swap_chain,
        //                                                 &internal_error);
        // } else {
        //     CoglOnscreenTemplate *tmpl;

        //     tmpl = dx_onscreen_template_new(swap_chain);

        //     // XXX: I have some doubts that this is a good design.
        //     //
        //     // Conceptually should we be able to check an onscreen_template
        //     // without more details about the CoglDisplay configuration?
        //     let res: bool = dx_renderer_check_onscreen_template(backend.dx_renderer,
        //                                                 tmpl,
        //                                                 &internal_error);

        //     if !res {
        //         return error();
        //     }

        //     backend.dx_display = dx_display_new(backend.dx_renderer, tmpl);

        //     // the display owns the template
        //     dx_object_unref(tmpl);
        // }

        // if backend.dx_display.is_none() {
        //     return error();
        // }

        // #[cfg(feature = "wayland")]
        // dx_wayland_display_set_compositor_display(backend.dx_display,
        //                                         _wayland_compositor_display);

        // info!("Setting up the display");
        // if !dx_display_setup(backend.dx_display, &internal_error) {
        //     return error();
        // }

        // info!("Creating the Cogl context");
        // backend.dx_context = dx_context_new(backend.dx_display, &internal_error);
        // if backend.dx_context.is_none() {
        //     return error();
        // }

        // // the display owns the renderer and the swap chain
        // dx_object_unref(backend.dx_renderer);
        // dx_object_unref(swap_chain);

        true
    }

    pub fn init_events(&self) {
        // let mut input_backend: Option<String> = None;

        // input_backend = g_getenv("CLUTTER_INPUT_BACKEND");
        // if input_backend.is_some() {
        //   input_backend = g_intern_string(input_backend);
        // }

        #[cfg(target_os = "macos")]
        if clutter_check_windowing_backend(CLUTTER_WINDOWING_OSX)
            && (input_backend.is_some() || input_backend == I_(CLUTTER_INPUT_OSX))
        {
            _clutter_backend_osx_events_init(backend);
        }

        #[cfg(target_os = "windows")]
        if clutter_check_windowing_backend(CLUTTER_WINDOWING_WIN32)
            && (input_backend.is_some() || input_backend == I_(CLUTTER_INPUT_WIN32))
        {
            _clutter_backend_win32_events_init(backend);
        }

        if cfg!(target_os = "linux") {
            #[cfg(feature = "x11")]
            if clutter_check_windowing_backend(CLUTTER_WINDOWING_X11)
                && (input_backend.is_some() || input_backend == I_(CLUTTER_INPUT_X11))
            {
                _clutter_backend_x11_events_init(backend);
            }

            #[cfg(feature = "gdk")]
            if clutter_check_windowing_backend(CLUTTER_WINDOWING_GDK)
                && (input_backend.is_some() || input_backend == I_(CLUTTER_INPUT_GDK))
            {
                _clutter_backend_gdk_events_init(backend);
            }

            #[cfg(feature = "evdev")]
            // Evdev can be used regardless of the windowing system
            if input_backend.is_some() && strcmp(input_backend, CLUTTER_INPUT_EVDEV) == 0 {
                _clutter_events_evdev_init(backend);
            }

            #[cfg(feature = "egl")]
            // but we do want to always use it for EGL native
            if clutter_check_windowing_backend(CLUTTER_WINDOWING_EGL) {
                _clutter_events_evdev_init(backend);
            }

            #[cfg(feature = "tslib")]
            // Tslib can be used regardless of the windowing system
            if input_backend.is_some() && strcmp(input_backend, CLUTTER_INPUT_TSLIB) == 0 {
                _clutter_events_tslib_init(backend);
            }

            #[cfg(feature = "wayland")]
            if clutter_check_windowing_backend(CLUTTER_WINDOWING_WAYLAND)
                && (input_backend.is_some() || input_backend == I_(CLUTTER_INPUT_WAYLAND))
            {
                _clutter_events_wayland_init(backend);
            }

            // if input_backend.is_some() {
            //     if input_backend != I_(CLUTTER_INPUT_NULL) {
            //         error!("Unrecognized input backend '%s'", input_backend);
            //     }
            // } else {
            //     error!("Unknown input backend");
            // }
        }
    }

    /// Retrieves the font options for `self`.
    ///
    /// # Returns
    ///
    /// the font options of the `Backend`.
    ///  The returned `cairo::FontOptions` is owned by the backend and should
    ///  not be modified or freed
    pub fn get_font_options(&self) -> cairo::FontOptions {
        let mut props = self.props.borrow_mut();
        match &props.font_options {
            Some(font_options) => font_options.clone(),
            None => {
                let mut font_options = cairo::FontOptions::new();
                font_options.set_hint_style(cairo::HintStyle::None);
                font_options.set_subpixel_order(cairo::SubpixelOrder::Default);
                font_options.set_antialias(cairo::Antialias::Default);

                props.font_options = Some(font_options.clone());

                font_options
            }
        }
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
        let resolution = Settings::global().get_font_dpi();

        if resolution < 0.0 {
            return 96.0;
        }

        resolution / 1024.0
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
    pub fn set_font_options(&self, options: Option<cairo::FontOptions>) {
        let mut props = self.props.borrow_mut();

        if props.font_options != options {
            props.font_options = options;
            // g_signal_emit (backend, backend_signals[FONT_CHANGED], 0);
        }
    }

    /// The ::font-changed signal is emitted each time the font options
    /// have been changed through `Settings`.
    pub fn connect_font_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    /// The ::resolution-changed signal is emitted each time the font
    /// resolutions has been changed through `Settings`.
    pub fn connect_resolution_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    /// The ::settings-changed signal is emitted each time the `Settings`
    /// properties have been changed.
    pub fn connect_settings_changed<F: Fn(&Backend) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl Default for Backend {
    fn default() -> Self {
        MainContext::global()
            .backend()
            .expect("Error in MainContext")
    }
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backend")
    }
}
