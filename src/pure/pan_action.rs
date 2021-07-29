use super::{Actor, HandlerId, PanAxis, Timeline};
use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum PanState {
    Inactive,
    Panning,
    Interpolating,
}

impl Default for PanState {
    fn default() -> Self {
        Self::Inactive
    }
}

#[derive(Debug, Clone)]
pub enum PinState {
    Unknown,
    None,
    Horizontal,
    Vertical,
}

impl Default for PinState {
    fn default() -> Self {
        Self::None
    }
}

// SECTION:clutter-pan-action
// @Title: ClutterPanAction
// @Short_Description: Action for pan gestures
//
// #ClutterPanAction is a sub-class of #ClutterGestureAction that implements
// the logic for recognizing pan gestures.
//
// The simplest usage of #ClutterPanAction consists in adding it to
// a #ClutterActor with a child and setting it as reactive; for instance,
// the following code:
//
// |[
//   clutter_actor_add_action (actor, clutter_pan_action_new ());
//   clutter_actor_set_reactive (actor, TRUE);
// ]|
//
// will automatically result in the actor children to be moved
// when dragging.
// @extends GestureAction, Action, ActorMeta
#[derive(Default, Debug, Clone)]
pub struct PanAction {
    pan_axis: PanAxis,

    state: PanState,

    // Variables for storing acceleration information
    deceleration_timeline: Option<Timeline>,
    target_x: f32,
    target_y: f32,
    dx: f32,
    dy: f32,
    deceleration_rate: f64,
    acceleration_factor: f64,

    // Inertial motion tracking
    interpolated_x: f32,
    interpolated_y: f32,
    release_x: f32,
    release_y: f32,

    should_interpolate: bool,

    pin_state: PinState,
}

impl PanAction {
    /// Creates a new `PanAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `PanAction`
    pub fn new() -> PanAction {
        Default::default()
    }
}

impl Object for PanAction {}
impl Is<PanAction> for PanAction {}

impl AsRef<PanAction> for PanAction {
    fn as_ref(&self) -> &PanAction {
        self
    }
}

