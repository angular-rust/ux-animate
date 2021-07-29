use super::{Actor, HandlerId, SwipeDirection};
use crate::prelude::*;
use std::fmt;

// SECTION:clutter-swipe-action
// @Title: ClutterSwipeAction
// @Short_Description: Action for swipe gestures
//
// #ClutterSwipeAction is a sub-class of #ClutterGestureAction that implements
// the logic for recognizing swipe gestures.
// @extends GestureAction, Action, ActorMeta
#[derive(Default, Debug, Clone)]
pub struct SwipeAction {
    h_direction: SwipeDirection,
    v_direction: SwipeDirection,

    distance_x: f32,
    distance_y: f32,
}

impl SwipeAction {
    /// Creates a new `SwipeAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `SwipeAction`
    pub fn new() -> SwipeAction {
        Default::default()
    }
}

impl Object for SwipeAction {}
impl Is<SwipeAction> for SwipeAction {}

impl AsRef<SwipeAction> for SwipeAction {
    fn as_ref(&self) -> &SwipeAction {
        self
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
    ) -> HandlerId;
}

impl<O: Is<SwipeAction>> SwipeActionExt for O {
    fn connect_swipe<F: Fn(&Self, &Actor, SwipeDirection) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for SwipeAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwipeAction")
    }
}
