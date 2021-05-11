use crate::prelude::*;
use glib::{object as gobject, translate::*};
use std::fmt;
// use crate::Scriptable;

// TODO: @implements Scriptable
#[derive(Debug, Clone)]
pub struct Alpha {}

impl Alpha {}

impl Object for Alpha {}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha")
    }
}
