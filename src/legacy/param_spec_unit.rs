// use glib::translate::*;
// use std::fmt;

// glib_wrapper! {
//     pub struct ParamSpecUnit(Object<ffi::ClutterParamSpecUnit, ParamSpecUnitClass>) @extends glib::ParamSpec;

//     match fn {
//         get_type => || ffi::clutter_param_units_get_type(),
//     }
// }

// impl ParamSpecUnit {}

// impl fmt::Display for ParamSpecUnit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "ParamSpecUnit")
//     }
// }
