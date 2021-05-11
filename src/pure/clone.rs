use crate::prelude::*;
use super::{Actor, Animatable, Container};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// * @short_description: An actor that displays a clone of a source actor
// *
// * #ClutterClone is a #ClutterActor which draws with the paint
// * function of another actor, scaled to fit its own allocation.
// *
// * #ClutterClone can be used to efficiently clone any other actor.
// *
// * Unlike clutter_texture_new_from_actor(), #ClutterClone does not require
// * the presence of support for FBOs in the underlying GL or GLES
// * implementation.
// TODO: implements atk::ImplementorIface, Scriptable
// @implements Animatable, Container
// @extends Actor
#[derive(Debug, Clone)]
pub struct Clone {
    clone_source: Option<Actor>,
}

impl Clone {
    /// Creates a new `Actor` which clones `source`/
    /// ## `source`
    /// a `Actor`, or `None`
    ///
    /// # Returns
    ///
    /// the newly created `Clone`
    pub fn new<P: Is<Actor>>(source: &P) -> Clone {
        unimplemented!()
    }
}

impl Object for Clone {}

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
    fn set_source<P: Is<Actor>>(&self, source: Option<&P>);

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Clone>> CloneExt for O {
    fn get_source(&self) -> Option<Actor> {
        // let clone = self.as_ref();
        // clone.clone_source
        unimplemented!()
    }

    fn set_source<P: Is<Actor>>(&self, source: Option<&P>) {
        unimplemented!()
    }

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clone")
    }
}
