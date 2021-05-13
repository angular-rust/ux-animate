use crate::Actor;
use std::fmt;

// @implements Animatable, Container, atk::ImplementorIface, Scriptable
pub struct Box {
    inner: Option<Actor>,
}

impl Box {}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Box")
    }
}
