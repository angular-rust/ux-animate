#![allow(unused_imports)]
use super::{Actor, Backend, Event, FeatureFlags, PickMode, Settings, StageManager, TextDirection};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

// SECTION:clutter-main
// @short_description: Various 'global'  functions.
//
// Functions to retrieve various global  resources and other utility
// functions for mainloops, events and threads
//
// ## The  Threading Model
//
//  is *thread-aware*: all operations performed by  are assumed
// to be under the Big  Lock, which is created when the threading is
// initialized through clutter_init(), and entered when calling user-related
// code during event handling and actor drawing.
//
// The only safe and portable way to use the  API in a multi-threaded
// environment is to only access the  API from a thread that did called
// clutter_init() and clutter_main().
//
// The common pattern for using threads with  is to use worker threads
// to perform blocking operations and then install idle or timeout sources with
// the result when the thread finishes, and update the UI from those callbacks.
//
// For a working example of how to use a worker thread to update the UI, see
// [threads.c](https://git.gnome.org/browse/clutter/tree/examples/threads.c?h=clutter-1.18)

#[derive(Default)]
pub struct MainContextProps {
    // the main windowing system backend
    backend: Option<Backend>,

    // the object holding all the stage instances
    stage_manager: Option<StageManager>,

    // the clock driving all the frame operations
    // master_clock: Option<MasterClock>,

    // the main event queue
    // events_queue: Option<GQueue>,

    // the event filters added via clutter_event_add_filter. these are
    // ordered from least recently added to most recently added
    event_filters: Option<Vec<bool>>,

    pick_mode: PickMode,

    // default FPS; this is only used if we cannot sync to vblank
    frame_rate: u32,

    // actors with a grab on all devices
    // pointer_grab_actor: Option<Actor>,
    // keyboard_grab_actor: Option<Actor>,

    // stack of actors with shaders during paint */
    shaders: Option<Vec<bool>>,

    // fb bit masks for col<->id mapping in picking
    fb_r_mask: i32,
    fb_g_mask: i32,
    fb_b_mask: i32,
    fb_r_mask_used: i32,
    fb_g_mask_used: i32,
    fb_b_mask_used: i32,

    // Global font map
    // font_map: Option<dx::pure::PangoFontMap>,

    // stack of #Event
    current_event: Option<Vec<Event>>,

    // list of repaint functions installed through
    // threads_add_repaint_func()
    repaint_funcs: Option<Vec<bool>>,
    last_repaint_id: u32,

    // main settings singleton
    settings: Option<Settings>,

    // boolean flags
    is_initialized: bool,
    motion_events_per_actor: bool,
    defer_display_setup: bool,
    options_parsed: bool,
    show_fps: bool,
}

unsafe impl Send for MainContextProps {}

// MainContext:
//
// The shared state of 
pub struct MainContext {
    props: Arc<Mutex<MainContextProps>>,
}

impl MainContext {
    fn new() -> Self {
        println!("CREATE MAIN CONTEXT INSTANCE");

        let props = MainContextProps::default();

        Self {
            props: Arc::new(Mutex::new(props)),
        }
    }

    // replacement for clutter_context_get_default
    pub fn global() -> &'static Self {
        static CONTEXT_INSTANCE: OnceCell<MainContext> = OnceCell::new();
        CONTEXT_INSTANCE.get_or_init(Self::new)
    }

    pub fn backend(&self) -> Option<Backend> {
        match self.props.lock() {
            Ok(props) => match &props.backend {
                Some(backend) => {
                    let result = backend.clone();
                    // Some(result)
                    unimplemented!()
                }
                None => None,
            },
            Err(_) => {
                panic!("MainContext poisoned");
            }
        }
    }
}

static GLOB: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

fn base_init() {
    match GLOB.lock() {
        Ok(mut initialized) => {
            *initialized = true;

            // bindtextdomain (GETTEXT_PACKAGE, CLUTTER_LOCALEDIR);
            // bind_textdomain_codeset (GETTEXT_PACKAGE, "UTF-8");

            // initialise GLib type system
            //g_type_init ();

            // initialise the Big Animate Lock™ if necessary
            // threads_init_default
        }
        Err(_) => {
            panic!("Glob poisoned");
        }
    }
}

