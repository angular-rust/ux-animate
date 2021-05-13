use super::{Actor, AllocationFlags, ActorBox, Event, Fog, Perspective, PickMode, StageWindow, HandlerId};
use crate::prelude::*;
use std::{fmt, cell::RefCell};

pub struct Vector4 {
  x: f32,
  y: f32,
  z: f32,
  w: f32,
}

#[derive(Default, Debug, Clone)]
struct StageProps {
    // the stage implementation
    implementation: Option<StageWindow>,
    perspective: Perspective,
    projection: dx::pure::Matrix,
    inverse_projection: dx::pure::Matrix,
    view: dx::pure::Matrix,
    viewport: [f32; 4],

    fog: Fog,

    title: Option<String>,
    key_focused_actor: Option<Actor>,

    // event_queue: Option<GQueue>,

    // stage_hints: StageHint,
    paint_volume_stack: Option<Vec<String>>,

    // current_clip_planes: [Plane; 4],
    pending_queue_redraws: Option<Vec<String>>,

    active_framebuffer: Option<dx::pure::Framebuffer>,

    sync_delay: i32,

    // fps_timer: Option<Timer>,
    timer_n_frames: i32,

    // pick_id_pool: Option<IDPool>,

    // current_state: StageState,

    // paint_data: ptr,
    // paint_notify: GDestroyNotify,
    relayout_pending: bool,
    redraw_pending: bool,
    is_fullscreen: bool,
    is_cursor_visible: bool,
    is_user_resizable: bool,
    use_fog: bool,
    throttle_motion_events: bool,
    use_alpha: bool,
    min_size_changed: bool,
    dirty_viewport: bool,
    dirty_projection: bool,
    accept_focus: bool,
    motion_events_enabled: bool,
    has_custom_perspective: bool,
}

// * SECTION:clutter-stage
// * @short_description: Top level visual element to which actors are placed.
// *
// * #Stage is a top level 'window' on which child actors are placed
// * and manipulated.
// *
// * Backends might provide support for multiple stages. The support for this
// * feature can be checked at run-time using the clutter_feature_available()
// * function and the %CLUTTER_FEATURE_STAGE_MULTIPLE flag. If the backend used
// * supports multiple stages, new #Stage instances can be created
// * using clutter_stage_new(). These stages must be managed by the developer
// * using clutter_actor_destroy(), which will take care of destroying all the
// * actors contained inside them.
// *
// * #Stage is a proxy actor, wrapping the backend-specific
// * implementation of the windowing system. It is possible to subclass
// * #Stage, as long as every overridden virtual function chains up to
// * the parent class corresponding function.
// TODO: implements atk::ImplementorIface, Scriptable, Animatable, Container
// @extends Group, Actor
#[derive(Default, Debug)]
pub struct Stage {
    props: RefCell<StageProps>
}

impl Stage {
    /// Creates a new, non-default stage. A non-default stage is a new
    /// top-level actor which can be used as another container. It works
    /// exactly like the default stage, but while `Stage::get_default`
    /// will always return the same instance, you will have to keep a pointer
    /// to any `Stage` returned by `Stage::new`.
    ///
    /// The ability to support multiple stages depends on the current
    /// backend. Use `feature_available` and
    /// `FeatureFlags::StageMultiple` to check at runtime whether a
    /// backend supports multiple stages.
    ///
    /// # Returns
    ///
    /// a new stage, or `None` if the default backend does
    ///  not support multiple stages. Use `ActorExt::destroy` to
    ///  programmatically close the returned stage.
    pub fn new() -> Stage {
        Default::default()
    }

