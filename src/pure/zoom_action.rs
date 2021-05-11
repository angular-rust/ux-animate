use super::{Actor, InternalPoint, ZoomAxis};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends GestureAction, Action, ActorMeta
#[derive(Debug, Clone)]
pub struct ZoomAction {}

impl ZoomAction {
    /// Creates a new `ZoomAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `ZoomAction`
    pub fn new() -> ZoomAction {
        // unsafe { Action::from_glib_none(ffi::clutter_zoom_action_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Object for ZoomAction {}
impl Is<ZoomAction> for ZoomAction {}

impl AsRef<ZoomAction> for ZoomAction {
    fn as_ref(&self) -> &ZoomAction {
        self
    }
}

impl Default for ZoomAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `ZoomAction` methods.
///
/// # Implementors
///
/// [`ZoomAction`](struct.ZoomAction.html)
pub trait ZoomActionExt: 'static {
    /// Retrieves the focal point of the current zoom
    /// ## `point`
    /// a `Point`
    fn get_focal_point(&self) -> InternalPoint;

    /// Retrieves the focal point relative to the actor's coordinates of
    /// the current zoom
    /// ## `point`
    /// a `Point`
    fn get_transformed_focal_point(&self) -> InternalPoint;

    /// Retrieves the axis constraint set by `ZoomActionExt::set_zoom_axis`
    ///
    /// # Returns
    ///
    /// the axis constraint
    fn get_zoom_axis(&self) -> ZoomAxis;

    /// Restricts the zooming action to a specific axis
    /// ## `axis`
    /// the axis to constraint the zooming to
    fn set_zoom_axis(&self, axis: ZoomAxis);

    /// The ::zoom signal is emitted for each series of touch events that
    /// change the distance and focal point between the touch points.
    ///
    /// The default handler of the signal will call
    /// `ActorExt::set_scale` on `actor` using the ratio of the first
    /// distance between the touch points and the current distance. To
    /// override the default behaviour, connect to this signal and return
    /// `false`.
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `focal_point`
    /// the focal point of the zoom
    /// ## `factor`
    /// the initial distance between the 2 touch points
    ///
    /// # Returns
    ///
    /// `true` if the zoom should continue, and `false` if
    ///  the zoom should be cancelled.
    fn connect_zoom<F: Fn(&Self, &Actor, &InternalPoint, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_zoom_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<ZoomAction>> ZoomActionExt for O {
    fn get_focal_point(&self) -> InternalPoint {
        // unsafe {
        //     let mut point = InternalPoint::uninitialized();
        //     ffi::clutter_zoom_action_get_focal_point(
        //         self.as_ref().to_glib_none().0,
        //         point.to_glib_none_mut().0,
        //     );
        //     point
        // }
        unimplemented!()
    }

    fn get_transformed_focal_point(&self) -> InternalPoint {
        // unsafe {
        //     let mut point = InternalPoint::uninitialized();
        //     ffi::clutter_zoom_action_get_transformed_focal_point(
        //         self.as_ref().to_glib_none().0,
        //         point.to_glib_none_mut().0,
        //     );
        //     point
        // }
        unimplemented!()
    }

    fn get_zoom_axis(&self) -> ZoomAxis {
        // unsafe {
        //     from_glib(ffi::clutter_zoom_action_get_zoom_axis(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_zoom_axis(&self, axis: ZoomAxis) {
        // unsafe {
        //     ffi::clutter_zoom_action_set_zoom_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        // }
        unimplemented!()
    }

    fn connect_zoom<F: Fn(&Self, &Actor, &InternalPoint, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_zoom_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ZoomAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZoomAction")
    }
}
