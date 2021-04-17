use crate::ActorMeta;
use glib::{object as gobject, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct Action(Object<ffi::ClutterAction, ffi::ClutterActionClass, ActionClass>) @extends ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_action_get_type(),
    }
}

impl Action {}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
