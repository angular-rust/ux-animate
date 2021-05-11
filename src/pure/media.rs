use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Media{
}

/// Trait containing all `Media` methods.
///
/// # Implementors
///
/// [`Media`](struct.Media.html)
pub trait MediaExt: 'static {}

impl Object for Media {}

impl<O: Is<Media>> MediaExt for O {}

impl fmt::Display for Media {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Media")
    }
}
