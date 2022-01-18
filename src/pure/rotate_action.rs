use super::{Actor, HandlerId};
use crate::prelude::*;
use std::fmt;

// @extends GestureAction, Action, ActorMeta
#[derive(Default, Debug, Clone)]
pub struct RotateAction {}

impl RotateAction {
    /// Creates a new `RotateAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `RotateAction`
    pub fn new() -> RotateAction {
        // unsafe { Action::from_glib_none(ffi::rotate_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for RotateAction {}
impl Is<RotateAction> for RotateAction {}

impl AsRef<RotateAction> for RotateAction {
    fn as_ref(&self) -> &RotateAction {
        self
    }
}

/// Trait containing all `RotateAction` methods.
///
/// # Implementors
///
/// [`RotateAction`](struct.RotateAction.html)
pub trait RotateActionExt: 'static {
    /// The ::rotate signal is emitted when a rotate gesture is
    /// recognized on the attached actor and when the gesture is
    /// cancelled (in this case with an angle value of 0).
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `angle`
    /// the difference of angle of rotation between the initial
    /// rotation and the current rotation
    ///
    /// # Returns
    ///
    /// `true` if the rotation should continue, and `false` if
    ///  the rotation should be cancelled.
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<RotateAction>> RotateActionExt for O {
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for RotateAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotateAction")
    }
}
