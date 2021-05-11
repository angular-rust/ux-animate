use std::fmt;

pub struct ShaderFloat {
}

impl ShaderFloat {}

impl fmt::Display for ShaderFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderFloat")
    }
}
