use std::fmt;

pub struct ShaderInt {
}

impl ShaderInt {}

impl fmt::Display for ShaderInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderInt")
    }
}