    fn init(&self) {
        // cairo_rectangle_int_t geom = { 0, };
        // ClutterStagePrivate *priv;
        // ClutterStageWindow *impl;
        // ClutterBackend *backend;
        // int window_scale = 1;
        // GError *error;

        // /* a stage is a top-level object */
        // CLUTTER_SET_PRIVATE_FLAGS(self, CLUTTER_IS_TOPLEVEL);

        // self->priv = priv = clutter_stage_get_instance_private(self);

        // CLUTTER_NOTE (BACKEND, "Creating stage from the default backend");
        // backend = clutter_get_default_backend();

        // error = NULL;
        // impl = _clutter_backend_create_stage(backend, self, &error);

        // if G_LIKELY(impl != NULL) {
        //     _clutter_stage_set_window(self, impl);
        //     _clutter_stage_window_get_geometry(priv->impl, &geom);
        //     window_scale = _clutter_stage_window_get_scale_factor(priv->impl);
        // } else {
        //     if error != NULL {
        //         g_critical("Unable to create a new stage implementation: %s",
        //                     error->message);
        //         g_error_free(error);
        //     } else {
        //         g_critical("Unable to create a new stage implementation.");
        //     }
        // }

        // priv->event_queue = g_queue_new();

        // priv->is_fullscreen = FALSE;
        // priv->is_user_resizable = FALSE;
        // priv->is_cursor_visible = TRUE;
        // priv->use_fog = FALSE;
        // priv->throttle_motion_events = TRUE;
        // priv->min_size_changed = FALSE;
        // priv->sync_delay = -1;

        // /* XXX - we need to keep the invariant that calling
        // * clutter_set_motion_event_enabled() before the stage creation
        // * will cause motion event delivery to be disabled on any newly
        // * created stage. this can go away when we break API and remove
        // * deprecated functions.
        // */
        // priv->motion_events_enabled = _clutter_context_get_motion_events_enabled();

        // clutter_actor_set_background_color(CLUTTER_ACTOR(self),
        //                                     &default_stage_color);

        // priv->perspective.fovy   = 60.0; /* 60 Degrees */
        // priv->perspective.aspect = (float) geom.width / (float) geom.height;
        // priv->perspective.z_near = 0.1;
        // priv->perspective.z_far  = 100.0;

        // cogl_matrix_init_identity(&priv->projection);
        // cogl_matrix_perspective(&priv->projection,
        //                         priv->perspective.fovy,
        //                         priv->perspective.aspect,
        //                         priv->perspective.z_near,
        //                         priv->perspective.z_far);
        // cogl_matrix_get_inverse(&priv->projection,
        //                         &priv->inverse_projection);
        // cogl_matrix_init_identity(&priv->view);
        // cogl_matrix_view_2d_in_perspective(&priv->view,
        //                                     priv->perspective.fovy,
        //                                     priv->perspective.aspect,
        //                                     priv->perspective.z_near,
        //                                     50, /* distance to 2d plane */
        //                                     geom.width * window_scale,
        //                                     geom.height * window_scale);


        // /* FIXME - remove for 2.0 */
        // priv->fog.z_near = 1.0;
        // priv->fog.z_far  = 2.0;

        // priv->relayout_pending = TRUE;

        // clutter_actor_set_reactive(CLUTTER_ACTOR(self), TRUE);
        // clutter_stage_set_title(self, g_get_prgname ());
        // clutter_stage_set_key_focus(self, NULL);

        // g_signal_connect(self, "notify::min-width",
        //                     G_CALLBACK(clutter_stage_notify_min_size), NULL);
        // g_signal_connect(self, "notify::min-height",
        //                     G_CALLBACK(clutter_stage_notify_min_size), NULL);

        // _clutter_stage_set_viewport(self,
        //                             0, 0,
        //                             geom.width,
        //                             geom.height);

        // priv->paint_volume_stack =
        //     g_array_new(FALSE, FALSE, sizeof (ClutterPaintVolume));

        // priv->pick_id_pool = _clutter_id_pool_new(256);
    }

