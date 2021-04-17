// use crate::Scriptable;
use glib::translate::*;
use std::fmt;

// @implements Scriptable
glib_wrapper! {
    pub struct State(Object<ffi::ClutterState, ffi::ClutterStateClass, StateClass>);

    match fn {
        get_type => || ffi::clutter_state_get_type(),
    }
}

impl State {}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State")
    }
}
