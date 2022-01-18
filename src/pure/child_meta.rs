
use crate::prelude::*;
use super::{Actor, Container};
use std::fmt;


// @short_description: Wrapper for actors inside a container
//
// #ChildMeta is a wrapper object created by #Container
// implementations in order to store child-specific data and properties.
//
// A #ChildMeta wraps a #Actor inside a #Container.
// ChildMeta:
// @container: the container handling this data
// @actor: the actor wrapped by this data
// 
// Base interface for container specific state for child actors. A child
// data is meant to be used when you need to keep track of information
// about each individual child added to a container.
//
// In order to use it you should create your own subclass of
// #ChildMeta and set the #ContainerIface child_meta_type
// interface member to your subclass type, like:
//
// ```
// static void
// my_container_iface_init (ContainerIface *iface)
// {
//   // set the rest of the #Container vtable
//
//   container_iface->child_meta_type  = MY_TYPE_CHILD_META;
// }
// ```
//
// This will automatically create a #ChildMeta of type
// `MY_TYPE_CHILD_META` for every actor that is added to the container.
//
// The child data for an actor can be retrieved using the
// container_get_child_meta() function.
// 
// The properties of the data and your subclass can be manipulated with
// container_child_set() and container_child_get() which
// act like g_object_set() and g_object_get().
//
// You can provide hooks for your own storage as well as control the
// instantiation by overriding the #ContainerIface virtual functions
// #ContainerIface.create_child_meta(), #ContainerIface.destroy_child_meta(),
// and #ContainerIface.get_child_meta().

#[derive(Debug, Clone)]
pub struct ChildMeta {
    actor: Option<Actor>,
    container: Option<Container>,
}

impl Object for ChildMeta {}
impl Is<ChildMeta> for ChildMeta {}

impl AsRef<ChildMeta> for ChildMeta {
    fn as_ref(&self) -> &ChildMeta {
        self
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

impl<O: Is<ChildMeta>> ChildMetaExt for O {
    fn get_actor(&self) -> Option<Actor> {
        // let meta = self.as_ref();
        // meta.actor
        unimplemented!()
    }

    fn get_container(&self) -> Option<Container> {
        // let meta = self.as_ref();
        // meta.container
        unimplemented!()
    }
}

impl fmt::Display for ChildMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ChildMeta")
    }
}
