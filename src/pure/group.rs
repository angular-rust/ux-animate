use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable
// @extends Actor
pub struct Group {
}

impl Group {}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group")
    }
}
