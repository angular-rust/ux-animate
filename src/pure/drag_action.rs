use crate::prelude::*;
use super::{Action, Actor, ActorMeta, DragAxis, EventSequence, InputDevice, ModifierType, Stage};
use crate::Rect;
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

// * @Title: ClutterDragAction
// * @Short_Description: Action enabling dragging on actors
// *
// * #ClutterDragAction is a sub-class of #ClutterAction that implements
// * all the necessary logic for dragging actors.
// *
// * The simplest usage of #ClutterDragAction consists in adding it to
// * a #ClutterActor and setting it as reactive; for instance, the following
// * code:
// *
// * |[<!-- language="C" -->
// *   clutter_actor_add_action (actor, clutter_drag_action_new ());
// *   clutter_actor_set_reactive (actor, TRUE);
// * ]|
// *
// * will automatically result in the actor moving to follow the pointer
// * whenever the pointer's button is pressed over the actor and moved
// * across the stage.
// *
// * The #ClutterDragAction will signal the begin and the end of a dragging
// * through the #ClutterDragAction::drag-begin and #ClutterDragAction::drag-end
// * signals, respectively. Each pointer motion during a drag will also result
// * in the #ClutterDragAction::drag-motion signal to be emitted.
// *
// * It is also possible to set another #ClutterActor as the dragged actor
// * by calling clutter_drag_action_set_drag_handle() from within a handle
// * of the #ClutterDragAction::drag-begin signal. The drag handle must be
// * parented and exist between the emission of #ClutterDragAction::drag-begin
// * and #ClutterDragAction::drag-end.
// *
// * The [drag-action example](https://git.gnome.org/browse/clutter/tree/examples/drag-action.c?h=clutter-1.18)
// * allows dragging the rectangle around the stage using a #ClutterDragAction.
// * When pressing the `Shift` key the actor that is being dragged will be a
// * separate rectangle, and when the drag ends, the original rectangle will be
// * animated to the final drop coordinates.
// *
// @extends Action, ActorMeta,
#[derive(Debug, Clone)]
pub struct DragAction {
    stage: Option<Stage>,

    x_drag_threshold: i32,
    y_drag_threshold: i32,
    drag_handle: Option<Actor>,
    drag_axis: DragAxis,
    drag_area: Rect<u32>,

    device: Option<InputDevice>,
    sequence: Option<EventSequence>,
    button_press_id: u64,
    touch_begin_id: u64,
    capture_id: u64,

    press_x: f32,
    press_y: f32,
    press_state: ModifierType,

    last_motion_x: f32,
    last_motion_y: f32,
    last_motion_state: ModifierType,
    last_motion_device: Option<InputDevice>,

    transformed_press_x: f32,
    transformed_press_y: f32,

    emit_delayed_press: bool,
    in_drag: bool,
    motion_events_enabled: bool,
    drag_area_set: bool,
}

impl DragAction {
    /// Creates a new `DragAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `DragAction`
    pub fn new() -> DragAction {
        unimplemented!()
    }
}

impl Object for DragAction {}

