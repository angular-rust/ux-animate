use crate::{Action, Actor, ActorMeta, GestureAction, PanAxis};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct PanAction(Object<ffi::ClutterPanAction, ffi::ClutterPanActionClass, PanActionClass>) @extends GestureAction, Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_pan_action_get_type(),
    }
}

impl PanAction {
    /// Creates a new `PanAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `PanAction`
    pub fn new() -> PanAction {
        unsafe { Action::from_glib_none(ffi::clutter_pan_action_new()).unsafe_cast() }
    }
}

impl Default for PanAction {
    fn default() -> Self {
        Self::new()
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
    fn connect_pan<F: Fn(&Self, &Actor, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::pan-stopped signal is emitted at the end of the interpolation
    /// phase of the pan action, only when :interpolate is set to `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_pan_stopped<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_interpolate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pan_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PanAction>> PanActionExt for O {
    fn get_acceleration_factor(&self) -> f64 {
        unsafe { ffi::clutter_pan_action_get_acceleration_factor(self.as_ref().to_glib_none().0) }
    }

    fn get_constrained_motion_delta(&self, point: u32) -> (f32, f32, f32) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            let ret = ffi::clutter_pan_action_get_constrained_motion_delta(
                self.as_ref().to_glib_none().0,
                point,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (ret, delta_x, delta_y)
        }
    }

    fn get_deceleration(&self) -> f64 {
        unsafe { ffi::clutter_pan_action_get_deceleration(self.as_ref().to_glib_none().0) }
    }

    fn get_interpolate(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_pan_action_get_interpolate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_interpolated_coords(&self) -> (f32, f32) {
        unsafe {
            let mut interpolated_x = mem::MaybeUninit::uninit();
            let mut interpolated_y = mem::MaybeUninit::uninit();
            ffi::clutter_pan_action_get_interpolated_coords(
                self.as_ref().to_glib_none().0,
                interpolated_x.as_mut_ptr(),
                interpolated_y.as_mut_ptr(),
            );
            let interpolated_x = interpolated_x.assume_init();
            let interpolated_y = interpolated_y.assume_init();
            (interpolated_x, interpolated_y)
        }
    }

    fn get_interpolated_delta(&self) -> (f32, f32, f32) {
        unsafe {
            let mut delta_x = mem::MaybeUninit::uninit();
            let mut delta_y = mem::MaybeUninit::uninit();
            let ret = ffi::clutter_pan_action_get_interpolated_delta(
                self.as_ref().to_glib_none().0,
                delta_x.as_mut_ptr(),
                delta_y.as_mut_ptr(),
            );
            let delta_x = delta_x.assume_init();
            let delta_y = delta_y.assume_init();
            (ret, delta_x, delta_y)
        }
    }

    fn get_pan_axis(&self) -> PanAxis {
        unsafe {
            from_glib(ffi::clutter_pan_action_get_pan_axis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_acceleration_factor(&self, factor: f64) {
        unsafe {
            ffi::clutter_pan_action_set_acceleration_factor(self.as_ref().to_glib_none().0, factor);
        }
    }

    fn set_deceleration(&self, rate: f64) {
        unsafe {
            ffi::clutter_pan_action_set_deceleration(self.as_ref().to_glib_none().0, rate);
        }
    }

    fn set_interpolate(&self, should_interpolate: bool) {
        unsafe {
            ffi::clutter_pan_action_set_interpolate(
                self.as_ref().to_glib_none().0,
                should_interpolate.to_glib(),
            );
        }
    }

    fn set_pan_axis(&self, axis: PanAxis) {
        unsafe {
            ffi::clutter_pan_action_set_pan_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        }
    }

    fn connect_pan<F: Fn(&Self, &Actor, bool) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pan_trampoline<P, F: Fn(&P, &Actor, bool) -> bool + 'static>(
            this: *mut ffi::ClutterPanAction,
            actor: *mut ffi::ClutterActor,
            is_interpolated: glib_sys::gboolean,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &PanAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                from_glib(is_interpolated),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pan\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pan_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_pan_stopped<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn pan_stopped_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterPanAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &PanAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"pan-stopped\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    pan_stopped_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_acceleration_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_acceleration_factor_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterPanAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(&PanAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::acceleration-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_acceleration_factor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_deceleration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_deceleration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterPanAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(&PanAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::deceleration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_deceleration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_interpolate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interpolate_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterPanAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(&PanAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interpolate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_interpolate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pan_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pan_axis_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterPanAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<PanAction>,
        {
            let f: &F = &*(f as *const F);
            f(&PanAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pan-axis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pan_axis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PanAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PanAction")
    }
}
