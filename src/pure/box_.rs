use crate::{Actor, Animatable, Container};
use glib::{object as gobject, translate::*};
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable
glib_wrapper! {
    pub struct Box(Object<ffi::ClutterBox, ffi::ClutterBoxClass, BoxClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_box_get_type(),
    }
}

impl Box {}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Box")
    }
}
