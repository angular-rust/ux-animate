use crate::ActorMeta;
use glib::{object as gobject, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct Constraint(Object<ffi::ClutterConstraint, ffi::ClutterConstraintClass, ConstraintClass>) @extends ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_constraint_get_type(),
    }
}

impl Constraint {}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Constraint")
    }
}
