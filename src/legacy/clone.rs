// Scriptable
use crate::{Actor, Animatable, Container};
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
    pub struct Clone(Object<ffi::ClutterClone, ffi::ClutterCloneClass, CloneClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_clone_get_type(),
    }
}

impl Clone {
    /// Creates a new `Actor` which clones `source`/
    /// ## `source`
    /// a `Actor`, or `None`
    ///
    /// # Returns
    ///
    /// the newly created `Clone`
    pub fn new<P: IsA<Actor>>(source: &P) -> Clone {
        unsafe {
            Actor::from_glib_none(ffi::clutter_clone_new(source.as_ref().to_glib_none().0))
                .unsafe_cast()
        }
    }
}

/// Trait containing all `Clone` methods.
///
/// # Implementors
///
/// [`Clone`](struct.Clone.html)
pub trait CloneExt: 'static {
    /// Retrieves the source `Actor` being cloned by `self`.
    ///
    /// # Returns
    ///
    /// the actor source for the clone
    fn get_source(&self) -> Option<Actor>;

    /// Sets `source` as the source actor to be cloned by `self`.
    /// ## `source`
    /// a `Actor`, or `None`
    fn set_source<P: IsA<Actor>>(&self, source: Option<&P>);

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Clone>> CloneExt for O {
    fn get_source(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_clone_get_source(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_source<P: IsA<Actor>>(&self, source: Option<&P>) {
        unsafe {
            ffi::clutter_clone_set_source(
                self.as_ref().to_glib_none().0,
                source.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_source_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClone,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Clone>,
        {
            let f: &F = &*(f as *const F);
            f(&Clone::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::source\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_source_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clone")
    }
}
