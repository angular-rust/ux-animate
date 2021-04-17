use crate::{Action, Actor, ActorMeta, GestureAction};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct RotateAction(Object<ffi::ClutterRotateAction, ffi::ClutterRotateActionClass, RotateActionClass>) @extends GestureAction, Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_rotate_action_get_type(),
    }
}

impl RotateAction {
    /// Creates a new `RotateAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `RotateAction`
    pub fn new() -> RotateAction {
        unsafe { Action::from_glib_none(ffi::clutter_rotate_action_new()).unsafe_cast() }
    }
}

impl Default for RotateAction {
    fn default() -> Self {
        Self::new()
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
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RotateAction>> RotateActionExt for O {
    fn connect_rotate<F: Fn(&Self, &Actor, f64) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn rotate_trampoline<P, F: Fn(&P, &Actor, f64) -> bool + 'static>(
            this: *mut ffi::ClutterRotateAction,
            actor: *mut ffi::ClutterActor,
            angle: libc::c_double,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<RotateAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &RotateAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                angle,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"rotate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    rotate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RotateAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RotateAction")
    }
}
