use crate::prelude::*;
use std::fmt;
// use crate::Scriptable;

// TODO: @implements Scriptable
#[derive(Debug, Clone)]
pub struct Alpha {}

impl Alpha {}

impl Object for Alpha {}
impl Is<Alpha> for Alpha {}

impl AsRef<Alpha> for Alpha {
    fn as_ref(&self) -> &Alpha {
        self
    }
}

impl fmt::Display for Alpha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha")
    }
}
