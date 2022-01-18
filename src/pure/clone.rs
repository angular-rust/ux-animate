use super::{Actor, HandlerId};
use crate::prelude::*;
use std::fmt;

// @short_description: An actor that displays a clone of a source actor
//
// #Clone is a #Actor which draws with the paint
// function of another actor, scaled to fit its own allocation.
//
// #Clone can be used to efficiently clone any other actor.
//
// Unlike texture_new_from_actor(), #Clone does not require
// the presence of support for FBOs in the underlying GL or GLES
// implementation.
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
impl Is<Clone> for Clone {}

impl AsRef<Clone> for Clone {
    fn as_ref(&self) -> &Clone {
        self
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
    fn set_source<P: Is<Actor>>(&self, source: Option<&P>);

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
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

    fn connect_property_source_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Clone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clone")
    }
}