fn get_text_direction() -> TextDirection {
    // TextDirection dir = CLUTTER_TEXT_DIRECTION_LTR;
    // const gchar *direction;

    // direction = g_getenv ("CLUTTER_TEXT_DIRECTION");
    // if direction && *direction != '\0' {
    //     if strcmp(direction, "rtl") == 0 {
    //       dir = CLUTTER_TEXT_DIRECTION_RTL;
    //     } else if strcmp(direction, "ltr") == 0 {
    //       dir = CLUTTER_TEXT_DIRECTION_LTR;
    //     }
    // } else {
    //     /* Translators: Leave this UNTRANSLATED if your language is
    //      * left-to-right.  If your language is right-to-left
    //      * (e.g. Hebrew, Arabic), translate it to "default:RTL".
    //      *
    //      * Do NOT translate it to non-English e.g. "predefinito:LTR"! If
    //      * it isn't default:LTR or default:RTL it will not work.
    //      */
    //     char *e = _("default:LTR");

    //     if strcmp (e, "default:RTL") == 0 {
    //       dir = CLUTTER_TEXT_DIRECTION_RTL;
    //     } else if (strcmp (e, "default:LTR") == 0 {
    //       dir = CLUTTER_TEXT_DIRECTION_LTR;
    //     } else {
    //       g_warning ("Whoever translated default:LTR did so wrongly.");
    //     }
    // }

    // CLUTTER_NOTE(MISC, "Text direction: %s",
    //               dir == CLUTTER_TEXT_DIRECTION_RTL ? "rtl" : "ltr");

    // return dir;
    unimplemented!()
}

pub struct Features {
    flags: FeatureFlags,
    features_set: bool,
}

// SECTION:clutter-feature
// @short_description: Run-time detection of  features
//
// Parts of  depend on the underlying platform, including the
// capabilities of the backend used and the OpenGL features exposed through the
//  and COGL API.
//
// It is possible to ask whether  has support for specific features at
// run-time.
//
// See also dx_get_features() and #CoglFeatureFlags
fn feature_init(backend: &Option<Backend>) -> bool {
    // MainContext *context;

    // CLUTTER_NOTE (MISC, "checking features");

    // if !__features {
    //     CLUTTER_NOTE (MISC, "allocating features data");
    //     __features = g_new0 (Features, 1);
    //     __features->features_set = FALSE; /* don't rely on zero-ing */
    // }

    // if __features->features_set {
    //     return true;
    // }

    // context = _clutter_context_get_default ();

    // makes sure we have a GL context; if we have, this is a no-op
    match backend {
        Some(backend) => {
            if !backend.create_context() {
                return false;
            }
        }
        None => {}
    }

    // __features->flags = (clutter_features_from_cogl (dx_get_features ())
    //                     | _clutter_backend_get_features (context->backend));

    // __features->features_set = TRUE;

    // CLUTTER_NOTE (MISC, "features checked");

    true
}

fn features_from_cogl(dx_flags: dx::core::FeatureFlags) -> FeatureFlags {
    let mut clutter_flags: FeatureFlags = FeatureFlags::NONE;

    if (dx_flags & dx::core::FeatureFlags::TEXTURE_NPOT).bits() != 0 {
        clutter_flags |= FeatureFlags::TEXTURE_NPOT;
    }

    if (dx_flags & dx::core::FeatureFlags::TEXTURE_YUV).bits() != 0 {
        clutter_flags |= FeatureFlags::TEXTURE_YUV;
    }

    if (dx_flags & dx::core::FeatureFlags::TEXTURE_READ_PIXELS).bits() != 0 {
        clutter_flags |= FeatureFlags::TEXTURE_READ_PIXELS;
    }

    if (dx_flags & dx::core::FeatureFlags::SHADERS_GLSL).bits() != 0 {
        clutter_flags |= FeatureFlags::SHADERS_GLSL;
    }

    if (dx_flags & dx::core::FeatureFlags::OFFSCREEN).bits() != 0 {
        clutter_flags |= FeatureFlags::OFFSCREEN;
    }

    clutter_flags
}