/// Trait containing all `PanAction` methods.
///
/// # Implementors
///
/// [`PanAction`](struct.PanAction.html)
pub trait PanActionExt: 'static {
    /// Retrieves the initial acceleration factor for interpolated ::pan events.
    ///
    /// # Returns
    ///
    /// The initial acceleration factor for interpolated events.
    fn get_acceleration_factor(&self) -> f64;

    /// Retrieves the delta, in stage space, dependent on the current state
    /// of the `PanAction`, and respecting the constraint specified by the
    /// `PanAction:pan-axis` property.
    /// ## `point`
    /// the touch point index, with 0 being the first touch
    ///  point received by the action
    /// ## `delta_x`
    /// return location for the X delta
    /// ## `delta_y`
    /// return location for the Y delta
    ///
    /// # Returns
    ///
    /// the distance since last motion event
    fn get_constrained_motion_delta(&self, point: u32) -> (f32, f32, f32);

    /// Retrieves the deceleration rate of interpolated ::pan events.
    ///
    /// # Returns
    ///
    /// The deceleration rate of the interpolated events.
    fn get_deceleration(&self) -> f64;

    /// Checks if the action should emit ::pan events even after releasing
    /// the pointer during a panning gesture, to emulate some kind of
    /// kinetic inertia.
    ///
    /// # Returns
    ///
    /// `true` if interpolated events emission is active.
    fn get_interpolate(&self) -> bool;

    /// Retrieves the coordinates, in stage space, of the latest interpolated
    /// event, analogous to `GestureActionExt::get_motion_coords`.
    /// ## `interpolated_x`
    /// return location for the latest
    ///  interpolated event's X coordinate
    /// ## `interpolated_y`
    /// return location for the latest
    ///  interpolated event's Y coordinate
    fn get_interpolated_coords(&self) -> (f32, f32);

    /// Retrieves the delta, in stage space, since the latest interpolated
    /// event, analogous to `GestureActionExt::get_motion_delta`.
    /// ## `delta_x`
    /// return location for the X delta since
    ///  the latest interpolated event
    /// ## `delta_y`
    /// return location for the Y delta since
    ///  the latest interpolated event
    ///
    /// # Returns
    ///
    /// the distance since the latest interpolated event
    fn get_interpolated_delta(&self) -> (f32, f32, f32);

    /// Retrieves the axis constraint set by `PanActionExt::set_pan_axis`
    ///
    /// # Returns
    ///
    /// the axis constraint
    fn get_pan_axis(&self) -> PanAxis;

    /// Factor applied to the momentum velocity at the time of releasing the
    /// pointer when generating interpolated ::pan events.
    /// ## `factor`
    /// The acceleration factor
    fn set_acceleration_factor(&self, factor: f64);

    /// Sets the deceleration rate of the interpolated ::pan events generated
    /// after a pan gesture. This is approximately the value that the momentum
    /// at the time of releasing the pointer is divided by every 60th of a second.
    /// ## `rate`
    /// The deceleration rate
    fn set_deceleration(&self, rate: f64);

    /// Sets whether the action should emit interpolated ::pan events
    /// after the drag has ended, to emulate the gesture kinetic inertia.
    /// ## `should_interpolate`
    /// whether to enable interpolated pan events
    fn set_interpolate(&self, should_interpolate: bool);

    /// Restricts the panning action to a specific axis
    /// ## `axis`
    /// the axis to constraint the panning to
    fn set_pan_axis(&self, axis: PanAxis);

    /// The ::pan signal is emitted to keep track of the motion during
    /// a pan gesture. `is_interpolated` is set to `true` during the
    /// interpolation phase of the pan, after the drag has ended and
    /// the :interpolate property was set to `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `is_interpolated`
    /// if the event is the result of interpolating
    ///  the motion velocity at the end of the drag
    ///
    /// # Returns
    ///
    /// `true` if the pan should continue, and `false` if
    ///  the pan should be cancelled.
    fn connect_pan<F: Fn(&Self, &Actor, bool) -> bool + 'static>(&self, f: F) -> HandlerId;

    /// The ::pan-stopped signal is emitted at the end of the interpolation
    /// phase of the pan action, only when :interpolate is set to `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_pan_stopped<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_interpolate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_pan_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<PanAction>> PanActionExt for O {
    fn get_acceleration_factor(&self) -> f64 {
        // unsafe { ffi::clutter_pan_action_get_acceleration_factor(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_constrained_motion_delta(&self, point: u32) -> (f32, f32, f32) {
        // unsafe {
        //     let mut delta_x = mem::MaybeUninit::uninit();
        //     let mut delta_y = mem::MaybeUninit::uninit();
        //     let ret = ffi::clutter_pan_action_get_constrained_motion_delta(
        //         self.as_ref().to_glib_none().0,
        //         point,
        //         delta_x.as_mut_ptr(),
        //         delta_y.as_mut_ptr(),
        //     );
        //     let delta_x = delta_x.assume_init();
        //     let delta_y = delta_y.assume_init();
        //     (ret, delta_x, delta_y)
        // }
        unimplemented!()
    }

    fn get_deceleration(&self) -> f64 {
        // unsafe { ffi::clutter_pan_action_get_deceleration(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_interpolate(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_pan_action_get_interpolate(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_interpolated_coords(&self) -> (f32, f32) {
        // unsafe {
        //     let mut interpolated_x = mem::MaybeUninit::uninit();
        //     let mut interpolated_y = mem::MaybeUninit::uninit();
        //     ffi::clutter_pan_action_get_interpolated_coords(
        //         self.as_ref().to_glib_none().0,
        //         interpolated_x.as_mut_ptr(),
        //         interpolated_y.as_mut_ptr(),
        //     );
        //     let interpolated_x = interpolated_x.assume_init();
        //     let interpolated_y = interpolated_y.assume_init();
        //     (interpolated_x, interpolated_y)
        // }
        unimplemented!()
    }

    fn get_interpolated_delta(&self) -> (f32, f32, f32) {
        // unsafe {
        //     let mut delta_x = mem::MaybeUninit::uninit();
        //     let mut delta_y = mem::MaybeUninit::uninit();
        //     let ret = ffi::clutter_pan_action_get_interpolated_delta(
        //         self.as_ref().to_glib_none().0,
        //         delta_x.as_mut_ptr(),
        //         delta_y.as_mut_ptr(),
        //     );
        //     let delta_x = delta_x.assume_init();
        //     let delta_y = delta_y.assume_init();
        //     (ret, delta_x, delta_y)
        // }
        unimplemented!()
    }

    fn get_pan_axis(&self) -> PanAxis {
        // unsafe {
        //     from_glib(ffi::clutter_pan_action_get_pan_axis(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_acceleration_factor(&self, factor: f64) {
        // unsafe {
        //     ffi::clutter_pan_action_set_acceleration_factor(self.as_ref().to_glib_none().0, factor);
        // }
        unimplemented!()
    }

    fn set_deceleration(&self, rate: f64) {
        // unsafe {
        //     ffi::clutter_pan_action_set_deceleration(self.as_ref().to_glib_none().0, rate);
        // }
        unimplemented!()
    }

    fn set_interpolate(&self, should_interpolate: bool) {
        // unsafe {
        //     ffi::clutter_pan_action_set_interpolate(
        //         self.as_ref().to_glib_none().0,
        //         should_interpolate.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_pan_axis(&self, axis: PanAxis) {
        // unsafe {
        //     ffi::clutter_pan_action_set_pan_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        // }
        unimplemented!()
    }

    fn connect_pan<F: Fn(&Self, &Actor, bool) -> bool + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_pan_stopped<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_interpolate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_pan_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for PanAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PanAction")
    }
}
