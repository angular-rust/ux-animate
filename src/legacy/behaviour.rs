use glib::translate::*;
use std::fmt;
// use crate::Scriptable;

// TODO:  @implements Scriptable
glib_wrapper! {
    pub struct Behaviour(Object<ffi::ClutterBehaviour, ffi::ClutterBehaviourClass, BehaviourClass>);

    match fn {
        get_type => || ffi::clutter_behaviour_get_type(),
    }
}

impl Behaviour {}

impl fmt::Display for Behaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Behaviour")
    }
}