    fn allocate(&self, box_: ActorBox, flags: AllocationFlags) {
        // ClutterStagePrivate *priv = CLUTTER_STAGE(self)->priv;
        // ClutterActorBox alloc = CLUTTER_ACTOR_BOX_INIT_ZERO;
        // float old_width, old_height;
        // float new_width, new_height;
        // float width, height;
        // cairo_rectangle_int_t window_size;
        // int scale_factor;
      
        // if priv->impl == NULL {
        //   return;
        // }
      
        // /* our old allocation */
        // clutter_actor_get_allocation_box(self, &alloc);
        // clutter_actor_box_get_size(&alloc, &old_width, &old_height);
      
        // /* the current allocation */
        // clutter_actor_box_get_size(box, &width, &height);
      
        // /* the current Stage implementation size */
        // _clutter_stage_window_get_geometry(priv->impl, &window_size);
      
        // /* if the stage is fixed size (for instance, it's using a EGL framebuffer)
        //  * then we simply ignore any allocation request and override the
        //  * allocation chain - because we cannot forcibly change the size of the
        //  * stage window.
        //  */
        // if (!clutter_feature_available(CLUTTER_FEATURE_STAGE_STATIC))
        //   {
        //     CLUTTER_NOTE (LAYOUT,
        //                   "Following allocation to %.2fx%.2f (absolute origin %s)",
        //                   width, height,
        //                   (flags & CLUTTER_ABSOLUTE_ORIGIN_CHANGED)
        //                     ? "changed"
        //                     : "not changed");
      
        //     clutter_actor_set_allocation(self, box,
        //                                   flags | CLUTTER_DELEGATE_LAYOUT);
      
        //     // Ensure the window is sized correctly
        //     if !priv->is_fullscreen {
        //         if priv->min_size_changed {
        //             gfloat min_width, min_height;
        //             gboolean min_width_set, min_height_set;
      
        //             g_object_get(G_OBJECT (self),
        //                           "min-width", &min_width,
        //                           "min-width-set", &min_width_set,
        //                           "min-height", &min_height,
        //                           "min-height-set", &min_height_set,
        //                           NULL);
      
        //             if !min_width_set {
        //               min_width = 1;
        //             }
                    
        //             if !min_height_set {
        //               min_height = 1;
        //             }
      
        //             if width < min_width {
        //               width = min_width;
        //             }

        //             if height < min_height {
        //               height = min_height;
        //             }
      
        //             priv->min_size_changed = FALSE;
        //         }
      
        //         if window_size.width != CLUTTER_NEARBYINT (width) ||
        //             window_size.height != CLUTTER_NEARBYINT (height) {
        //             _clutter_stage_window_resize(priv->impl,
        //                                           CLUTTER_NEARBYINT (width),
        //                                           CLUTTER_NEARBYINT (height));
        //         }
        //     }
        // } else {
        //     ClutterActorBox override = { 0, };
      
        //     /* override the passed allocation */
        //     override.x1 = 0;
        //     override.y1 = 0;
        //     override.x2 = window_size.width;
        //     override.y2 = window_size.height;
      
        //     CLUTTER_NOTE(LAYOUT,
        //                   "Overriding original allocation of %.2fx%.2f "
        //                   "with %.2fx%.2f (absolute origin %s)",
        //                   width, height,
        //                   override.x2, override.y2,
        //                   (flags & CLUTTER_ABSOLUTE_ORIGIN_CHANGED)
        //                     ? "changed"
        //                     : "not changed");
      
        //     /* and store the overridden allocation */
        //     clutter_actor_set_allocation(self, &override,
        //                                   flags | CLUTTER_DELEGATE_LAYOUT);
        //   }
      
        // /* XXX: Until Cogl becomes fully responsible for backend windows
        //  * Clutter need to manually keep it informed of the current window
        //  * size. We do this after the allocation above so that the stage
        //  * window has a chance to update the window size based on the
        //  * allocation.
        //  */
        // _clutter_stage_window_get_geometry(priv->impl, &window_size);
      
        // scale_factor = _clutter_stage_window_get_scale_factor(priv->impl);
      
        // window_size.width *= scale_factor;
        // window_size.height *= scale_factor;
      
        // cogl_onscreen_clutter_backend_set_size(window_size.width,
        //                                         window_size.height);
      
        // /* reset the viewport if the allocation effectively changed */
        // clutter_actor_get_allocation_box(self, &alloc);
        // clutter_actor_box_get_size(&alloc, &new_width, &new_height);
      
        // if CLUTTER_NEARBYINT (old_width) != CLUTTER_NEARBYINT (new_width) ||
        //     CLUTTER_NEARBYINT (old_height) != CLUTTER_NEARBYINT (new_height) {
        //     int real_width = CLUTTER_NEARBYINT(new_width);
        //     int real_height = CLUTTER_NEARBYINT(new_height);
      
        //     _clutter_stage_set_viewport(CLUTTER_STAGE (self),
        //                                  0, 0,
        //                                  real_width,
        //                                  real_height);
      
        //     /* Note: we don't assume that set_viewport will queue a full redraw
        //      * since it may bail-out early if something preemptively set the
        //      * viewport before the stage was really allocated its new size.
        //      */
        //     queue_full_redraw(CLUTTER_STAGE (self));
        // }
    }
}

