use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ModelIter(Object<ffi::ClutterModelIter, ffi::ClutterModelIterClass, ModelIterClass>);

    match fn {
        get_type => || ffi::clutter_model_iter_get_type(),
    }
}

impl ModelIter {}

impl fmt::Display for ModelIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModelIter")
    }
}
