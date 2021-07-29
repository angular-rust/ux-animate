use super::{Actor, HandlerId};
use crate::prelude::*;
use std::fmt;

// typedef struct _DropTarget {
//     ClutterActor *stage;

//     gulong capture_id;

//     GHashTable *actions;

//     ClutterDropAction *last_action;
//   } DropTarget;

// SECTION:clutter-drop-action
// @Title: DropAction
// @short_description: An action for drop targets
//
// #DropAction is a #Action that allows a #Actor
// implementation to control what happens when an actor dragged using
// a #DragAction crosses the target area or when a dragged actor
// is released (or "dropped") on the target area.
//
// A trivial use of #DropAction consists in connecting to the
// #DropAction::drop signal and handling the drop from there,
// for instance:
//
// |[<!-- language="C" -->
//   Action *action = clutter_drop_action ();
//
//   g_signal_connect (action, "drop", G_CALLBACK (on_drop), NULL);
//   clutter_actor_add_action (an_actor, action);
// ]|
//
// The #DropAction::can-drop can be used to control whether the
// #DropAction::drop signal is going to be emitted; returning %FALSE
// from a handler connected to the #DropAction::can-drop signal will
// cause the #DropAction::drop signal to be skipped when the input
// device button is released.
//
// It's important to note that #DropAction will only work with
// actors dragged using #DragAction.
//
// See [drop-action.c](https://git.gnome.org/browse/clutter/tree/examples/drop-action.c?h=clutter-1.18)
// for an example of how to use #DropAction.
// @extends Action, ActorMeta
#[derive(Default, Debug, Clone)]
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
        Default::default()
    }
}

impl Object for DropAction {}
impl Is<DropAction> for DropAction {}

impl AsRef<DropAction> for DropAction {
    fn as_ref(&self) -> &DropAction {
        self
    }
}

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
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(&self, f: F)
        -> HandlerId;

    /// The ::drop signal is emitted when the dragged actor is dropped
    /// on `actor`. This signal is only emitted if at least an handler of
    /// `DropAction::can-drop` returns `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> HandlerId;

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
    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> HandlerId;

    /// The ::over-in signal is emitted when the dragged actor crosses
    /// into `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    /// The ::over-out signal is emitted when the dragged actor crosses
    /// outside `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<DropAction>> DropActionExt for O {
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DropAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DropAction")
    }
}
