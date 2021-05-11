use std::fmt;

pub struct ShaderMatrix {
}

impl ShaderMatrix {}

impl fmt::Display for ShaderMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderMatrix")
    }
}
