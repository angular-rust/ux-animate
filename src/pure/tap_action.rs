use crate::prelude::*;
use super::{Action, Actor, ActorMeta, GestureAction};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// @extends GestureAction, Action, ActorMeta
#[derive(Debug, Clone)]
pub struct TapAction {
}

impl TapAction {
    /// Creates a new `TapAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `TapAction`
    pub fn new() -> TapAction {
        // unsafe { Action::from_glib_none(ffi::clutter_tap_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for TapAction {}
impl Is<TapAction> for TapAction {}

impl AsRef<TapAction> for TapAction {
    fn as_ref(&self) -> &TapAction {
        self
    }
}

impl Default for TapAction {
    fn default() -> Self {
        Self::new()
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
    fn connect_tap<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<TapAction>> TapActionExt for O {
    fn connect_tap<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for TapAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TapAction")
    }
}
