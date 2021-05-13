use std::fmt;

pub struct ModelIter {
}

impl ModelIter {}

impl fmt::Display for ModelIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModelIter")
    }
}
