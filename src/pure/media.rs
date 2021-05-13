use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Media{
}

impl Object for Media {}
impl Is<Media> for Media {}

impl AsRef<Media> for Media {
    fn as_ref(&self) -> &Media {
        self
    }
}

/// Trait containing all `Media` methods.
///
/// # Implementors
///
/// [`Media`](struct.Media.html)
pub trait MediaExt: 'static {}

impl<O: Is<Media>> MediaExt for O {}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Media")
    }
}