impl Object for Stage {}
impl Is<Stage> for Stage {}

impl AsRef<Stage> for Stage {
    fn as_ref(&self) -> &Stage {
        self
    }
}

/// Trait containing all `Stage` methods.
///
/// # Implementors
///
/// [`Stage`](struct.Stage.html)
pub trait StageExt: 'static {
    /// This function essentially makes sure the right GL context is
    /// current for the passed stage. It is not intended to
    /// be used by applications.
    fn ensure_current(&self);

    /// Ensures that `self` is redrawn
    ///
    /// This function should not be called by applications: it is
    /// used when embedding a `Stage` into a toolkit with
    /// another windowing system, like GTK+.
    fn ensure_redraw(&self);

    /// Ensures that the GL viewport is updated with the current
    /// stage window size.
    ///
    /// This function will queue a redraw of `self`.
    ///
    /// This function should not be called by applications; it is used
    /// when embedding a `Stage` into a toolkit with another
    /// windowing system, like GTK+.
    fn ensure_viewport(&self);

    /// This function is used to emit an event on the main stage.
    ///
    /// You should rarely need to use this function, except for
    /// synthetised events.
    /// ## `event`
    /// a `Event`
    ///
    /// # Returns
    ///
    /// the return value from the signal emission
    fn event(&self, event: &mut Event) -> bool;

    /// Retrieves the value set with `StageExt::set_accept_focus`.
    ///
    /// # Returns
    ///
    /// `true` if the `Stage` should accept focus, and `false`
    ///  otherwise
    fn get_accept_focus(&self) -> bool;

    /// Checks the scene at the coordinates `x` and `y` and returns a pointer
    /// to the `Actor` at those coordinates.
    ///
    /// By using `pick_mode` it is possible to control which actors will be
    /// painted and thus available.
    /// ## `pick_mode`
    /// how the scene graph should be painted
    /// ## `x`
    /// X coordinate to check
    /// ## `y`
    /// Y coordinate to check
    ///
    /// # Returns
    ///
    /// the actor at the specified coordinates,
    ///  if any
    fn get_actor_at_pos(&self, pick_mode: PickMode, x: i32, y: i32) -> Option<Actor>;

    /// Retrieves whether the stage is full screen or not
    ///
    /// # Returns
    ///
    /// `true` if the stage is full screen
    fn get_fullscreen(&self) -> bool;

    /// Retrieves the actor that is currently under key focus.
    ///
    /// # Returns
    ///
    /// the actor with key focus, or the stage
    fn get_key_focus(&self) -> Option<Actor>;

    /// Retrieves the minimum size for a stage window as set using
    /// `StageExt::set_minimum_size`.
    ///
    /// The returned size may not correspond to the actual minimum size and
    /// it is specific to the `Stage` implementation inside the
    /// backend
    /// ## `width`
    /// return location for the minimum width, in pixels,
    ///  or `None`
    /// ## `height`
    /// return location for the minimum height, in pixels,
    ///  or `None`
    fn get_minimum_size(&self) -> (u32, u32);

    /// Retrieves the value set using `StageExt::set_motion_events_enabled`.
    ///
    /// # Returns
    ///
    /// `true` if the per-actor motion event delivery is enabled
    ///  and `false` otherwise
    fn get_motion_events_enabled(&self) -> bool;

    /// Retrieves the hint set with `StageExt::set_no_clear_hint`
    ///
    /// # Returns
    ///
    /// `true` if the stage should not clear itself on every paint
    ///  cycle, and `false` otherwise
    fn get_no_clear_hint(&self) -> bool;

    /// Retrieves the stage perspective.
    /// ## `perspective`
    /// return location for a
    ///  `Perspective`
    fn get_perspective(&self) -> Perspective;

    /// Gets the bounds of the current redraw for `self` in stage pixel
    /// coordinates. E.g., if only a single actor has queued a redraw then
    /// it may redraw the stage with a clip so that it doesn't have to
    /// paint every pixel in the stage. This function would then return the
    /// bounds of that clip. An application can use this information to
    /// avoid some extra work if it knows that some regions of the stage
    /// aren't going to be painted. This should only be called while the
    /// stage is being painted. If there is no current redraw clip then
    /// this function will set `clip` to the full extents of the stage.
    /// ## `clip`
    /// Return location for the clip bounds
    fn get_redraw_clip_bounds(&self) -> cairo::RectangleInt;

    /// Retrieves the value set with `StageExt::set_throttle_motion_events`
    ///
    /// # Returns
    ///
    /// `true` if the motion events are being throttled,
    ///  and `false` otherwise
    fn get_throttle_motion_events(&self) -> bool;

    /// Gets the stage title.
    ///
    /// # Returns
    ///
    /// pointer to the title string for the stage. The
    /// returned string is owned by the actor and should not
    /// be modified or freed.
    fn get_title(&self) -> Option<String>;

    /// Retrieves the value set using `StageExt::set_use_alpha`
    ///
    /// # Returns
    ///
    /// `true` if the stage should honour the opacity and the
    ///  alpha channel of the stage color
    fn get_use_alpha(&self) -> bool;

    /// Retrieves the value set with `StageExt::set_user_resizable`.
    ///
    /// # Returns
    ///
    /// `true` if the stage is resizable by the user.
    fn get_user_resizable(&self) -> bool;

    /// Makes the cursor invisible on the stage window
    fn hide_cursor(&self);

    // /// Makes a screenshot of the stage in RGBA 8bit data, returns a
    // /// linear buffer with `width` * 4 as rowstride.
    // ///
    // /// The alpha data contained in the returned buffer is driver-dependent,
    // /// and not guaranteed to hold any sensible value.
    // /// ## `x`
    // /// x coordinate of the first pixel that is read from stage
    // /// ## `y`
    // /// y coordinate of the first pixel that is read from stage
    // /// ## `width`
    // /// Width dimention of pixels to be read, or -1 for the
    // ///  entire stage width
    // /// ## `height`
    // /// Height dimention of pixels to be read, or -1 for the
    // ///  entire stage height
    // ///
    // /// # Returns
    // ///
    // /// a pointer to newly allocated memory with the buffer
    // ///  or `None` if the read failed. Use `g_free` on the returned data
    // ///  to release the resources it has allocated.
    // fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32) -> Vec<u8>;

    /// Sets whether the `self` should accept the key focus when shown.
    ///
    /// This function should be called before showing `self` using
    /// `ActorExt::show`.
    /// ## `accept_focus`
    /// `true` to accept focus on show
    fn set_accept_focus(&self, accept_focus: bool);

    /// Asks to place the stage window in the fullscreen or unfullscreen
    /// states.
    ///
    ///  ( Note that you shouldn't assume the window is definitely full screen
    /// afterward, because other entities (e.g. the user or window manager)
    /// could unfullscreen it again, and not all window managers honor
    /// requests to fullscreen windows.
    ///
    /// If you want to receive notification of the fullscreen state you
    /// should either use the `Stage::fullscreen` and
    /// `Stage::unfullscreen` signals, or use the notify signal
    /// for the `Stage:fullscreen-set` property
    /// ## `fullscreen`
    /// `true` to to set the stage fullscreen
    fn set_fullscreen(&self, fullscreen: bool);

    /// Sets the key focus on `actor`. An actor with key focus will receive
    /// all the key events. If `actor` is `None`, the stage will receive
    /// focus.
    /// ## `actor`
    /// the actor to set key focus to, or `None`
    fn set_key_focus<P: Is<Actor>>(&self, actor: Option<&P>);

    /// Sets the minimum size for a stage window, if the default backend
    /// uses `Stage` inside a window
    ///
    /// This is a convenience function, and it is equivalent to setting the
    /// `Actor:min-width` and `Actor:min-height` on `self`
    ///
    /// If the current size of `self` is smaller than the minimum size, the
    /// `self` will be resized to the new `width` and `height`
    ///
    /// This function has no effect if `self` is fullscreen
    /// ## `width`
    /// width, in pixels
    /// ## `height`
    /// height, in pixels
    fn set_minimum_size(&self, width: u32, height: u32);

    /// Sets whether per-actor motion events (and relative crossing
    /// events) should be disabled or not.
    ///
    /// The default is `true`.
    ///
    /// If `enable` is `false` the following signals will not be emitted
    /// by the actors children of `self`:
    ///
    ///  - `Actor::motion-event`
    ///  - `Actor::enter-event`
    ///  - `Actor::leave-event`
    ///
    /// The events will still be delivered to the `Stage`.
    ///
    /// The main side effect of this function is that disabling the motion
    /// events will disable picking to detect the `Actor` underneath
    /// the pointer for each motion event. This is useful, for instance,
    /// when dragging a `Actor` across the `self`: the actor underneath
    /// the pointer is not going to change, so it's meaningless to perform
    /// a pick.
    /// ## `enabled`
    /// `true` to enable the motion events delivery, and `false`
    ///  otherwise
    fn set_motion_events_enabled(&self, enabled: bool);

    /// Sets whether the `self` should clear itself at the beginning
    /// of each paint cycle or not.
    ///
    /// Clearing the `Stage` can be a costly operation, especially
    /// if the stage is always covered - for instance, in a full-screen
    /// video player or in a game with a background texture.
    ///
    /// This setting is a hint; it might discard this hint
    /// depending on its internal state.
    ///
    /// If parts of the stage are visible and you disable clearing you
    /// might end up with visual artifacts while painting the contents of
    /// the stage.
    /// ## `no_clear`
    /// `true` if the `self` should not clear itself on every
    ///  repaint cycle
    fn set_no_clear_hint(&self, no_clear: bool);

    /// Sets the stage perspective. Using this function is not recommended
    /// because it will disable it's attempts to generate an
    /// appropriate perspective based on the size of the stage.
    /// ## `perspective`
    /// A `Perspective`
    fn set_perspective(&self, perspective: &mut Perspective);

    /// Sets whether motion events received between redraws should
    /// be throttled or not. If motion events are throttled, those
    /// events received by the windowing system between redraws will
    /// be compressed so that only the last event will be propagated
    /// to the `self` and its actors.
    ///
    /// This function should only be used if you want to have all
    /// the motion events delivered to your application code.
    /// ## `throttle`
    /// `true` to throttle motion events
    fn set_throttle_motion_events(&self, throttle: bool);

    /// Sets the stage title.
    /// ## `title`
    /// A utf8 string for the stage windows title.
    fn set_title(&self, title: &str);

    /// Sets whether the `self` should honour the `Actor:opacity` and
    /// the alpha channel of the `Stage:color`
    /// ## `use_alpha`
    /// whether the stage should honour the opacity or the
    ///  alpha channel of the stage color
    fn set_use_alpha(&self, use_alpha: bool);

    /// Sets if the stage is resizable by user interaction (e.g. via
    /// window manager controls)
    /// ## `resizable`
    /// whether the stage should be user resizable.
    fn set_user_resizable(&self, resizable: bool);

    /// Shows the cursor on the stage window
    fn show_cursor(&self);

    /// Whether the mouse pointer should be visible
    fn get_property_cursor_visible(&self) -> bool;

    /// Whether the mouse pointer should be visible
    fn set_property_cursor_visible(&self, cursor_visible: bool);

    fn get_property_fullscreen_set(&self) -> bool;

    /// The ::activate signal is emitted when the stage receives key focus
    /// from the underlying window system.
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::after-paint signal is emitted after the stage is painted,
    /// but before the results are displayed on the screen.
    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::deactivate signal is emitted when the stage loses key focus
    /// from the underlying window system.
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::delete-event signal is emitted when the user closes a
    /// `Stage` window using the window controls.
    ///
    /// by default will call `main_quit` if `stage` is
    /// the default stage, and `ActorExt::destroy` for any other
    /// stage.
    ///
    /// It is possible to override the default behaviour by connecting
    /// a new handler and returning `true` there.
    ///
    /// This signal is emitted only on backends that
    /// embed `Stage` in native windows. It is not emitted for
    /// backends that use a static frame buffer.
    /// ## `event`
    /// a `Event` of type `EventType::Delete`
    fn connect_delete_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F)
        -> HandlerId;

    /// The ::fullscreen signal is emitted when the stage is made fullscreen.
    fn connect_fullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::unfullscreen signal is emitted when the stage leaves a fullscreen
    /// state.
    fn connect_unfullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_accept_focus_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> HandlerId;

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_fullscreen_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_no_clear_hint_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_user_resizable_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;
}

