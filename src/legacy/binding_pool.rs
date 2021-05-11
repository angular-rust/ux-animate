use crate::ModifierType;
use glib::{
    object::{IsA, ObjectType as ObjectType_},
    translate::*,
    GString, StaticType, Value,
};
use std::fmt;

glib_wrapper! {
    pub struct BindingPool(Object<ffi::ClutterBindingPool, ffi::ClutterBindingPoolClass, BindingPoolClass>);

    match fn {
        get_type => || ffi::clutter_binding_pool_get_type(),
    }
}

impl BindingPool {
    /// Creates a new `BindingPool` that can be used to store
    /// key bindings for an actor. The `name` must be a unique identifier
    /// for the binding pool, so that `BindingPool::find` will
    /// be able to return the correct binding pool.
    /// ## `name`
    /// the name of the binding pool
    ///
    /// # Returns
    ///
    /// the newly created binding pool with the given
    ///  name. Use `gobject::ObjectExt::unref` when done.
    pub fn new(name: &str) -> BindingPool {
        unsafe { from_glib_full(ffi::clutter_binding_pool_new(name.to_glib_none().0)) }
    }

    /// Activates the callback associated to the action that is
    /// bound to the `key_val` and `modifiers` pair.
    ///
    /// The callback has the following signature:
    ///
    ///
    /// ```text
    ///   void (* callback) (GObject             *gobject,
    ///                      const gchar         *action_name,
    ///                      guint                key_val,
    ///                      ModifierType  modifiers,
    ///                      gpointer             user_data);
    /// ```
    ///
    /// Where the `gobject::Object` instance is `gobject` and the user data
    /// is the one passed when installing the action with
    /// `BindingPool::install_action`.
    ///
    /// If the action bound to the `key_val`, `modifiers` pair has been
    /// blocked using `BindingPool::block_action`, the callback
    /// will not be invoked, and this function will return `false`.
    /// ## `key_val`
    /// the key symbol
    /// ## `modifiers`
    /// bitmask for the modifiers
    /// ## `gobject`
    /// a `gobject::Object`
    ///
    /// # Returns
    ///
    /// `true` if an action was found and was activated
    pub fn activate<P: IsA<glib::Object>>(
        &self,
        key_val: u32,
        modifiers: ModifierType,
        gobject: &P,
    ) -> bool {
        unsafe {
            from_glib(ffi::clutter_binding_pool_activate(
                self.to_glib_none().0,
                key_val,
                modifiers.to_glib(),
                gobject.as_ref().to_glib_none().0,
            ))
        }
    }

    /// Blocks all the actions with name `action_name` inside `self`.
    /// ## `action_name`
    /// an action name
    pub fn block_action(&self, action_name: &str) {
        unsafe {
            ffi::clutter_binding_pool_block_action(
                self.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    /// Retrieves the name of the action matching the given key symbol
    /// and modifiers bitmask.
    /// ## `key_val`
    /// a key symbol
    /// ## `modifiers`
    /// a bitmask for the modifiers
    ///
    /// # Returns
    ///
    /// the name of the action, if found, or `None`. The
    ///  returned string is owned by the binding pool and should never
    ///  be modified or freed
    pub fn find_action(&self, key_val: u32, modifiers: ModifierType) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_binding_pool_find_action(
                self.to_glib_none().0,
                key_val,
                modifiers.to_glib(),
            ))
        }
    }

