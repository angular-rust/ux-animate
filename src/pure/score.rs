use std::fmt;

pub struct Score {}

impl Score {}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score")
    }
}
