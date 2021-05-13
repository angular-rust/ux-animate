use super::{Actor, EventSequence, HandlerId, LongPressState, ModifierType};
use crate::prelude::*;
use std::fmt;

// * SECTION:clutter-click-action
// * @Title: ClickAction
// * @Short_Description: Action for clickable actors
// *
// * #ClickAction is a sub-class of #Action that implements
// * the logic for clickable actors, by using the low level events of
// * #Actor, such as #Actor::button-press-event and
// * #Actor::button-release-event, to synthesize the high level
// * #ClickAction::clicked signal.
// *
// * To use #ClickAction you just need to apply it to a #Actor
// * using clutter_actor_add_action() and connect to the
// * #ClickAction::clicked signal:
// *
// * |[
// *   Action *action = clutter_click_action_new ();
// *
// *   clutter_actor_add_action (actor, action);
// *
// *   g_signal_connect (action, "clicked", G_CALLBACK (on_clicked), NULL);
// * ]|
// *
// * #ClickAction also supports long press gestures: a long press is
// * activated if the pointer remains pressed within a certain threshold (as
// * defined by the #ClickAction:long-press-threshold property) for a
// * minimum amount of time (as the defined by the
// * #ClickAction:long-press-duration property).
// * The #ClickAction::long-press signal is emitted multiple times,
// * using different #LongPressState values; to handle long presses
// * you should connect to the #ClickAction::long-press signal and
// * handle the different states:
// *
// * |[
// *   static gboolean
// *   on_long_press (ClickAction    *action,
// *                  Actor          *actor,
// *                  LongPressState  state)
// *   {
// *     switch (state)
// *       {
// *       case CLUTTER_LONG_PRESS_QUERY:
// *         /&ast; return TRUE if the actor should support long press
// *          &ast; gestures, and FALSE otherwise; this state will be
// *          &ast; emitted on button presses
// *          &ast;/
// *         return TRUE;
// *
// *       case CLUTTER_LONG_PRESS_ACTIVATE:
// *         /&ast; this state is emitted if the minimum duration has
// *          &ast; been reached without the gesture being cancelled.
// *          &ast; the return value is not used
// *          &ast;/
// *         return TRUE;
// *
// *       case CLUTTER_LONG_PRESS_CANCEL:
// *         /&ast; this state is emitted if the long press was cancelled;
// *          &ast; for instance, the pointer went outside the actor or the
// *          &ast; allowed threshold, or the button was released before
// *          &ast; the minimum duration was reached. the return value is
// *          &ast; not used
// *          &ast;/
// *         return FALSE;
// *       }
// *   }
// * ]|
// *
// @extends Action, ActorMeta
#[derive(Default, Debug, Clone)]
pub struct ClickAction {
    stage: Option<Actor>,

    event_id: u32,
    capture_id: u32,
    long_press_id: u32,

    long_press_threshold: i32,
    long_press_duration: i32,
    drag_threshold: i32,

    press_button: u32,
    press_device_id: i32,
    press_sequence: Option<EventSequence>,
    modifier_state: ModifierType,
    press_x: f32,
    press_y: f32,

    is_held: bool,
    is_pressed: bool,
}

impl ClickAction {
    /// Creates a new `ClickAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `ClickAction`
    pub fn new() -> ClickAction {
        Default::default()
    }
}

impl Object for ClickAction {}
impl Is<ClickAction> for ClickAction {}

impl AsRef<ClickAction> for ClickAction {
    fn as_ref(&self) -> &ClickAction {
        self
    }
}