    // /// Installs a new action inside a `BindingPool`. The action
    // /// is bound to `key_val` and `modifiers`.
    // ///
    // /// The same action name can be used for multiple `key_val`, `modifiers`
    // /// pairs.
    // ///
    // /// When an action has been activated using `BindingPool::activate`
    // /// the passed `callback` will be invoked (with `data`).
    // ///
    // /// Actions can be blocked with `BindingPool::block_action`
    // /// and then unblocked using `BindingPool::unblock_action`.
    // /// ## `action_name`
    // /// the name of the action
    // /// ## `key_val`
    // /// key symbol
    // /// ## `modifiers`
    // /// bitmask of modifiers
    // /// ## `callback`
    // /// function to be called
    // ///  when the action is activated
    // /// ## `data`
    // /// data to be passed to `callback`
    // /// ## `notify`
    // /// function to be called when the action is removed
    // ///  from the pool
    // pub fn install_action<P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static>(
    //     &self,
    //     action_name: &str,
    //     key_val: u32,
    //     modifiers: ModifierType,
    //     callback: P,
    // ) {
    //     let callback_data: Box_<P> = Box_::new(callback);
    //     unsafe extern "C" fn callback_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         gobject: *mut gobject_sys::GObject,
    //         action_name: *const libc::c_char,
    //         key_val: libc::c_uint,
    //         modifiers: ffi::ClutterModifierType,
    //         user_data: glib_sys::gpointer,
    //     ) -> glib_sys::gboolean {
    //         let gobject = from_glib_borrow(gobject);
    //         let action_name: Borrowed<GString> = from_glib_borrow(action_name);
    //         let modifiers = from_glib_borrow(modifiers);
    //         let callback: &P = &*(user_data as *mut _);
    //         let res = (*callback)(&gobject, action_name.as_str(), key_val, &modifiers);
    //         res.to_glib()
    //     }
    //     let callback = Some(callback_func::<P> as _);
    //     unsafe extern "C" fn notify_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         data: glib_sys::gpointer,
    //     ) {
    //         let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    //     }
    //     let destroy_call6 = Some(notify_func::<P> as _);
    //     let super_callback0: Box_<P> = callback_data;
    //     unsafe {
    //         ffi::clutter_binding_pool_install_action(
    //             self.to_glib_none().0,
    //             action_name.to_glib_none().0,
    //             key_val,
    //             modifiers.to_glib(),
    //             callback,
    //             Box_::into_raw(super_callback0) as *mut _,
    //             destroy_call6,
    //         );
    //     }
    // }

    /// A `gobject::Closure` variant of `BindingPool::install_action`.
    ///
    /// Installs a new action inside a `BindingPool`. The action
    /// is bound to `key_val` and `modifiers`.
    ///
    /// The same action name can be used for multiple `key_val`, `modifiers`
    /// pairs.
    ///
    /// When an action has been activated using `BindingPool::activate`
    /// the passed `closure` will be invoked.
    ///
    /// Actions can be blocked with `BindingPool::block_action`
    /// and then unblocked using `BindingPool::unblock_action`.
    /// ## `action_name`
    /// the name of the action
    /// ## `key_val`
    /// key symbol
    /// ## `modifiers`
    /// bitmask of modifiers
    /// ## `closure`
    /// a `gobject::Closure`
    pub fn install_closure(
        &self,
        action_name: &str,
        key_val: u32,
        modifiers: ModifierType,
        closure: &glib::Closure,
    ) {
        unsafe {
            ffi::clutter_binding_pool_install_closure(
                self.to_glib_none().0,
                action_name.to_glib_none().0,
                key_val,
                modifiers.to_glib(),
                closure.to_glib_none().0,
            );
        }
    }

