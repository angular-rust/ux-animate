use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Score(Object<ffi::ClutterScore, ffi::ClutterScoreClass, ScoreClass>);

    match fn {
        get_type => || ffi::clutter_score_get_type(),
    }
}

impl Score {}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score")
    }
}
