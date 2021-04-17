// Scriptable
use crate::{Actor, Animatable, Container, InternalPoint, InternalRect, ScrollMode};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// TODO: implements atk::ImplementorIface, Scriptable
glib_wrapper! {
    pub struct ScrollActor(Object<ffi::ClutterScrollActor, ffi::ClutterScrollActorClass, ScrollActorClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_scroll_actor_get_type(),
    }
}

impl ScrollActor {
    /// Creates a new `ScrollActor`.
    ///
    /// # Returns
    ///
    /// The newly created `ScrollActor`
    ///  instance.
    pub fn new() -> ScrollActor {
        unsafe { Actor::from_glib_none(ffi::clutter_scroll_actor_new()).unsafe_cast() }
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

impl<O: IsA<ScrollActor>> ScrollActorExt for O {
    fn get_scroll_mode(&self) -> ScrollMode {
        unsafe {
            from_glib(ffi::clutter_scroll_actor_get_scroll_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn scroll_to_point(&self, point: &InternalPoint) {
        unsafe {
            ffi::clutter_scroll_actor_scroll_to_point(
                self.as_ref().to_glib_none().0,
                point.to_glib_none().0,
            );
        }
    }

    fn scroll_to_rect(&self, rect: &InternalRect) {
        unsafe {
            ffi::clutter_scroll_actor_scroll_to_rect(
                self.as_ref().to_glib_none().0,
                rect.to_glib_none().0,
            );
        }
    }

    fn set_scroll_mode(&self, mode: ScrollMode) {
        unsafe {
            ffi::clutter_scroll_actor_set_scroll_mode(
                self.as_ref().to_glib_none().0,
                mode.to_glib(),
            );
        }
    }

    fn connect_property_scroll_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterScrollActor,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ScrollActor>,
        {
            let f: &F = &*(f as *const F);
            f(&ScrollActor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scroll-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scroll_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ScrollActor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ScrollActor")
    }
}
