use crate::{Action, Actor, ActorMeta, GestureAction, SwipeDirection};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct SwipeAction(Object<ffi::ClutterSwipeAction, ffi::ClutterSwipeActionClass, SwipeActionClass>) @extends GestureAction, Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_swipe_action_get_type(),
    }
}

impl SwipeAction {
    /// Creates a new `SwipeAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `SwipeAction`
    pub fn new() -> SwipeAction {
        unsafe { Action::from_glib_none(ffi::clutter_swipe_action_new()).unsafe_cast() }
    }
}

impl Default for SwipeAction {
    fn default() -> Self {
        Self::new()
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
    ) -> SignalHandlerId;
}

impl<O: IsA<SwipeAction>> SwipeActionExt for O {
    fn connect_swipe<F: Fn(&Self, &Actor, SwipeDirection) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn swipe_trampoline<
            P,
            F: Fn(&P, &Actor, SwipeDirection) -> bool + 'static,
        >(
            this: *mut ffi::ClutterSwipeAction,
            actor: *mut ffi::ClutterActor,
            direction: ffi::ClutterSwipeDirection,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<SwipeAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &SwipeAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                from_glib(direction),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"swipe\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    swipe_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SwipeAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwipeAction")
    }
}