    // /// Allows overriding the action for `key_val` and `modifiers` inside a
    // /// `BindingPool`. See `BindingPool::install_action`.
    // ///
    // /// When an action has been activated using `BindingPool::activate`
    // /// the passed `callback` will be invoked (with `data`).
    // ///
    // /// Actions can be blocked with `BindingPool::block_action`
    // /// and then unblocked using `BindingPool::unblock_action`.
    // /// ## `key_val`
    // /// key symbol
    // /// ## `modifiers`
    // /// bitmask of modifiers
    // /// ## `callback`
    // /// function to be called when the action is activated
    // /// ## `data`
    // /// data to be passed to `callback`
    // /// ## `notify`
    // /// function to be called when the action is removed
    // ///  from the pool
    // pub fn override_action<P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static>(
    //     &self,
    //     key_val: u32,
    //     modifiers: ModifierType,
    //     callback: P,
    // ) {
    //     let callback_data: Box_<P> = Box_::new(callback);
    //     unsafe extern "C" fn callback_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         gobject: *mut gobject_sys::GObject,
    //         action_name: *const libc::c_char,
    //         key_val: libc::c_uint,
    //         modifiers: ffi::ClutterModifierType,
    //         user_data: glib_sys::gpointer,
    //     ) -> glib_sys::gboolean {
    //         let gobject = from_glib_borrow(gobject);
    //         let action_name: Borrowed<GString> = from_glib_borrow(action_name);
    //         let modifiers = from_glib_borrow(modifiers);
    //         let callback: &P = &*(user_data as *mut _);
    //         let res = (*callback)(&gobject, action_name.as_str(), key_val, &modifiers);
    //         res.to_glib()
    //     }
    //     let callback = Some(callback_func::<P> as _);
    //     unsafe extern "C" fn notify_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         data: glib_sys::gpointer,
    //     ) {
    //         let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    //     }
    //     let destroy_call5 = Some(notify_func::<P> as _);
    //     let super_callback0: Box_<P> = callback_data;
    //     unsafe {
    //         ffi::clutter_binding_pool_override_action(
    //             self.to_glib_none().0,
    //             key_val,
    //             modifiers.to_glib(),
    //             callback,
    //             Box_::into_raw(super_callback0) as *mut _,
    //             destroy_call5,
    //         );
    //     }
    // }

    /// A `gobject::Closure` variant of `BindingPool::override_action`.
    ///
    /// Allows overriding the action for `key_val` and `modifiers` inside a
    /// `BindingPool`. See `BindingPool::install_closure`.
    ///
    /// When an action has been activated using `BindingPool::activate`
    /// the passed `callback` will be invoked (with `data`).
    ///
    /// Actions can be blocked with `BindingPool::block_action`
    /// and then unblocked using `BindingPool::unblock_action`.
    /// ## `key_val`
    /// key symbol
    /// ## `modifiers`
    /// bitmask of modifiers
    /// ## `closure`
    /// a `gobject::Closure`
    pub fn override_closure(&self, key_val: u32, modifiers: ModifierType, closure: &glib::Closure) {
        unsafe {
            ffi::clutter_binding_pool_override_closure(
                self.to_glib_none().0,
                key_val,
                modifiers.to_glib(),
                closure.to_glib_none().0,
            );
        }
    }

    /// Removes the action matching the given `key_val`, `modifiers` pair,
    /// if any exists.
    /// ## `key_val`
    /// a key symbol
    /// ## `modifiers`
    /// a bitmask for the modifiers
    pub fn remove_action(&self, key_val: u32, modifiers: ModifierType) {
        unsafe {
            ffi::clutter_binding_pool_remove_action(
                self.to_glib_none().0,
                key_val,
                modifiers.to_glib(),
            );
        }
    }

    /// Unblockes all the actions with name `action_name` inside `self`.
    ///
    /// Unblocking an action does not cause the callback bound to it to
    /// be invoked in case `BindingPool::activate` was called on
    /// an action previously blocked with `BindingPool::block_action`.
    /// ## `action_name`
    /// an action name
    pub fn unblock_action(&self, action_name: &str) {
        unsafe {
            ffi::clutter_binding_pool_unblock_action(
                self.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    /// The unique name of the `BindingPool`.
    pub fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name` getter")
        }
    }

    /// Finds the `BindingPool` with `name`.
    /// ## `name`
    /// the name of the binding pool to find
    ///
    /// # Returns
    ///
    /// a pointer to the `BindingPool`, or `None`
    pub fn find(name: &str) -> Option<BindingPool> {
        unsafe { from_glib_none(ffi::clutter_binding_pool_find(name.to_glib_none().0)) }
    }

    //pub fn get_for_class(klass: /*Unimplemented*/Option<Fundamental: Pointer>) -> Option<BindingPool> {
    //    unsafe { TODO: call clutter_sys:clutter_binding_pool_get_for_class() }
    //}
}

impl fmt::Display for BindingPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BindingPool")
    }
}
