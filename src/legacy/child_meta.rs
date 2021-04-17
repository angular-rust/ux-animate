use crate::{Actor, Container};
use glib::{object::IsA, translate::*};
use std::fmt;

glib_wrapper! {
    pub struct ChildMeta(Object<ffi::ClutterChildMeta, ffi::ClutterChildMetaClass, ChildMetaClass>);

    match fn {
        get_type => || ffi::clutter_child_meta_get_type(),
    }
}

/// Trait containing all `ChildMeta` methods.
///
/// # Implementors
///
/// [`ChildMeta`](struct.ChildMeta.html), [`LayoutMeta`](struct.LayoutMeta.html)
pub trait ChildMetaExt: 'static {
    /// Retrieves the actor wrapped by `self`
    ///
    /// # Returns
    ///
    /// a `Actor`
    fn get_actor(&self) -> Option<Actor>;

    /// Retrieves the container using `self`
    ///
    /// # Returns
    ///
    /// a `Container`
    fn get_container(&self) -> Option<Container>;
}

impl<O: IsA<ChildMeta>> ChildMetaExt for O {
    fn get_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_child_meta_get_actor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_container(&self) -> Option<Container> {
        unsafe {
            from_glib_none(ffi::clutter_child_meta_get_container(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ChildMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChildMeta")
    }
}
