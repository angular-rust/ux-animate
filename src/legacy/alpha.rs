use glib::{object as gobject, translate::*};
use std::fmt;
// use crate::Scriptable;

// TODO: , @implements Scriptable
glib_wrapper! {
    pub struct Alpha(Object<ffi::ClutterAlpha, ffi::ClutterAlphaClass, AlphaClass>) @extends gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_alpha_get_type(),
    }
}

impl Alpha {}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha")
    }
}
