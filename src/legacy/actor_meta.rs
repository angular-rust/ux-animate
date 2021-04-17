use crate::Actor;
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct ActorMeta(Object<ffi::ClutterActorMeta, ffi::ClutterActorMetaClass, ActorMetaClass>) @extends gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_actor_meta_get_type(),
    }
}

/// Trait containing all `ActorMeta` methods.
///
/// # Implementors
///
/// [`Action`](struct.Action.html), [`ActorMeta`](struct.ActorMeta.html), [`Constraint`](struct.Constraint.html), [`Effect`](struct.Effect.html)
pub trait ActorMetaExt: 'static {
    /// Retrieves a pointer to the `Actor` that owns `self`
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor` or `None`
    fn get_actor(&self) -> Option<Actor>;

    /// Retrieves whether `self` is enabled
    ///
    /// # Returns
    ///
    /// `true` if the `ActorMeta` instance is enabled
    fn get_enabled(&self) -> bool;

    /// Retrieves the name set using `ActorMetaExt::set_name`
    ///
    /// # Returns
    ///
    /// the name of the `ActorMeta`
    ///  instance, or `None` if none was set. The returned string is owned
    ///  by the `ActorMeta` instance and it should not be modified
    ///  or freed
    fn get_name(&self) -> Option<GString>;

    /// Sets whether `self` should be enabled or not
    /// ## `is_enabled`
    /// whether `self` is enabled
    fn set_enabled(&self, is_enabled: bool);

    /// Sets the name of `self`
    ///
    /// The name can be used to identify the `ActorMeta` instance
    /// ## `name`
    /// the name of `self`
    fn set_name(&self, name: &str);

    fn connect_property_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActorMeta>> ActorMetaExt for O {
    fn get_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_actor_meta_get_actor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_actor_meta_get_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_actor_meta_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_enabled(&self, is_enabled: bool) {
        unsafe {
            ffi::clutter_actor_meta_set_enabled(
                self.as_ref().to_glib_none().0,
                is_enabled.to_glib(),
            );
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::clutter_actor_meta_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn connect_property_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_actor_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterActorMeta,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorMeta>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::actor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_actor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterActorMeta,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorMeta>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterActorMeta,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorMeta>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorMeta::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActorMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorMeta")
    }
}
