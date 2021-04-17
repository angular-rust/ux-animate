use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ShaderMatrix(Object<ffi::ClutterShaderMatrix, ShaderMatrixClass>);

    match fn {
        get_type => || ffi::clutter_shader_matrix_get_type(),
    }
}

impl ShaderMatrix {}

impl fmt::Display for ShaderMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderMatrix")
    }
}
