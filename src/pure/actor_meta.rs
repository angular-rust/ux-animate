use crate::prelude::*;
use super::Actor;
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

#[derive(Debug, Clone)]
pub struct ActorMeta {
    actor: Option<Actor>,
    destroy_id: u32,
    name: Option<String>,
    is_enabled: bool,
    priority: u32,
}

impl Object for ActorMeta {}
impl Is<ActorMeta> for ActorMeta {}

impl AsRef<ActorMeta> for ActorMeta {
    fn as_ref(&self) -> &ActorMeta {
        self
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
    fn get_name(&self) -> Option<String>;

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

impl<O: Is<ActorMeta>> ActorMetaExt for O {
    fn get_actor(&self) -> Option<Actor> {
        // let meta = self.as_ref();
        // meta.actor
        unimplemented!()
    }

    fn get_enabled(&self) -> bool {
        let meta = self.as_ref();
        meta.is_enabled
    }

    fn get_name(&self) -> Option<String> {
        // let meta = self.as_ref();
        // meta.name
        unimplemented!()
    }

    fn set_enabled(&self, is_enabled: bool) {
        unimplemented!()
    }

    fn set_name(&self, name: &str) {
        unimplemented!()
    }

    fn connect_property_actor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ActorMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ActorMeta")
    }
}
