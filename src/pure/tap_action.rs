use super::{Actor, GestureAction, HandlerId};
use crate::prelude::*;
use std::fmt;

// @Title: TapAction
// @Short_Description: Action for tap gestures
//
// #TapAction is a sub-class of #GestureAction that implements
// the logic for recognizing mouse clicks and touch tap gestures.
//
// The simplest usage of #TapAction consists in adding it to
// a #Actor, setting it as reactive and connecting a
// callback for the #TapAction::tap signal, along the lines of the
// following code:
//
// ```
//   actor_add_action (actor, tap_action_new ());
//   actor_set_reactive (actor, TRUE);
//   g_signal_connect (action, "tap", G_CALLBACK (on_tap_callback), NULL);
// ```
//
// Since: 1.14
// @extends GestureAction, Action, ActorMeta
#[derive(Default, Debug)]
pub struct TapAction {
    inner: GestureAction,
}

impl TapAction {
    /// Creates a new `TapAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `TapAction`
    pub fn new() -> TapAction {
        Default::default()
    }
}

impl Object for TapAction {}
impl Is<TapAction> for TapAction {}

impl AsRef<TapAction> for TapAction {
    fn as_ref(&self) -> &TapAction {
        self
    }
}

/// Trait containing all `TapAction` methods.
///
/// # Implementors
///
/// [`TapAction`](struct.TapAction.html)
pub trait TapActionExt: 'static {
    /// The ::tap signal is emitted when the tap gesture is complete.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_tap<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<TapAction>> TapActionExt for O {
    fn connect_tap<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for TapAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TapAction")
    }
}
