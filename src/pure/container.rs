#![allow(unused_imports)]
use super::{Actor, ChildMeta, HandlerId};
use crate::prelude::*;
use std::fmt;

// SECTION:clutter-container
// @short_description: An interface for container actors
//
// #Container is an interface implemented by #Actor, and
// it provides some common API for notifying when a child actor is added
// or removed, as well as the infrastructure for accessing child properties
// through #ChildMeta.
//
// Until  1.10, the #Container interface was also the public
// API for implementing container actors; this part of the interface has
// been deprecated: #Container has a default implementation which
// defers to #Actor the child addition and removal, as well as the
// iteration. See the documentation of #ContainerIface for the list
// of virtual functions that should be overridden.

// ContainerIface:
// @add: virtual function for adding an actor to the container. This virtual
//   function is deprecated, and it should not be overridden.
// @remove: virtual function for removing an actor from the container. This
//   virtual function is deprecated, and it should not be overridden.
// @foreach: virtual function for iterating over the container's children.
//   This virtual function is deprecated, and it should not be overridden.
// @foreach_with_internals: virtual functions for iterating over the
//   container's children, both added using the #Container API
//   and internal children. The implementation of this virtual function
//   is required only if the #Container implementation has
//   internal children. This virtual function is deprecated, and it should
//   not be overridden.
// @raise: virtual function for raising a child. This virtual function is
//   deprecated and it should not be overridden.
// @lower: virtual function for lowering a child. This virtual function is
//   deprecated and it should not be overridden.
// @sort_depth_order: virtual function for sorting the children of a
//   container depending on their depth. This virtual function is deprecated
//   and it should not be overridden.
// @child_meta_type: The GType used for storing auxiliary information about
//   each of the containers children.
// @create_child_meta: virtual function that gets called for each added
//   child, the function should instantiate an object of type
//   #ContainerIface::child_meta_type, set the container and actor
//   fields in the instance and add the record to a data structure for
//   subsequent access for #ContainerIface::get_child_meta
// @destroy_child_meta: virtual function that gets called when a child is
//   removed; it shuld release all resources held by the record
// @get_child_meta: return the record for a container child
// @actor_added: class handler for #Container::actor-added
// @actor_removed: class handler for #Container::actor-removed
// @child_notify: class handler for #Container::child-notify
//
// Base interface for container actors. The @add, @remove and @foreach
// virtual functions must be provided by any implementation; the other
// virtual functions are optional.

#[derive(Debug, Clone)]
pub struct Container {}

impl Container {
    // /// Looks up the `gobject::ParamSpec` for a child property of `klass`.
    // /// ## `klass`
    // /// a `gobject::ObjectClass` implementing the `Container` interface.
    // /// ## `property_name`
    // /// a property name.
    // ///
    // /// # Returns
    // ///
    // /// The `gobject::ParamSpec` for the property or `None`
    // ///  if no such property exist.
    // pub fn class_find_child_property(
    //     klass: &mut glib::ObjectClass,
    //     property_name: &str,
    // ) -> Option<glib::ParamSpec> {
    //     unsafe {
    //         from_glib_none(ffi::clutter_container_class_find_child_property(
    //             klass.to_glib_none_mut().0,
    //             property_name.to_glib_none().0,
    //         ))
    //     }
    // }

    // /// Returns an array of `gobject::ParamSpec` for all child properties.
    // /// ## `klass`
    // /// a `gobject::ObjectClass` implementing the `Container` interface.
    // /// ## `n_properties`
    // /// return location for length of returned array.
    // ///
    // /// # Returns
    // ///
    // /// an array
    // ///  of `gobject::ParamSpec`<!-- -->s which should be freed after use.
    // pub fn class_list_child_properties(klass: &mut glib::ObjectClass) -> Vec<glib::ParamSpec> {
    //     unsafe {
    //         let mut n_properties = mem::MaybeUninit::uninit();
    //         let ret = FromGlibContainer::from_glib_full_num(
    //             ffi::clutter_container_class_list_child_properties(
    //                 klass.to_glib_none_mut().0,
    //                 n_properties.as_mut_ptr(),
    //             ),
    //             n_properties.assume_init() as usize,
    //         );
    //         ret
    //     }
    // }
}

impl Object for Container {}
impl Is<Container> for Container {}

impl AsRef<Container> for Container {
    fn as_ref(&self) -> &Container {
        self
    }
}

