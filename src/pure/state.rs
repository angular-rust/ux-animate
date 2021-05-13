use std::fmt;

// @implements Scriptable
pub struct State {
}

impl State {}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "State")
    }
}
