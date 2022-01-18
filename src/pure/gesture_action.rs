use super::{Action, Actor, Event, EventSequence, GestureTriggerEdge, HandlerId, InputDevice};
use crate::prelude::*;
use std::fmt;

#[derive(Default, Debug)]
pub struct GesturePoint {
    device: Option<InputDevice>,
    sequence: Option<EventSequence>,
    last_event: Option<Event>,

    press_x: f32,
    press_y: f32,
    last_motion_time: i64,
    last_motion_x: f32,
    last_motion_y: f32,
    last_delta_time: i64,
    last_delta_x: f32,
    last_delta_y: f32,
    release_x: f32,
    release_y: f32,
}

#[derive(Default, Debug)]
struct GestureActionProps {
    stage: Option<Actor>,

    requested_nb_points: u32,
    points: Option<Vec<GesturePoint>>,

    actor_capture_id: u32,
    stage_capture_id: u64,

    edge: GestureTriggerEdge,
    distance_x: f32,
    distance_y: f32,

    in_gesture: bool,
}

// @Title: GestureAction
// @Short_Description: Action for gesture gestures
//
// #GestureAction is a sub-class of #Action that implements
// the logic for recognizing gesture gestures. It listens for low level events
// such as #ButtonEvent and #MotionEvent on the stage to raise
// the #GestureAction::gesture-begin, #GestureAction::gesture-progress,
// and #GestureAction::gesture-end signals.
//
// To use #GestureAction you just need to apply it to a #Actor
// using actor_add_action() and connect to the signals:
//
// ```
//   Action *action = gesture_action_new ();
//
//   actor_add_action (actor, action);
//
//   g_signal_connect (action, "gesture-begin", G_CALLBACK (on_gesture_begin), NULL);
//   g_signal_connect (action, "gesture-progress", G_CALLBACK (on_gesture_progress), NULL);
//   g_signal_connect (action, "gesture-end", G_CALLBACK (on_gesture_end), NULL);
// ```
//
// ## Creating Gesture actions
//
// A #GestureAction provides four separate states that can be
// used to recognize or ignore gestures when writing a new action class:
//
//  - Prepare -> Cancel
//  - Prepare -> Begin -> Cancel
//  - Prepare -> Begin -> End
//  - Prepare -> Begin -> Progress -> Cancel
//  - Prepare -> Begin -> Progress -> End
//
// Each #GestureAction starts in the "prepare" state, and calls
// the #GestureActionClass.gesture_prepare() virtual function; this
// state can be used to reset the internal state of a #GestureAction
// subclass, but it can also immediately cancel a gesture without going
// through the rest of the states.
//
// The "begin" state follows the "prepare" state, and calls the
// #GestureActionClass.gesture_begin() virtual function. This state
// signals the start of a gesture recognizing process. From the "begin" state
// the gesture recognition process can successfully end, by going to the
// "end" state; it can continue in the "progress" state, in case of a
// continuous gesture; or it can be terminated, by moving to the "cancel"
// state.
//
// In case of continuous gestures, the #GestureAction will use
// the "progress" state, calling the #GestureActionClass.gesture_progress()
// virtual function; the "progress" state will continue until the end of the
// gesture, in which case the "end" state will be reached, or until the
// gesture is cancelled, in which case the "cancel" gesture will be used
// instead.
//
// Since: 1.8
// @extends Action, ActorMeta,
#[derive(Default, Debug)]
pub struct GestureAction {
    props: GestureActionProps,
    inner: Action,
}