impl Default for DragAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `DragAction` methods.
///
/// # Implementors
///
/// [`DragAction`](struct.DragAction.html)
pub trait DragActionExt: 'static {
    /// Retrieves the "drag area" associated with `self`, that
    /// is a `Rect` that constrains the actor movements,
    /// in parents coordinates.
    /// ## `drag_area`
    /// a `Rect` to be filled
    ///
    /// # Returns
    ///
    /// `true` if the actor is actually constrained (and thus
    ///  `drag_area` is valid), `false` otherwise
    fn get_drag_area(&self) -> Option<Rect<u32>>;

    /// Retrieves the axis constraint set by `DragActionExt::set_drag_axis`
    ///
    /// # Returns
    ///
    /// the axis constraint
    fn get_drag_axis(&self) -> DragAxis;

    /// Retrieves the drag handle set by `DragActionExt::set_drag_handle`
    ///
    /// # Returns
    ///
    /// a `Actor`, used as the drag
    ///  handle, or `None` if none was set
    fn get_drag_handle(&self) -> Option<Actor>;

    /// Retrieves the values set by `DragActionExt::set_drag_threshold`.
    ///
    /// If the `DragAction:x-drag-threshold` property or the
    /// `DragAction:y-drag-threshold` property have been set to -1 then
    /// this function will return the default drag threshold value as stored
    /// by the `Settings:dnd-drag-threshold` property of `Settings`.
    /// ## `x_threshold`
    /// return location for the horizontal drag
    ///  threshold value, in pixels
    /// ## `y_threshold`
    /// return location for the vertical drag
    ///  threshold value, in pixels
    fn get_drag_threshold(&self) -> (u32, u32);

    /// Retrieves the coordinates, in stage space, of the latest motion
    /// event during the dragging
    /// ## `motion_x`
    /// return location for the latest motion
    ///  event's X coordinate
    /// ## `motion_y`
    /// return location for the latest motion
    ///  event's Y coordinate
    fn get_motion_coords(&self) -> (f32, f32);

    /// Retrieves the coordinates, in stage space, of the press event
    /// that started the dragging
    /// ## `press_x`
    /// return location for the press event's X coordinate
    /// ## `press_y`
    /// return location for the press event's Y coordinate
    fn get_press_coords(&self) -> (f32, f32);

    /// Sets `drag_area` to constrain the dragging of the actor associated
    /// with `self`, so that it position is always within `drag_area`, expressed
    /// in parent's coordinates.
    /// If `drag_area` is `None`, the actor is not constrained.
    /// ## `drag_area`
    /// a `Rect`
    fn set_drag_area(&self, drag_area: Option<Rect<u32>>);

    /// Restricts the dragging action to a specific axis
    /// ## `axis`
    /// the axis to constraint the dragging to
    fn set_drag_axis(&self, axis: DragAxis);

    /// Sets the actor to be used as the drag handle.
    /// ## `handle`
    /// a `Actor`, or `None` to unset
    fn set_drag_handle<P: Is<Actor>>(&self, handle: Option<&P>);

    /// Sets the horizontal and vertical drag thresholds that must be
    /// cleared by the pointer before `self` can begin the dragging.
    ///
    /// If `x_threshold` or `y_threshold` are set to -1 then the default
    /// drag threshold stored in the `Settings:dnd-drag-threshold`
    /// property of `Settings` will be used.
    /// ## `x_threshold`
    /// a distance on the horizontal axis, in pixels, or
    ///  -1 to use the default drag threshold from `Settings`
    /// ## `y_threshold`
    /// a distance on the vertical axis, in pixels, or
    ///  -1 to use the default drag threshold from `Settings`
    fn set_drag_threshold(&self, x_threshold: i32, y_threshold: i32);

    /// Whether the `DragAction:drag-area` property has been set.
    fn get_property_drag_area_set(&self) -> bool;

    /// The horizontal threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// horizontally at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the default threshold
    /// stored in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn get_property_x_drag_threshold(&self) -> i32;

    /// The horizontal threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// horizontally at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the default threshold
    /// stored in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn set_property_x_drag_threshold(&self, x_drag_threshold: i32);

    /// The vertical threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// vertically at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the value stored
    /// in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn get_property_y_drag_threshold(&self) -> i32;

    /// The vertical threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// vertically at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the value stored
    /// in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn set_property_y_drag_threshold(&self, y_drag_threshold: i32);

    /// The ::drag-begin signal is emitted when the `DragAction`
    /// starts the dragging
    ///
    /// The emission of this signal can be delayed by using the
    /// `DragAction:x-drag-threshold` and
    /// `DragAction:y-drag-threshold` properties
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `event_x`
    /// the X coordinate (in stage space) of the press event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the press event
    /// ## `modifiers`
    /// the modifiers of the press event
    fn connect_drag_begin<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-end signal is emitted at the end of the dragging,
    /// when the pointer button's is released
    ///
    /// This signal is emitted if and only if the `DragAction::drag-begin`
    /// signal has been emitted first
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `event_x`
    /// the X coordinate (in stage space) of the release event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the release event
    /// ## `modifiers`
    /// the modifiers of the release event
    fn connect_drag_end<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-motion signal is emitted for each motion event after
    /// the `DragAction::drag-begin` signal has been emitted.
    ///
    /// The components of the distance between the press event and the
    /// latest motion event are computed in the actor's coordinate space,
    /// to take into account eventual transformations. If you want the
    /// stage coordinates of the latest motion event you can use
    /// `DragActionExt::get_motion_coords`.
    ///
    /// The default handler of the signal will call `ActorExt::move_by`
    /// either on `actor` or, if set, of `DragAction:drag-handle` using
    /// the `delta_x` and `delta_y` components of the dragging motion. If you
    /// want to override the default behaviour, you can connect to the
    /// `DragAction::drag-progress` signal and return `false` from the
    /// handler.
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `delta_x`
    /// the X component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    /// ## `delta_y`
    /// the Y component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    fn connect_drag_motion<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-progress signal is emitted for each motion event after
    /// the `DragAction::drag-begin` signal has been emitted.
    ///
    /// The components of the distance between the press event and the
    /// latest motion event are computed in the actor's coordinate space,
    /// to take into account eventual transformations. If you want the
    /// stage coordinates of the latest motion event you can use
    /// `DragActionExt::get_motion_coords`.
    ///
    /// The default handler will emit `DragAction::drag-motion`,
    /// if `DragAction::drag-progress` emission returns `true`.
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `delta_x`
    /// the X component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    /// ## `delta_y`
    /// the Y component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    ///
    /// # Returns
    ///
    /// `true` if the drag should continue, and `false`
    ///  if it should be stopped.
    fn connect_drag_progress<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_drag_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_drag_area_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_drag_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_drag_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_y_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<DragAction>> DragActionExt for O {
    fn get_drag_area(&self) -> Option<Rect<u32>> {
        unimplemented!()
    }

    fn get_drag_axis(&self) -> DragAxis {
        unimplemented!()
    }

    fn get_drag_handle(&self) -> Option<Actor> {
        unimplemented!()
    }

    fn get_drag_threshold(&self) -> (u32, u32) {
        unimplemented!()
    }

    fn get_motion_coords(&self) -> (f32, f32) {
        unimplemented!()
    }

    fn get_press_coords(&self) -> (f32, f32) {
        unimplemented!()
    }

    fn set_drag_area(&self, drag_area: Option<Rect<u32>>) {
        unimplemented!()
    }

    fn set_drag_axis(&self, axis: DragAxis) {
        unimplemented!()
    }

    fn set_drag_handle<P: Is<Actor>>(&self, handle: Option<&P>) {
        unimplemented!()
    }

    fn set_drag_threshold(&self, x_threshold: i32, y_threshold: i32) {
        unimplemented!()
    }

    fn get_property_drag_area_set(&self) -> bool {
        unimplemented!()
    }

    fn get_property_x_drag_threshold(&self) -> i32 {
        unimplemented!()
    }

    fn set_property_x_drag_threshold(&self, x_drag_threshold: i32) {
        unimplemented!()
    }

    fn get_property_y_drag_threshold(&self) -> i32 {
        unimplemented!()
    }

    fn set_property_y_drag_threshold(&self, y_drag_threshold: i32) {
        unimplemented!()
    }

    fn connect_drag_begin<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_drag_end<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_drag_motion<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_drag_progress<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_drag_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_drag_area_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_drag_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_drag_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_x_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_y_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DragAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DragAction")
    }
}
