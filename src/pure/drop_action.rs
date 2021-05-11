use crate::prelude::*;
use super::{Action, Actor, ActorMeta};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// * SECTION:clutter-drop-action
// * @Title: ClutterDropAction
// * @short_description: An action for drop targets
// *
// * #ClutterDropAction is a #ClutterAction that allows a #ClutterActor
// * implementation to control what happens when an actor dragged using
// * a #ClutterDragAction crosses the target area or when a dragged actor
// * is released (or "dropped") on the target area.
// *
// * A trivial use of #ClutterDropAction consists in connecting to the
// * #ClutterDropAction::drop signal and handling the drop from there,
// * for instance:
// *
// * |[<!-- language="C" -->
// *   ClutterAction *action = clutter_drop_action ();
// *
// *   g_signal_connect (action, "drop", G_CALLBACK (on_drop), NULL);
// *   clutter_actor_add_action (an_actor, action);
// * ]|
// *
// * The #ClutterDropAction::can-drop can be used to control whether the
// * #ClutterDropAction::drop signal is going to be emitted; returning %FALSE
// * from a handler connected to the #ClutterDropAction::can-drop signal will
// * cause the #ClutterDropAction::drop signal to be skipped when the input
// * device button is released.
// *
// * It's important to note that #ClutterDropAction will only work with
// * actors dragged using #ClutterDragAction.
// *
// * See [drop-action.c](https://git.gnome.org/browse/clutter/tree/examples/drop-action.c?h=clutter-1.18)
// * for an example of how to use #ClutterDropAction.
// @extends Action, ActorMeta
#[derive(Debug, Clone)]
pub struct DropAction {
    actor: Option<Actor>,
    stage: Option<Actor>,
  
    mapped_id: u64,
}

impl DropAction {
    /// Creates a new `DropAction`.
    ///
    /// Use `ActorExt::add_action` to add the action to a `Actor`.
    ///
    /// # Returns
    ///
    /// the newly created `DropAction`
    pub fn new() -> DropAction {
        // unsafe { Action::from_glib_none(ffi::clutter_drop_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for DropAction {
    fn default() -> Self {
        Self::new()
    }
}

impl Object for DropAction {}

/// Trait containing all `DropAction` methods.
///
/// # Implementors
///
/// [`DropAction`](struct.DropAction.html)
pub trait DropActionExt: 'static {
    /// The ::can-drop signal is emitted when the dragged actor is dropped
    /// on `actor`. The return value of the ::can-drop signal will determine
    /// whether or not the `DropAction::drop` signal is going to be
    /// emitted on `action`.
    ///
    /// The default implementation of `DropAction` returns `true` for
    /// this signal.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    ///
    /// # Returns
    ///
    /// `true` if the drop is accepted, and `false` otherwise
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drop signal is emitted when the dragged actor is dropped
    /// on `actor`. This signal is only emitted if at least an handler of
    /// `DropAction::can-drop` returns `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::drop-cancel signal is emitted when the drop is refused
    /// by an emission of the `DropAction::can-drop` signal.
    ///
    /// After the ::drop-cancel signal is fired the active drag is
    /// terminated.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::over-in signal is emitted when the dragged actor crosses
    /// into `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::over-out signal is emitted when the dragged actor crosses
    /// outside `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<DropAction>> DropActionExt for O {
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DropAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DropAction")
    }
}