impl GestureAction {
    /// Creates a new `GestureAction` instance.
    ///
    /// # Returns
    ///
    /// the newly created `GestureAction`
    pub fn new() -> GestureAction {
        // unsafe { Action::from_glib_none(ffi::gesture_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for GestureAction {}
impl Is<GestureAction> for GestureAction {}

impl AsRef<GestureAction> for GestureAction {
    fn as_ref(&self) -> &GestureAction {
        self
    }
}

/// Trait containing all `GestureAction` methods.
///
/// # Implementors
///
/// [`GestureAction`](struct.GestureAction.html), [`PanAction`](struct.PanAction.html), [`RotateAction`](struct.RotateAction.html), [`SwipeAction`](struct.SwipeAction.html), [`TapAction`](struct.TapAction.html), [`ZoomAction`](struct.ZoomAction.html)
pub trait GestureActionExt: 'static {
    /// Cancel a `GestureAction` before it begins
    fn cancel(&self);

    /// Retrieves the `InputDevice` of a touch point.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    ///
    /// # Returns
    ///
    /// the `InputDevice` of a touch point.
    fn get_device(&self, point: u32) -> Option<InputDevice>;

    /// Retrieves a reference to the last `Event` for a touch point. Call
    /// `event_copy` if you need to store the reference somewhere.
    /// ## `point`
    /// index of a point currently active
    ///
    /// # Returns
    ///
    /// the last `Event` for a touch point.
    fn get_last_event(&self, point: u32) -> Option<Event>;

    /// Retrieves the coordinates, in stage space, of the latest motion
    /// event during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `motion_x`
    /// return location for the latest motion
    ///  event's X coordinate
    /// ## `motion_y`
    /// return location for the latest motion
    ///  event's Y coordinate
    fn get_motion_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the incremental delta since the last motion event
    /// during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `delta_x`
    /// return location for the X axis
    ///  component of the incremental motion delta
    /// ## `delta_y`
    /// return location for the Y axis
    ///  component of the incremental motion delta
    ///
    /// # Returns
    ///
    /// the distance since last motion event
    fn get_motion_delta(&self, point: u32) -> (f32, f32, f32);

    /// Retrieves the number of points currently active.
    ///
    /// # Returns
    ///
    /// the number of points currently active.
    fn get_n_current_points(&self) -> u32;

    /// Retrieves the number of requested points to trigger the gesture.
    ///
    /// # Returns
    ///
    /// the number of points to trigger the gesture.
    fn get_n_touch_points(&self) -> i32;

    /// Retrieves the coordinates, in stage space, of the press event
    /// that started the dragging for a specific touch point.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `press_x`
    /// return location for the press
    ///  event's X coordinate
    /// ## `press_y`
    /// return location for the press
    ///  event's Y coordinate
    fn get_press_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the coordinates, in stage space, where the touch point was
    /// last released.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `release_x`
    /// return location for the X coordinate of
    ///  the last release
    /// ## `release_y`
    /// return location for the Y coordinate of
    ///  the last release
    fn get_release_coords(&self, point: u32) -> (f32, f32);

    /// Retrieves the `EventSequence` of a touch point.
    /// ## `point`
    /// index of a point currently active
    ///
    /// # Returns
    ///
    /// the `EventSequence` of a touch point.
    fn get_sequence(&self, point: u32) -> Option<EventSequence>;

    /// Retrieves the threshold trigger distance of the gesture `self`,
    /// as set using `GestureActionExt::set_threshold_trigger_distance`.
    /// ## `x`
    /// The return location for the horizontal distance, or `None`
    /// ## `y`
    /// The return location for the vertical distance, or `None`
    fn get_threshold_trigger_distance(&self) -> (f32, f32);

    /// Retrieves the edge trigger of the gesture `self`, as set using
    /// `GestureActionExt::set_threshold_trigger_edge`.
    ///
    /// # Returns
    ///
    /// the edge trigger
    fn get_threshold_trigger_edge(&self) -> GestureTriggerEdge;

    /// Retrieves the velocity, in stage pixels per millisecond, of the
    /// latest motion event during the dragging.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `velocity_x`
    /// return location for the latest motion
    ///  event's X velocity
    /// ## `velocity_y`
    /// return location for the latest motion
    ///  event's Y velocity
    fn get_velocity(&self, point: u32) -> (f32, f32, f32);

    /// Sets the number of points needed to trigger the gesture.
    /// ## `nb_points`
    /// a number of points
    fn set_n_touch_points(&self, nb_points: i32);

    /// Sets the threshold trigger distance for the gesture drag threshold, if any.
    ///
    /// This function should only be called by sub-classes of
    /// `GestureAction` during their construction phase.
    /// ## `x`
    /// the distance on the horizontal axis
    /// ## `y`
    /// the distance on the vertical axis
    fn set_threshold_trigger_distance(&self, x: f32, y: f32);

    /// Sets the edge trigger for the gesture drag threshold, if any.
    ///
    /// This function should only be called by sub-classes of
    /// `GestureAction` during their construction phase.
    /// ## `edge`
    /// the `GestureTriggerEdge`
    fn set_threshold_trigger_edge(&self, edge: GestureTriggerEdge);

    /// The horizontal trigger distance to be used by the action to either
    /// emit the `GestureAction::gesture-begin` signal or to emit
    /// the `GestureAction::gesture-cancel` signal.
    ///
    /// A negative value will be interpreted as the default drag threshold.
    fn get_property_threshold_trigger_distance_x(&self) -> f32;

    /// The vertical trigger distance to be used by the action to either
    /// emit the `GestureAction::gesture-begin` signal or to emit
    /// the `GestureAction::gesture-cancel` signal.
    ///
    /// A negative value will be interpreted as the default drag threshold.
    fn get_property_threshold_trigger_distance_y(&self) -> f32;

    /// The ::gesture_begin signal is emitted when the `Actor` to which
    /// a `GestureAction` has been applied starts receiving a gesture.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    ///
    /// # Returns
    ///
    /// `true` if the gesture should start, and `false` if
    ///  the gesture should be ignored.
    fn connect_gesture_begin<F: Fn(&Self, &Actor) -> bool + 'static>(&self, f: F) -> HandlerId;

    /// The ::gesture-cancel signal is emitted when the ongoing gesture gets
    /// cancelled from the `GestureAction::gesture-progress` signal handler.
    ///
    /// This signal is emitted if and only if the `GestureAction::gesture-begin`
    /// signal has been emitted first.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_gesture_cancel<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    /// The ::gesture-end signal is emitted at the end of the gesture gesture,
    /// when the pointer's button is released
    ///
    /// This signal is emitted if and only if the `GestureAction::gesture-begin`
    /// signal has been emitted first.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_gesture_end<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    /// The ::gesture-progress signal is emitted for each motion event after
    /// the `GestureAction::gesture-begin` signal has been emitted.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    ///
    /// # Returns
    ///
    /// `true` if the gesture should continue, and `false` if
    ///  the gesture should be cancelled.
    fn connect_gesture_progress<F: Fn(&Self, &Actor) -> bool + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_n_touch_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<GestureAction>> GestureActionExt for O {
    fn cancel(&self) {
        // unsafe {
        //     ffi::gesture_action_cancel(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn get_device(&self, point: u32) -> Option<InputDevice> {
        // unsafe {
        //     from_glib_none(ffi::gesture_action_get_device(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //     ))
        // }
        unimplemented!()
    }

    fn get_last_event(&self, point: u32) -> Option<Event> {
        // unsafe {
        //     from_glib_none(ffi::gesture_action_get_last_event(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //     ))
        // }
        unimplemented!()
    }

    fn get_motion_coords(&self, point: u32) -> (f32, f32) {
        // unsafe {
        //     let mut motion_x = mem::MaybeUninit::uninit();
        //     let mut motion_y = mem::MaybeUninit::uninit();
        //     ffi::gesture_action_get_motion_coords(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         motion_x.as_mut_ptr(),
        //         motion_y.as_mut_ptr(),
        //     );
        //     let motion_x = motion_x.assume_init();
        //     let motion_y = motion_y.assume_init();
        //     (motion_x, motion_y)
        // }
        unimplemented!()
    }

    fn get_motion_delta(&self, point: u32) -> (f32, f32, f32) {
        // unsafe {
        //     let mut delta_x = mem::MaybeUninit::uninit();
        //     let mut delta_y = mem::MaybeUninit::uninit();
        //     let ret = ffi::gesture_action_get_motion_delta(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         delta_x.as_mut_ptr(),
        //         delta_y.as_mut_ptr(),
        //     );
        //     let delta_x = delta_x.assume_init();
        //     let delta_y = delta_y.assume_init();
        //     (ret, delta_x, delta_y)
        // }
        unimplemented!()
    }

    fn get_n_current_points(&self) -> u32 {
        // unsafe { ffi::gesture_action_get_n_current_points(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_n_touch_points(&self) -> i32 {
        // unsafe { ffi::gesture_action_get_n_touch_points(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_press_coords(&self, point: u32) -> (f32, f32) {
        // unsafe {
        //     let mut press_x = mem::MaybeUninit::uninit();
        //     let mut press_y = mem::MaybeUninit::uninit();
        //     ffi::gesture_action_get_press_coords(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         press_x.as_mut_ptr(),
        //         press_y.as_mut_ptr(),
        //     );
        //     let press_x = press_x.assume_init();
        //     let press_y = press_y.assume_init();
        //     (press_x, press_y)
        // }
        unimplemented!()
    }

    fn get_release_coords(&self, point: u32) -> (f32, f32) {
        // unsafe {
        //     let mut release_x = mem::MaybeUninit::uninit();
        //     let mut release_y = mem::MaybeUninit::uninit();
        //     ffi::gesture_action_get_release_coords(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         release_x.as_mut_ptr(),
        //         release_y.as_mut_ptr(),
        //     );
        //     let release_x = release_x.assume_init();
        //     let release_y = release_y.assume_init();
        //     (release_x, release_y)
        // }
        unimplemented!()
    }

    fn get_sequence(&self, point: u32) -> Option<EventSequence> {
        // unsafe {
        //     from_glib_none(ffi::gesture_action_get_sequence(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //     ))
        // }
        unimplemented!()
    }

    fn get_threshold_trigger_distance(&self) -> (f32, f32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     ffi::gesture_action_get_threshold_trigger_distance(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     (x, y)
        // }
        unimplemented!()
    }

    fn get_threshold_trigger_edge(&self) -> GestureTriggerEdge {
        // unsafe {
        //     from_glib(ffi::gesture_action_get_threshold_trigger_edge(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_velocity(&self, point: u32) -> (f32, f32, f32) {
        // unsafe {
        //     let mut velocity_x = mem::MaybeUninit::uninit();
        //     let mut velocity_y = mem::MaybeUninit::uninit();
        //     let ret = ffi::gesture_action_get_velocity(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         velocity_x.as_mut_ptr(),
        //         velocity_y.as_mut_ptr(),
        //     );
        //     let velocity_x = velocity_x.assume_init();
        //     let velocity_y = velocity_y.assume_init();
        //     (ret, velocity_x, velocity_y)
        // }
        unimplemented!()
    }

    fn set_n_touch_points(&self, nb_points: i32) {
        // unsafe {
        //     ffi::gesture_action_set_n_touch_points(
        //         self.as_ref().to_glib_none().0,
        //         nb_points,
        //     );
        // }
        unimplemented!()
    }

    fn set_threshold_trigger_distance(&self, x: f32, y: f32) {
        // unsafe {
        //     ffi::gesture_action_set_threshold_trigger_distance(
        //         self.as_ref().to_glib_none().0,
        //         x,
        //         y,
        //     );
        // }
        unimplemented!()
    }

    fn set_threshold_trigger_edge(&self, edge: GestureTriggerEdge) {
        // unsafe {
        //     ffi::gesture_action_set_threshold_trigger_edge(
        //         self.as_ref().to_glib_none().0,
        //         edge.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn get_property_threshold_trigger_distance_x(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"threshold-trigger-distance-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `threshold-trigger-distance-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_threshold_trigger_distance_y(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"threshold-trigger-distance-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `threshold-trigger-distance-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn connect_gesture_begin<F: Fn(&Self, &Actor) -> bool + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_gesture_cancel<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_gesture_end<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_gesture_progress<F: Fn(&Self, &Actor) -> bool + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_n_touch_points_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for GestureAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureAction")
    }
}