/// Trait containing all `ClickAction` methods.
///
/// # Implementors
///
/// [`ClickAction`](struct.ClickAction.html)
pub trait ClickActionExt: 'static {
    /// Retrieves the button that was pressed.
    ///
    /// - 1 - left mouse button in a right-handed configuration, or the right mouse button in a left-handed configuration
    /// - 2 - scroll wheel button
    /// - 3 - right mouse button in a right-handed configuration, or the left mouse button in a left-handed configuration
    ///
    /// # Returns
    ///
    /// the button value
    fn get_button(&self) -> u32;

    /// Retrieves the screen coordinates of the button press.
    /// ## `press_x`
    /// return location for the X coordinate, or `None`
    /// ## `press_y`
    /// return location for the Y coordinate, or `None`
    fn get_coords(&self) -> (f32, f32);

    /// Retrieves the modifier state of the click action.
    ///
    /// # Returns
    ///
    /// the modifier state parameter, or 0
    fn get_state(&self) -> ModifierType;

    /// Emulates a release of the pointer button, which ungrabs the pointer
    /// and unsets the `ClickAction:pressed` state.
    ///
    /// This function will also cancel the long press gesture if one was
    /// initiated.
    ///
    /// This function is useful to break a grab, for instance after a certain
    /// amount of time has passed.
    fn release(&self);

    /// Whether the clickable actor has the pointer grabbed
    fn get_property_held(&self) -> bool;

    /// The minimum duration of a press for it to be recognized as a long
    /// press gesture, in milliseconds.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:long-press-duration` property.
    fn get_property_long_press_duration(&self) -> i32;

    /// The minimum duration of a press for it to be recognized as a long
    /// press gesture, in milliseconds.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:long-press-duration` property.
    fn set_property_long_press_duration(&self, long_press_duration: i32);

    /// The maximum allowed distance that can be covered (on both axes) before
    /// a long press gesture is cancelled, in pixels.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:dnd-drag-threshold` property.
    fn get_property_long_press_threshold(&self) -> i32;

    /// The maximum allowed distance that can be covered (on both axes) before
    /// a long press gesture is cancelled, in pixels.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:dnd-drag-threshold` property.
    fn set_property_long_press_threshold(&self, long_press_threshold: i32);

    /// Whether the clickable actor should be in "pressed" state
    fn get_property_pressed(&self) -> bool;

    /// The ::clicked signal is emitted when the `Actor` to which
    /// a `ClickAction` has been applied should respond to a
    /// pointer button press and release events
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    /// The ::long-press signal is emitted during the long press gesture
    /// handling.
    ///
    /// This signal can be emitted multiple times with different states.
    ///
    /// The `LongPressState::Query` state will be emitted on button presses,
    /// and its return value will determine whether the long press handling
    /// should be initiated. If the signal handlers will return `true`, the
    /// `LongPressState::Query` state will be followed either by a signal
    /// emission with the `LongPressState::Activate` state if the long press
    /// constraints were respected, or by a signal emission with the
    /// `LongPressState::Cancel` state if the long press was cancelled.
    ///
    /// It is possible to forcibly cancel a long press detection using
    /// `ClickActionExt::release`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `state`
    /// the long press state
    ///
    /// # Returns
    ///
    /// Only the `LongPressState::Query` state uses the
    ///  returned value of the handler; other states will ignore it
    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<ClickAction>> ClickActionExt for O {
    fn get_button(&self) -> u32 {
        let action = self.as_ref();
        action.press_button
    }

    fn get_coords(&self) -> (f32, f32) {
        let action = self.as_ref();
        (action.press_x, action.press_y)
    }

    fn get_state(&self) -> ModifierType {
        let action = self.as_ref();
        action.modifier_state
    }

    fn release(&self) {
        unimplemented!()
    }

    fn get_property_held(&self) -> bool {
        unimplemented!()
    }

    fn get_property_long_press_duration(&self) -> i32 {
        unimplemented!()
    }

    fn set_property_long_press_duration(&self, long_press_duration: i32) {
        unimplemented!()
    }

    fn get_property_long_press_threshold(&self) -> i32 {
        unimplemented!()
    }

    fn set_property_long_press_threshold(&self, long_press_threshold: i32) {
        unimplemented!()
    }

    fn get_property_pressed(&self) -> bool {
        unimplemented!()
    }

    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ClickAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClickAction")
    }
}
