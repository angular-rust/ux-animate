use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ShaderFloat(Object<ffi::ClutterShaderFloat, ShaderFloatClass>);

    match fn {
        get_type => || ffi::clutter_shader_float_get_type(),
    }
}

impl ShaderFloat {}

impl fmt::Display for ShaderFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderFloat")
    }
}
