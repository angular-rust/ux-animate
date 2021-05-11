use super::{Actor, Event, Fog, Perspective, PickMode, StageWindow};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// * SECTION:clutter-stage
// * @short_description: Top level visual element to which actors are placed.
// *
// * #ClutterStage is a top level 'window' on which child actors are placed
// * and manipulated.
// *
// * Backends might provide support for multiple stages. The support for this
// * feature can be checked at run-time using the clutter_feature_available()
// * function and the %CLUTTER_FEATURE_STAGE_MULTIPLE flag. If the backend used
// * supports multiple stages, new #ClutterStage instances can be created
// * using clutter_stage_new(). These stages must be managed by the developer
// * using clutter_actor_destroy(), which will take care of destroying all the
// * actors contained inside them.
// *
// * #ClutterStage is a proxy actor, wrapping the backend-specific
// * implementation of the windowing system. It is possible to subclass
// * #ClutterStage, as long as every overridden virtual function chains up to
// * the parent class corresponding function.
// TODO: implements atk::ImplementorIface, Scriptable, Animatable, Container
// @extends Group, Actor
#[derive(Debug, Clone)]
pub struct Stage {
    /* the stage implementation */
    implementation: Option<StageWindow>,
    perspective: Perspective,
    projection: dx::Matrix,
    inverse_projection: dx::Matrix,
    view: dx::Matrix,
    viewport: [f32; 4],

    fog: Fog,

    title: Option<String>,
    key_focused_actor: Option<Actor>,

    // event_queue: Option<GQueue>,

    // stage_hints: StageHint,
    paint_volume_stack: Option<Vec<String>>,

    // current_clip_planes: [Plane; 4],
    pending_queue_redraws: Option<Vec<String>>,

    active_framebuffer: Option<dx::Framebuffer>,

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
        // unsafe { Actor::from_glib_none(ffi::clutter_stage_new()).unsafe_cast() }
        unimplemented!()
    }

    fn init(&self) {}
}

impl Object for Stage {}
impl Is<Stage> for Stage {}

impl AsRef<Stage> for Stage {
    fn as_ref(&self) -> &Stage {
        self
    }
}

impl Default for Stage {
    fn default() -> Self {
        Self::new()
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
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::after-paint signal is emitted after the stage is painted,
    /// but before the results are displayed on the screen.
    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::deactivate signal is emitted when the stage loses key focus
    /// from the underlying window system.
    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

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
        -> SignalHandlerId;

    /// The ::fullscreen signal is emitted when the stage is made fullscreen.
    fn connect_fullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::unfullscreen signal is emitted when the stage leaves a fullscreen
    /// state.
    fn connect_unfullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accept_focus_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_fullscreen_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_clear_hint_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_user_resizable_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
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

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_after_paint<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_delete_event<F: Fn(&Self, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_fullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_unfullscreen<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_accept_focus_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_fullscreen_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_key_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_no_clear_hint_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_perspective_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_user_resizable_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stage")
    }
}
