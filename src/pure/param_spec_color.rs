// use glib::{object as gobject, translate::*};
// use std::fmt;

// glib_wrapper! {
//     pub struct ParamSpecColor(Object<ffi::ParamSpecColor, ParamSpecColorClass>) @extends gobject::ParamSpec;

//     match fn {
//         get_type => || ffi::param_color_get_type(),
//     }
// }

// impl ParamSpecColor {}

// impl fmt::Display for ParamSpecColor {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "ParamSpecColor")
//     }
// }
