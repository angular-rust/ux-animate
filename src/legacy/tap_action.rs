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
    pub struct TapAction(Object<ffi::ClutterTapAction, ffi::ClutterTapActionClass, TapActionClass>) @extends GestureAction, Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_tap_action_get_type(),
    }
}

impl TapAction {
    /// Creates a new `TapAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `TapAction`
    pub fn new() -> TapAction {
        unsafe { Action::from_glib_none(ffi::clutter_tap_action_new()).unsafe_cast() }
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

impl<O: IsA<TapAction>> TapActionExt for O {
    fn connect_tap<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tap_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterTapAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TapAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TapAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tap\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tap_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TapAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TapAction")
    }
}
