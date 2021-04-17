use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Model(Object<ffi::ClutterModel, ffi::ClutterModelClass, ModelClass>);

    match fn {
        get_type => || ffi::clutter_model_get_type(),
    }
}

impl Model {}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Model")
    }
}
