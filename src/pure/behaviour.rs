use std::fmt;
// use crate::Scriptable;

// TODO:  @implements Scriptable
pub struct Behaviour{
    
}

impl Behaviour {}

impl fmt::Display for Behaviour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Behaviour")
    }
}
