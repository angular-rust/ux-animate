use crate::{Action, Actor, ActorMeta};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct DropAction(Object<ffi::ClutterDropAction, ffi::ClutterDropActionClass, DropActionClass>) @extends Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_drop_action_get_type(),
    }
}

impl DropAction {
    /// Creates a new `DropAction`.
    ///
    /// Use `ActorExt::add_action` to add the action to a `Actor`.
    ///
    /// # Returns
    ///
    /// the newly created `DropAction`
    pub fn new() -> DropAction {
        unsafe { Action::from_glib_none(ffi::clutter_drop_action_new()).unsafe_cast() }
    }
}

impl Default for DropAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `DropAction` methods.
///
/// # Implementors
///
/// [`DropAction`](struct.DropAction.html)
pub trait DropActionExt: 'static {
    /// The ::can-drop signal is emitted when the dragged actor is dropped
    /// on `actor`. The return value of the ::can-drop signal will determine
    /// whether or not the `DropAction::drop` signal is going to be
    /// emitted on `action`.
    ///
    /// The default implementation of `DropAction` returns `true` for
    /// this signal.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    ///
    /// # Returns
    ///
    /// `true` if the drop is accepted, and `false` otherwise
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drop signal is emitted when the dragged actor is dropped
    /// on `actor`. This signal is only emitted if at least an handler of
    /// `DropAction::can-drop` returns `true`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::drop-cancel signal is emitted when the drop is refused
    /// by an emission of the `DropAction::can-drop` signal.
    ///
    /// After the ::drop-cancel signal is fired the active drag is
    /// terminated.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `event_x`
    /// the X coordinate (in stage space) of the drop event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the drop event
    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::over-in signal is emitted when the dragged actor crosses
    /// into `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::over-out signal is emitted when the dragged actor crosses
    /// outside `actor`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DropAction>> DropActionExt for O {
    fn connect_can_drop<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn can_drop_trampoline<P, F: Fn(&P, &Actor, f32, f32) -> bool + 'static>(
            this: *mut ffi::ClutterDropAction,
            actor: *mut ffi::ClutterActor,
            event_x: libc::c_float,
            event_y: libc::c_float,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<DropAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DropAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                event_x,
                event_y,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"can-drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    can_drop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drop<F: Fn(&Self, &Actor, f32, f32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drop_trampoline<P, F: Fn(&P, &Actor, f32, f32) + 'static>(
            this: *mut ffi::ClutterDropAction,
            actor: *mut ffi::ClutterActor,
            event_x: libc::c_float,
            event_y: libc::c_float,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DropAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                event_x,
                event_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drop_cancel<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_cancel_trampoline<P, F: Fn(&P, &Actor, f32, f32) + 'static>(
            this: *mut ffi::ClutterDropAction,
            actor: *mut ffi::ClutterActor,
            event_x: libc::c_float,
            event_y: libc::c_float,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DropAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                event_x,
                event_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop-cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_cancel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_over_in<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn over_in_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterDropAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DropAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"over-in\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    over_in_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_over_out<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn over_out_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterDropAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DropAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DropAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"over-out\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    over_out_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DropAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DropAction")
    }
}
