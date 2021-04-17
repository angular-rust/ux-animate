use glib::{object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct Media(Interface<ffi::ClutterMedia>);

    match fn {
        get_type => || ffi::clutter_media_get_type(),
    }
}

/// Trait containing all `Media` methods.
///
/// # Implementors
///
/// [`Media`](struct.Media.html)
pub trait MediaExt: 'static {}

impl<O: IsA<Media>> MediaExt for O {}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Media")
    }
}
