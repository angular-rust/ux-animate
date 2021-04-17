use crate::{Actor, ChildMeta};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct Container(Interface<ffi::ClutterContainer>);

    match fn {
        get_type => || ffi::clutter_container_get_type(),
    }
}

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

/// Trait containing all `Container` methods.
///
/// # Implementors
///
/// [`Actor`](struct.Actor.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Container`](struct.Container.html), [`Group`](struct.Group.html), [`Rectangle`](struct.Rectangle.html), [`ScrollActor`](struct.ScrollActor.html), [`Stage`](struct.Stage.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html)
pub trait ContainerExt: 'static {
    //fn child_get<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Gets a container specific property of a child of `self`, In general,
    /// a copy is made of the property contents and the caller is responsible for
    /// freeing the memory by calling `gobject::Value::unset`.
    ///
    /// Note that `Container::child_set_property` is really intended for
    /// language bindings, `Container::child_set` is much more convenient
    /// for C programming.
    /// ## `child`
    /// a `Actor` that is a child of `self`.
    /// ## `property`
    /// the name of the property to set.
    /// ## `value`
    /// the value.
    fn child_get_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: &mut glib::Value);

    /// Calls the `ContainerIface.child_notify`() virtual function
    /// of `Container`. The default implementation will emit the
    /// `Container::child-notify` signal.
    /// ## `child`
    /// a `Actor`
    /// ## `pspec`
    /// a `gobject::ParamSpec`
    // fn child_notify<P: IsA<Actor>, Q: IsA<glib::ParamSpec>>(&self, child: &P, pspec: &Q);

    //fn child_set<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets a container-specific property on a child of `self`.
    /// ## `child`
    /// a `Actor` that is a child of `self`.
    /// ## `property`
    /// the name of the property to set.
    /// ## `value`
    /// the value.
    fn child_set_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: &glib::Value);

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
    fn create_child_meta<P: IsA<Actor>>(&self, actor: &P);

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
    fn destroy_child_meta<P: IsA<Actor>>(&self, actor: &P);

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
    fn get_child_meta<P: IsA<Actor>>(&self, actor: &P) -> Option<ChildMeta>;

    /// The ::actor-added signal is emitted each time an actor
    /// has been added to `container`.
    /// ## `actor`
    /// the new child that has been added to `container`
    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::actor-removed signal is emitted each time an actor
    /// is removed from `container`.
    /// ## `actor`
    /// the child that has been removed from `container`
    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::child-notify signal is emitted each time a property is
    /// being set through the `Container::child_set` and
    /// `Container::child_set_property` calls.
    /// ## `actor`
    /// the child that has had a property set
    /// ## `pspec`
    /// the `gobject::ParamSpec` of the property set
    fn connect_child_notify<F: Fn(&Self, &Actor, &glib::ParamSpec) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Container>> ContainerExt for O {
    //fn child_get<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_container_child_get() }
    //}

    fn child_get_property<P: IsA<Actor>>(
        &self,
        child: &P,
        property: &str,
        value: &mut glib::Value,
    ) {
        unsafe {
            ffi::clutter_container_child_get_property(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
        }
    }

    // fn child_notify<P: IsA<Actor>, Q: IsA<glib::ParamSpec>>(&self, child: &P, pspec: &Q) {
    //     unsafe {
    //         ffi::clutter_container_child_notify(
    //             self.as_ref().to_glib_none().0,
    //             child.as_ref().to_glib_none().0,
    //             pspec.as_ref().to_glib_none().0,
    //         );
    //     }
    // }

    //fn child_set<P: IsA<Actor>>(&self, actor: &P, first_prop: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_container_child_set() }
    //}

    fn child_set_property<P: IsA<Actor>>(&self, child: &P, property: &str, value: &glib::Value) {
        unsafe {
            ffi::clutter_container_child_set_property(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    fn create_child_meta<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_create_child_meta(
                self.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
            );
        }
    }

    fn destroy_child_meta<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_container_destroy_child_meta(
                self.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
            );
        }
    }

    fn find_child_by_name(&self, child_name: &str) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_container_find_child_by_name(
                self.as_ref().to_glib_none().0,
                child_name.to_glib_none().0,
            ))
        }
    }

    fn get_child_meta<P: IsA<Actor>>(&self, actor: &P) -> Option<ChildMeta> {
        unsafe {
            from_glib_none(ffi::clutter_container_get_child_meta(
                self.as_ref().to_glib_none().0,
                actor.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_actor_added<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn actor_added_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContainer,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"actor-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    actor_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_actor_removed<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn actor_removed_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContainer,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"actor-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    actor_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_child_notify<F: Fn(&Self, &Actor, &glib::ParamSpec) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_notify_trampoline<
            P,
            F: Fn(&P, &Actor, &glib::ParamSpec) + 'static,
        >(
            this: *mut ffi::ClutterContainer,
            actor: *mut ffi::ClutterActor,
            pspec: *mut gobject_sys::GParamSpec,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Container>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Container::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                &from_glib_borrow(pspec),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-notify\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_notify_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Container")
    }
}
