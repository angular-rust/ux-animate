// Scriptable
use super::{Timeline, Transition};
use glib::{
    object::{Cast, IsA},
    translate::*,
};
use std::fmt;

// TODO: @implements Scriptable
glib_wrapper! {
    pub struct TransitionGroup(Object<ffi::ClutterTransitionGroup, ffi::ClutterTransitionGroupClass, TransitionGroupClass>) @extends Transition, Timeline;

    match fn {
        get_type => || ffi::clutter_transition_group_get_type(),
    }
}

impl TransitionGroup {
    /// Creates a new `TransitionGroup` instance.
    ///
    /// # Returns
    ///
    /// the newly created `TransitionGroup`. Use
    ///  `gobject::ObjectExt::unref` when done to deallocate the resources it
    ///  uses
    pub fn new() -> TransitionGroup {
        unsafe { Transition::from_glib_full(ffi::clutter_transition_group_new()).unsafe_cast() }
    }
}

impl Default for TransitionGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `TransitionGroup` methods.
///
/// # Implementors
///
/// [`TransitionGroup`](struct.TransitionGroup.html)
pub trait TransitionGroupExt: 'static {
    /// Adds `transition` to `self`.
    ///
    /// This function acquires a reference on `transition` that will be released
    /// when calling `TransitionGroupExt::remove_transition`.
    /// ## `transition`
    /// a `Transition`
    fn add_transition<P: IsA<Transition>>(&self, transition: &P);

    /// Removes all transitions from `self`.
    ///
    /// This function releases the reference acquired when calling
    /// `TransitionGroupExt::add_transition`.
    fn remove_all(&self);

    /// Removes `transition` from `self`.
    ///
    /// This function releases the reference acquired on `transition` when
    /// calling `TransitionGroupExt::add_transition`.
    /// ## `transition`
    /// a `Transition`
    fn remove_transition<P: IsA<Transition>>(&self, transition: &P);
}

impl<O: IsA<TransitionGroup>> TransitionGroupExt for O {
    fn add_transition<P: IsA<Transition>>(&self, transition: &P) {
        unsafe {
            ffi::clutter_transition_group_add_transition(
                self.as_ref().to_glib_none().0,
                transition.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_all(&self) {
        unsafe {
            ffi::clutter_transition_group_remove_all(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_transition<P: IsA<Transition>>(&self, transition: &P) {
        unsafe {
            ffi::clutter_transition_group_remove_transition(
                self.as_ref().to_glib_none().0,
                transition.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for TransitionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransitionGroup")
    }
}
