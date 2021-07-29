#![allow(unused_imports)]
use super::ModifierType;
use std::{fmt, collections::HashMap};

struct BindingEntry {
    // gchar *name; /* interned string, do not free */

    // guint key_val;
    // ModifierType modifiers;
  
    // GClosure *closure;
  
    // guint is_blocked  : 1;
}
// @short_description: Pool for key bindings
//
// #BindingPool is a data structure holding a set of key bindings.
// Each key binding associates a key symbol (eventually with modifiers)
// to an action. A callback function is associated to each action.
//
// For a given key symbol and modifier mask combination there can be only one
// action; for each action there can be only one callback. There can be
// multiple actions with the same name, and the same callback can be used
// to handle multiple key bindings.
//
// Actors requiring key bindings should create a new #BindingPool
// inside their class initialization function and then install actions
// like this:
//
// |[<!-- language="C" -->
// static void
// foo_class_init (FooClass *klass)
// {
//   BindingPool *binding_pool;
//
//   binding_pool = clutter_binding_pool_get_for_class (klass);
//
//   clutter_binding_pool_install_action (binding_pool, "move-up",
//                                        CLUTTER_Up, 0,
//                                        G_CALLBACK (foo_action_move_up),
//                                        NULL, NULL);
//   clutter_binding_pool_install_action (binding_pool, "move-up",
//                                        CLUTTER_KP_Up, 0,
//                                        G_CALLBACK (foo_action_move_up),
//                                        NULL, NULL);
// }
// ]|
//
// The callback has a signature of:
//
// |[<!-- language="C" -->
//    gboolean (* callback) (GObject             *instance,
//                           const gchar         *action_name,
//                           guint                key_val,
//                           ModifierType  modifiers,
//                           gpointer             user_data);
// ]|
//
// The actor should then override the #Actor::key-press-event and
// use clutter_binding_pool_activate() to match a #KeyEvent structure
// to one of the actions:
//
// |[<!-- language="C" -->
//   BindingPool *pool;
//
//   // retrieve the binding pool for the type of the actor
//   pool = clutter_binding_pool_find (G_OBJECT_TYPE_NAME (actor));
//
//   // activate any callback matching the key symbol and modifiers
//   // mask of the key event. the returned value can be directly
//   // used to signal that the actor has handled the event.
//   return clutter_binding_pool_activate (pool,
//                                         key_event->keyval,
//                                         key_event->modifier_state,
//                                         G_OBJECT (actor));
// ]|
//
// The clutter_binding_pool_activate() function will return %FALSE if
// no action for the given key binding was found, if the action was
// blocked (using clutter_binding_pool_block_action()) or if the
// key binding handler returned %FALSE.
pub struct BindingPool {
    // GObject parent_instance;
    name: Option<String>, /* interned string, do not free */
    entries: Vec<BindingEntry>,
    entries_hash: HashMap<String, BindingEntry>,
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
        unimplemented!()
    }

    // /// Activates the callback associated to the action that is
    // /// bound to the `key_val` and `modifiers` pair.
    // ///
    // /// The callback has the following signature:
    // ///
    // ///
    // /// ```text
    // ///   void (* callback) (GObject             *gobject,
    // ///                      const gchar         *action_name,
    // ///                      guint                key_val,
    // ///                      ModifierType  modifiers,
    // ///                      gpointer             user_data);
    // /// ```
    // ///
    // /// Where the `gobject::Object` instance is `gobject` and the user data
    // /// is the one passed when installing the action with
    // /// `BindingPool::install_action`.
    // ///
    // /// If the action bound to the `key_val`, `modifiers` pair has been
    // /// blocked using `BindingPool::block_action`, the callback
    // /// will not be invoked, and this function will return `false`.
    // /// ## `key_val`
    // /// the key symbol
    // /// ## `modifiers`
    // /// bitmask for the modifiers
    // /// ## `gobject`
    // /// a `gobject::Object`
    // ///
    // /// # Returns
    // ///
    // /// `true` if an action was found and was activated
    // pub fn activate<P: Is<Object>>(
    //     &self,
    //     key_val: u32,
    //     modifiers: ModifierType,
    //     gobject: &P,
    // ) -> bool {
    //     unimplemented!()
    // }

    /// Blocks all the actions with name `action_name` inside `self`.
    /// ## `action_name`
    /// an action name
    pub fn block_action(&self, action_name: &str) {
        unimplemented!()
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
    pub fn find_action(&self, key_val: u32, modifiers: ModifierType) -> Option<String> {
        unimplemented!()
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
    //     let callback_data: Box<P> = Box::new(callback);
    //     unsafe extern "C" fn callback_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         gobject: *mut gobject_sys::GObject,
    //         action_name: *const libc::c_char,
    //         key_val: libc::c_uint,
    //         modifiers: ffi::ModifierType,
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
    //         let _callback: Box<P> = Box::from_raw(data as *mut _);
    //     }
    //     let destroy_call6 = Some(notify_func::<P> as _);
    //     let super_callback0: Box<P> = callback_data;
    //     unsafe {
    //         ffi::clutter_binding_pool_install_action(
    //             self.to_glib_none().0,
    //             action_name.to_glib_none().0,
    //             key_val,
    //             modifiers.to_glib(),
    //             callback,
    //             Box::into_raw(super_callback0) as *mut _,
    //             destroy_call6,
    //         );
    //     }
    // }

    // /// A `gobject::Closure` variant of `BindingPool::install_action`.
    // ///
    // /// Installs a new action inside a `BindingPool`. The action
    // /// is bound to `key_val` and `modifiers`.
    // ///
    // /// The same action name can be used for multiple `key_val`, `modifiers`
    // /// pairs.
    // ///
    // /// When an action has been activated using `BindingPool::activate`
    // /// the passed `closure` will be invoked.
    // ///
    // /// Actions can be blocked with `BindingPool::block_action`
    // /// and then unblocked using `BindingPool::unblock_action`.
    // /// ## `action_name`
    // /// the name of the action
    // /// ## `key_val`
    // /// key symbol
    // /// ## `modifiers`
    // /// bitmask of modifiers
    // /// ## `closure`
    // /// a `gobject::Closure`
    // pub fn install_closure(
    //     &self,
    //     action_name: &str,
    //     key_val: u32,
    //     modifiers: ModifierType,
    //     closure: &glib::Closure,
    // ) {
    //     unimplemented!()
    // }

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
    //     let callback_data: Box<P> = Box::new(callback);
    //     unsafe extern "C" fn callback_func<
    //         P: Fn(&glib::Object, &str, u32, &ModifierType) -> bool + 'static,
    //     >(
    //         gobject: *mut gobject_sys::GObject,
    //         action_name: *const libc::c_char,
    //         key_val: libc::c_uint,
    //         modifiers: ffi::ModifierType,
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
    //         let _callback: Box<P> = Box::from_raw(data as *mut _);
    //     }
    //     let destroy_call5 = Some(notify_func::<P> as _);
    //     let super_callback0: Box<P> = callback_data;
    //     unsafe {
    //         ffi::clutter_binding_pool_override_action(
    //             self.to_glib_none().0,
    //             key_val,
    //             modifiers.to_glib(),
    //             callback,
    //             Box::into_raw(super_callback0) as *mut _,
    //             destroy_call5,
    //         );
    //     }
    // }

    // /// A `gobject::Closure` variant of `BindingPool::override_action`.
    // ///
    // /// Allows overriding the action for `key_val` and `modifiers` inside a
    // /// `BindingPool`. See `BindingPool::install_closure`.
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
    // /// ## `closure`
    // /// a `gobject::Closure`
    // pub fn override_closure(&self, key_val: u32, modifiers: ModifierType, closure: &glib::Closure) {
    //     unimplemented!()
    // }

    /// Removes the action matching the given `key_val`, `modifiers` pair,
    /// if any exists.
    /// ## `key_val`
    /// a key symbol
    /// ## `modifiers`
    /// a bitmask for the modifiers
    pub fn remove_action(&self, key_val: u32, modifiers: ModifierType) {
        unimplemented!()
    }

    /// Unblockes all the actions with name `action_name` inside `self`.
    ///
    /// Unblocking an action does not cause the callback bound to it to
    /// be invoked in case `BindingPool::activate` was called on
    /// an action previously blocked with `BindingPool::block_action`.
    /// ## `action_name`
    /// an action name
    pub fn unblock_action(&self, action_name: &str) {
        unimplemented!()
    }

    /// The unique name of the `BindingPool`.
    pub fn get_property_name(&self) -> Option<String> {
        unimplemented!()
    }

    /// Finds the `BindingPool` with `name`.
    /// ## `name`
    /// the name of the binding pool to find
    ///
    /// # Returns
    ///
    /// a pointer to the `BindingPool`, or `None`
    pub fn find(name: &str) -> Option<BindingPool> {
        unimplemented!()
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