impl<O: Is<Stage>> StageExt for O {
    fn ensure_current(&self) {
        // unsafe {
        //     ffi::clutter_stage_ensure_current(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn ensure_redraw(&self) {
        // unsafe {
        //     ffi::clutter_stage_ensure_redraw(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn ensure_viewport(&self) {
        // unsafe {
        //     ffi::clutter_stage_ensure_viewport(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn event(&self, event: &mut Event) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_event(
        //         self.as_ref().to_glib_none().0,
        //         event.to_glib_none_mut().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_accept_focus(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_accept_focus(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_actor_at_pos(&self, pick_mode: PickMode, x: i32, y: i32) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_stage_get_actor_at_pos(
        //         self.as_ref().to_glib_none().0,
        //         pick_mode.to_glib(),
        //         x,
        //         y,
        //     ))
        // }
        unimplemented!()
    }

    fn get_fullscreen(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_fullscreen(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_key_focus(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_stage_get_key_focus(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_minimum_size(&self) -> (u32, u32) {
        // unsafe {
        //     let mut width = mem::MaybeUninit::uninit();
        //     let mut height = mem::MaybeUninit::uninit();
        //     ffi::clutter_stage_get_minimum_size(
        //         self.as_ref().to_glib_none().0,
        //         width.as_mut_ptr(),
        //         height.as_mut_ptr(),
        //     );
        //     let width = width.assume_init();
        //     let height = height.assume_init();
        //     (width, height)
        // }
        unimplemented!()
    }

    fn get_motion_events_enabled(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_motion_events_enabled(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_no_clear_hint(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_no_clear_hint(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_perspective(&self) -> Perspective {
        // unsafe {
        //     let mut perspective = Perspective::uninitialized();
        //     ffi::clutter_stage_get_perspective(
        //         self.as_ref().to_glib_none().0,
        //         perspective.to_glib_none_mut().0,
        //     );
        //     perspective
        // }
        unimplemented!()
    }

    fn get_redraw_clip_bounds(&self) -> cairo::RectangleInt {
        // unsafe {
        //     let mut clip = cairo::RectangleInt::uninitialized();
        //     ffi::clutter_stage_get_redraw_clip_bounds(
        //         self.as_ref().to_glib_none().0,
        //         clip.to_glib_none_mut().0,
        //     );
        //     clip
        // }
        unimplemented!()
    }

    fn get_throttle_motion_events(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_throttle_motion_events(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_title(&self) -> Option<String> {
        // unsafe { from_glib_none(ffi::clutter_stage_get_title(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_use_alpha(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_use_alpha(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_user_resizable(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_stage_get_user_resizable(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn hide_cursor(&self) {
        // unsafe {
        //     ffi::clutter_stage_hide_cursor(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    // fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32) -> Vec<u8> {
    //     unsafe {
    //         FromGlibPtrContainer::from_glib_full(ffi::clutter_stage_read_pixels(
    //             self.as_ref().to_glib_none().0,
    //             x,
    //             y,
    //             width,
    //             height,
    //         ))
    //     }
    // }

    fn set_accept_focus(&self, accept_focus: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_accept_focus(
        //         self.as_ref().to_glib_none().0,
        //         accept_focus.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_fullscreen(&self, fullscreen: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_fullscreen(self.as_ref().to_glib_none().0, fullscreen.to_glib());
        // }
        unimplemented!()
    }

    fn set_key_focus<P: Is<Actor>>(&self, actor: Option<&P>) {
        // unsafe {
        //     ffi::clutter_stage_set_key_focus(
        //         self.as_ref().to_glib_none().0,
        //         actor.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_minimum_size(&self, width: u32, height: u32) {
        // unsafe {
        //     ffi::clutter_stage_set_minimum_size(self.as_ref().to_glib_none().0, width, height);
        // }
        unimplemented!()
    }

    fn set_motion_events_enabled(&self, enabled: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_motion_events_enabled(
        //         self.as_ref().to_glib_none().0,
        //         enabled.to_glib(),
        //     );
        // }
    }

    fn set_no_clear_hint(&self, no_clear: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_no_clear_hint(
        //         self.as_ref().to_glib_none().0,
        //         no_clear.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_perspective(&self, perspective: &mut Perspective) {
        // unsafe {
        //     ffi::clutter_stage_set_perspective(
        //         self.as_ref().to_glib_none().0,
        //         perspective.to_glib_none_mut().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_throttle_motion_events(&self, throttle: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_throttle_motion_events(
        //         self.as_ref().to_glib_none().0,
        //         throttle.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_title(&self, title: &str) {
        // unsafe {
        //     ffi::clutter_stage_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_use_alpha(self.as_ref().to_glib_none().0, use_alpha.to_glib());
        // }
        unimplemented!()
    }

    fn set_user_resizable(&self, resizable: bool) {
        // unsafe {
        //     ffi::clutter_stage_set_user_resizable(
        //         self.as_ref().to_glib_none().0,
        //         resizable.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn show_cursor(&self) {
        // unsafe {
        //     ffi::clutter_stage_show_cursor(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn get_property_cursor_visible(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"cursor-visible\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `cursor-visible` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_cursor_visible(&self, cursor_visible: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"cursor-visible\0".as_ptr() as *const _,
        //         Value::from(&cursor_visible).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_fullscreen_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fullscreen-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `fullscreen-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_delete_event<F: Fn(&Self, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_fullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_unfullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_accept_focus_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_fullscreen_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_no_clear_hint_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_user_resizable_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stage")
    }
}