fn init_real(props: &mut MainContextProps) -> InitError {
    // Note, creates backend if not already existing, though parse args will
    // have likely created it

    let backend = &props.backend;

    if !props.options_parsed {
        // if error {
        //     // g_set_error(error, InitError,
        //     //     InitError::ErrorInternal,
        //     //     r#"When using clutter_get_option_group_without_init()
        //     //     "you must parse options before calling clutter_init()"#);
        // } else {
        //     // g_critical(r#"When using clutter_get_option_group_without_init()
        //     //     "you must parse options before calling clutter_init()"#);
        // }
        // return InitError::Internal;
    }

    // // Call backend post parse hooks.
    // if !_clutter_backend_post_parse(backend, error) {
    //     return InitError::Backend;
    // }

    // // If we are displaying the regions that would get redrawn with clipped
    // // redraws enabled we actually have to disable the clipped redrawing
    // // because otherwise we end up with nasty trails of rectangles everywhere.
    // if clutter_paint_debug_flags & CLUTTER_DEBUG_REDRAWS {
    //     clutter_paint_debug_flags |= CLUTTER_DEBUG_DISABLE_CLIPPED_REDRAWS;
    // }

    // // The same is true when drawing the outlines of paint volumes...
    // if clutter_paint_debug_flags & CLUTTER_DEBUG_PAINT_VOLUMES {
    //     clutter_paint_debug_flags |=
    //         CLUTTER_DEBUG_DISABLE_CLIPPED_REDRAWS | CLUTTER_DEBUG_DISABLE_CULLING;
    // }

    // this will take care of initializing Cogl's state and
    // query the GL machinery for features
    if !feature_init(backend) {
        return InitError::Backend;
    }

    // clutter_text_direction = get_text_direction();

    // Initiate event collection
    match backend {
        Some(backend) => {
            backend.init_events();
        }
        None => {}
    }

    // clutter_is_initialized = true;
    props.is_initialized = true;

    // // Initialize a11y */
    // if clutter_enable_accessibility {
    //     cally_accessibility_init();
    // }

    InitError::Sucess
}

// clutter_init:
// @argc: (inout): The number of arguments in @argv
// @argv: (array length=argc) (inout) (allow-none): A pointer to an array
//   of arguments.
//
// Initialises everything needed to operate with  and parses some
// standard command line options; @argc and @argv are adjusted accordingly
// so your own code will never see those standard arguments.
//
// It is safe to call this function multiple times.
//
// This function will not abort in case of errors during
// initialization; clutter_init() will print out the error message on
// stderr, and will return an error code. It is up to the application
// code to handle this case. If you need to display the error message
// yourself, you can use clutter_init_with_args(), which takes a #GError
// pointer.
//
// If this function fails, and returns an error code, any subsequent
//  API will have undefined behaviour - including segmentation
// faults and assertion failures. Make sure to handle the returned
// #InitError enumeration value.
//
// Return value: a #InitError value
pub fn animate_init() {
    base_init();
    let ctx = MainContext::global();
    match ctx.props.lock() {
        Ok(mut props) => {
            if !props.defer_display_setup {
                // parse_args will trigger backend creation and things like
                //  DISPLAY connection etc.

                // if (!parse_args (argc, argv, &error))
                // {
                //     g_critical ("Unable to initialize UX Animate: %s", error->message);
                //     g_error_free (error);

                //     res = InitError::ErrorInternal;
                // } else {
                //     res = InitError::Sucess;
                // }
            } else {
                let res = init_real(&mut props);
                // println!("Unable to initialize UX Animate")
            }
        }
        Err(_) => {
            panic!("MainContext poisoned")
        }
    }
}

pub fn animate_main() {}

pub fn animate_quit() {}

/* shared between main and frame-source */
pub struct ThreadsDispatch {
    // func: GSourceFunc,
// gpointer data: ptr,
// notify: GDestroyNotify,
}

pub struct Vertex4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct Plane {
    pub v0: [f32; 3],
    pub n: [f32; 3],
}

pub enum CullResult {
    ResultUnknown,
    ResultIn,
    ResultOut,
    ResultPartial,
}

// InitError:
// @CLUTTER_INIT_SUCCESS: Initialisation successful
// @CLUTTER_INIT_ERROR_UNKNOWN: Unknown error
// @CLUTTER_INIT_ERROR_THREADS: Thread initialisation failed
// @CLUTTER_INIT_ERROR_BACKEND: Backend initialisation failed
// @CLUTTER_INIT_ERROR_INTERNAL: Internal error
//
// Error conditions returned by clutter_init() and clutter_init_with_args().
//
pub enum InitError {
    Sucess = 1,
    Unknown = 0,
    Threads = -1,
    Backend = -2,
    Internal = -3,
}
