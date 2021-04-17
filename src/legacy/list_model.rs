use super::Model;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct ListModel(Object<ffi::ClutterListModel, ffi::ClutterListModelClass, ListModelClass>) @extends Model;

    match fn {
        get_type => || ffi::clutter_list_model_get_type(),
    }
}

impl ListModel {}

impl fmt::Display for ListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListModel")
    }
}
