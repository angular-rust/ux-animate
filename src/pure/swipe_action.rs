use crate::prelude::*;
use super::{Action, Actor, ActorMeta, GestureAction, SwipeDirection};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// @extends GestureAction, Action, ActorMeta
#[derive(Debug, Clone)]
pub struct SwipeAction {
}

impl SwipeAction {
    /// Creates a new `SwipeAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `SwipeAction`
    pub fn new() -> SwipeAction {
        // unsafe { Action::from_glib_none(ffi::clutter_swipe_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for SwipeAction {}
impl Is<SwipeAction> for SwipeAction {}

impl AsRef<SwipeAction> for SwipeAction {
    fn as_ref(&self) -> &SwipeAction {
        self
    }
}

impl Default for SwipeAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `SwipeAction` methods.
///
/// # Implementors
///
/// [`SwipeAction`](struct.SwipeAction.html)
pub trait SwipeActionExt: 'static {
    /// The ::swipe signal is emitted when a swipe gesture is recognized on the
    /// attached actor.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `direction`
    /// the main direction of the swipe gesture
    ///
    /// # Returns
    ///
    /// `true` if the swipe should continue, and `false` if
    ///  the swipe should be cancelled.
    fn connect_swipe<F: Fn(&Self, &Actor, SwipeDirection) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: Is<SwipeAction>> SwipeActionExt for O {
    fn connect_swipe<F: Fn(&Self, &Actor, SwipeDirection) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for SwipeAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwipeAction")
    }
}