/// Trait containing all `Container` methods.
///
/// # Implementors
///
/// [`Actor`](struct.Actor.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Container`](struct.Container.html), [`Group`](struct.Group.html), [`Rectangle`](struct.Rectangle.html), [`ScrollActor`](struct.ScrollActor.html), [`Stage`](struct.Stage.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html)
pub trait ContainerExt: 'static {
    //fn child_get<P: Is<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    // /// Gets a container specific property of a child of `self`, In general,
    // /// a copy is made of the property contents and the caller is responsible for
    // /// freeing the memory by calling `gobject::Value::unset`.
    // ///
    // /// Note that `Container::child_set_property` is really intended for
    // /// language bindings, `Container::child_set` is much more convenient
    // /// for C programming.
    // /// ## `child`
    // /// a `Actor` that is a child of `self`.
    // /// ## `property`
    // /// the name of the property to set.
    // /// ## `value`
    // /// the value.
    // fn child_get_property<P: Is<Actor>>(&self, child: &P, property: &str, value: &mut glib::Value);

    /// Calls the `ContainerIface.child_notify`() virtual function
    /// of `Container`. The default implementation will emit the
    /// `Container::child-notify` signal.
    /// ## `child`
    /// a `Actor`
    /// ## `pspec`
    /// a `gobject::ParamSpec`
    // fn child_notify<P: Is<Actor>, Q: Is<glib::ParamSpec>>(&self, child: &P, pspec: &Q);

    //fn child_set<P: Is<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    // /// Sets a container-specific property on a child of `self`.
    // /// ## `child`
    // /// a `Actor` that is a child of `self`.
    // /// ## `property`
    // /// the name of the property to set.
    // /// ## `value`
    // /// the value.
    // fn child_set_property<P: Is<Actor>>(&self, child: &P, property: &str, value: &glib::Value);

    /// Creates the `ChildMeta` wrapping `actor` inside the
    /// `self`, if the `ContainerIface::child_meta_type`
    /// class member is not set to `G_TYPE_INVALID`.
    ///
    /// This function is only useful when adding a `Actor` to
    /// a `Container` implementation outside of the
    /// `Container::add`() virtual function implementation.
    ///
    /// Applications should not call this function.
    /// ## `actor`
    /// a `Actor`
    fn create_child_meta<P: Is<Actor>>(&self, actor: &P);

    /// Destroys the `ChildMeta` wrapping `actor` inside the
    /// `self`, if any.
    ///
    /// This function is only useful when removing a `Actor` to
    /// a `Container` implementation outside of the
    /// `Container::add`() virtual function implementation.
    ///
    /// Applications should not call this function.
    /// ## `actor`
    /// a `Actor`
    fn destroy_child_meta<P: Is<Actor>>(&self, actor: &P);

    /// Finds a child actor of a container by its name. Search recurses
    /// into any child container.
    /// ## `child_name`
    /// the name of the requested child.
    ///
    /// # Returns
    ///
    /// The child actor with the requested name,
    ///  or `None` if no actor with that name was found.
    fn find_child_by_name(&self, child_name: &str) -> Option<Actor>;

    /// Retrieves the `ChildMeta` which contains the data about the
    /// `self` specific state for `actor`.
    /// ## `actor`
    /// a `Actor` that is a child of `self`.
    ///
    /// # Returns
    ///
    /// the `ChildMeta` for the `actor` child
    ///  of `self` or `None` if the specifiec actor does not exist or the
    ///  container is not configured to provide `ChildMeta`<!-- -->s
    fn get_child_meta<P: Is<Actor>>(&self, actor: &P) -> Option<ChildMeta>;

    /// The ::actor-added signal is emitted each time an actor
    /// has been added to `container`.
    /// ## `actor`
    /// the new child that has been added to `container`
    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    /// The ::actor-removed signal is emitted each time an actor
    /// is removed from `container`.
    /// ## `actor`
    /// the child that has been removed from `container`
    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId;

    // /// The ::child-notify signal is emitted each time a property is
    // /// being set through the `Container::child_set` and
    // /// `Container::child_set_property` calls.
    // /// ## `actor`
    // /// the child that has had a property set
    // /// ## `pspec`
    // /// the `gobject::ParamSpec` of the property set
    // fn connect_child_notify<F: Fn(&Self, &Actor, &glib::ParamSpec) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId;
}

impl<O: Is<Container>> ContainerExt for O {
    //fn child_get<P: Is<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unimplemented!()
    //}

    // fn child_get_property<P: Is<Actor>>(&self, child: &P, property: &str, value: &mut glib::Value) {
    //     unimplemented!()
    // }

    // fn child_notify<P: Is<Actor>, Q: Is<glib::ParamSpec>>(&self, child: &P, pspec: &Q) {
    //     unimplemented!()
    // }

    //fn child_set<P: Is<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unimplemented!()
    //}

    // fn child_set_property<P: Is<Actor>>(&self, child: &P, property: &str, value: &glib::Value) {
    //     unimplemented!()
    // }

    fn create_child_meta<P: Is<Actor>>(&self, actor: &P) {
        unimplemented!()
    }

    fn destroy_child_meta<P: Is<Actor>>(&self, actor: &P) {
        unimplemented!()
    }

    fn find_child_by_name(&self, child_name: &str) -> Option<Actor> {
        unimplemented!()
    }

    fn get_child_meta<P: Is<Actor>>(&self, actor: &P) -> Option<ChildMeta> {
        unimplemented!()
    }

    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    // fn connect_child_notify<F: Fn(&Self, &Actor, &glib::ParamSpec) + 'static>(
    //     &self,
    //     f: F,
    // ) -> HandlerId {
    //     unimplemented!()
    // }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Container")
    }
}
