use super::{InternalPoint, InternalRect, ScrollMode};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable, @implements Animatable, Container
// @extends Actor
#[derive(Debug, Clone)]
pub struct ScrollActor {}

impl ScrollActor {
    /// Creates a new `ScrollActor`.
    ///
    /// # Returns
    ///
    /// The newly created `ScrollActor`
    ///  instance.
    pub fn new() -> ScrollActor {
        // unsafe { Actor::from_glib_none(ffi::clutter_scroll_actor_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for ScrollActor {}
impl Is<ScrollActor> for ScrollActor {}

impl AsRef<ScrollActor> for ScrollActor {
    fn as_ref(&self) -> &ScrollActor {
        self
    }
}

impl Default for ScrollActor {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `ScrollActor` methods.
///
/// # Implementors
///
/// [`ScrollActor`](struct.ScrollActor.html)
pub trait ScrollActorExt: 'static {
    /// Retrieves the `ScrollActor:scroll-mode` property
    ///
    /// # Returns
    ///
    /// the scrolling mode
    fn get_scroll_mode(&self) -> ScrollMode;

    /// Scrolls the contents of `self` so that `point` is the new origin
    /// of the visible area.
    ///
    /// The coordinates of `point` must be relative to the `self`.
    ///
    /// This function will use the currently set easing state of the `self`
    /// to transition from the current scroll origin to the new one.
    /// ## `point`
    /// a `Point`
    fn scroll_to_point(&self, point: &InternalPoint);

    /// Scrolls `self` so that `rect` is in the visible portion.
    /// ## `rect`
    /// a `Rect`
    fn scroll_to_rect(&self, rect: &InternalRect);

    /// Sets the `ScrollActor:scroll-mode` property.
    /// ## `mode`
    /// a `ScrollMode`
    fn set_scroll_mode(&self, mode: ScrollMode);

    fn connect_property_scroll_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ScrollActor>> ScrollActorExt for O {
    fn get_scroll_mode(&self) -> ScrollMode {
        // unsafe {
        //     from_glib(ffi::clutter_scroll_actor_get_scroll_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn scroll_to_point(&self, point: &InternalPoint) {
        // unsafe {
        //     ffi::clutter_scroll_actor_scroll_to_point(
        //         self.as_ref().to_glib_none().0,
        //         point.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn scroll_to_rect(&self, rect: &InternalRect) {
        // unsafe {
        //     ffi::clutter_scroll_actor_scroll_to_rect(
        //         self.as_ref().to_glib_none().0,
        //         rect.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_scroll_mode(&self, mode: ScrollMode) {
        // unsafe {
        //     ffi::clutter_scroll_actor_set_scroll_mode(
        //         self.as_ref().to_glib_none().0,
        //         mode.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_scroll_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ScrollActor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollActor")
    }
}
