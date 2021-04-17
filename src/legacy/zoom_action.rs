use super::{Action, Actor, ActorMeta, GestureAction, InternalPoint, ZoomAxis};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct ZoomAction(Object<ffi::ClutterZoomAction, ffi::ClutterZoomActionClass, ZoomActionClass>) @extends GestureAction, Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_zoom_action_get_type(),
    }
}

impl ZoomAction {
    /// Creates a new `ZoomAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `ZoomAction`
    pub fn new() -> ZoomAction {
        unsafe { Action::from_glib_none(ffi::clutter_zoom_action_new()).unsafe_cast() }
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

impl<O: IsA<ZoomAction>> ZoomActionExt for O {
    fn get_focal_point(&self) -> InternalPoint {
        unsafe {
            let mut point = InternalPoint::uninitialized();
            ffi::clutter_zoom_action_get_focal_point(
                self.as_ref().to_glib_none().0,
                point.to_glib_none_mut().0,
            );
            point
        }
    }

    fn get_transformed_focal_point(&self) -> InternalPoint {
        unsafe {
            let mut point = InternalPoint::uninitialized();
            ffi::clutter_zoom_action_get_transformed_focal_point(
                self.as_ref().to_glib_none().0,
                point.to_glib_none_mut().0,
            );
            point
        }
    }

    fn get_zoom_axis(&self) -> ZoomAxis {
        unsafe {
            from_glib(ffi::clutter_zoom_action_get_zoom_axis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_zoom_axis(&self, axis: ZoomAxis) {
        unsafe {
            ffi::clutter_zoom_action_set_zoom_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        }
    }

    fn connect_zoom<F: Fn(&Self, &Actor, &InternalPoint, f64) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn zoom_trampoline<P, F: Fn(&P, &Actor, &InternalPoint, f64) -> bool + 'static>(
            this: *mut ffi::ClutterZoomAction,
            actor: *mut ffi::ClutterActor,
            focal_point: *mut ffi::ClutterPoint,
            factor: libc::c_double,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ZoomAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ZoomAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                &from_glib_borrow(focal_point),
                factor,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"zoom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    zoom_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_zoom_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_zoom_axis_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterZoomAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ZoomAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ZoomAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::zoom-axis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_zoom_axis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ZoomAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ZoomAction")
    }
}
