// Scriptable
use crate::{Actor, Animatable, Container};
use glib::{object as gobject, translate::*};
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable
glib_wrapper! {
    pub struct Group(Object<ffi::ClutterGroup, ffi::ClutterGroupClass, GroupClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_group_get_type(),
    }
}

impl Group {}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group")
    }
}
