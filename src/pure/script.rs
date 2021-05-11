// use glib::{
//     signal::{connect_raw, SignalHandlerId},

// };
// use std::boxed::Box as Box_;
// use std::{fmt, mem::transmute, ptr};

// glib_wrapper! {
//     pub struct Script(Object<ffi::ClutterScript, ffi::ClutterScriptClass, ScriptClass>);

//     match fn {
//         get_type => || ffi::clutter_script_get_type(),
//     }
// }

// impl Script {
//     /// Creates a new `Script` instance. `Script` can be used
//     /// to load objects definitions for scenegraph elements, like actors,
//     /// or behavioural elements, like behaviours and timelines. The
//     /// definitions must be encoded using the JavaScript Object Notation (JSON)
//     /// language.
//     ///
//     /// # Returns
//     ///
//     /// the newly created `Script` instance. Use
//     ///  `gobject::ObjectExt::unref` when done.
//     pub fn new() -> Script {
//         unsafe { from_glib_full(ffi::clutter_script_new()) }
//     }
// }

// impl Default for Script {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// pub const NONE_SCRIPT: Option<&Script> = None;

// /// Trait containing all `Script` methods.
// ///
// /// # Implementors
// ///
// /// [`Script`](struct.Script.html)
// pub trait ScriptExt: 'static {
//     /// Adds `paths` to the list of search paths held by `self`.
//     ///
//     /// The search paths are used by `ScriptExt::lookup_filename`, which
//     /// can be used to define search paths for the textures source file name
//     /// or other custom, file-based properties.
//     /// ## `paths`
//     /// an array of strings containing
//     ///  different search paths
//     /// ## `n_paths`
//     /// the length of the passed array
//     fn add_search_paths(&self, paths: &[&str]);

//     //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

//     //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Script, &glib::Object, &str, &str, &glib::Object, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

//     /// Ensure that every object defined inside `self` is correctly
//     /// constructed. You should rarely need to use this function.
//     fn ensure_objects(&self);

//     /// Retrieves the object bound to `name`. This function does not increment
//     /// the reference count of the returned object.
//     /// ## `name`
//     /// the name of the object to retrieve
//     ///
//     /// # Returns
//     ///
//     /// the named object, or `None` if no object
//     ///  with the given name was available
//     fn get_object(&self, name: &str) -> Option<glib::Object>;

//     //fn get_objects(&self, first_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> i32;

//     /// Retrieves the translation domain set using
//     /// `ScriptExt::set_translation_domain`.
//     ///
//     /// # Returns
//     ///
//     /// the translation domain, if any is set,
//     ///  or `None`
//     fn get_translation_domain(&self) -> Option<GString>;

//     /// Looks up a type by name, using the virtual function that
//     /// `Script` has for that purpose. This function should
//     /// rarely be used.
//     /// ## `type_name`
//     /// name of the type to look up
//     ///
//     /// # Returns
//     ///
//     /// the type for the requested type name, or
//     ///  `G_TYPE_INVALID` if not corresponding type was found.
//     fn get_type_from_name(&self, type_name: &str) -> glib::types::Type;

//     /// Retrieves all the objects created by `self`.
//     ///
//     /// Note: this function does not increment the reference count of the
//     /// objects it returns.
//     ///
//     /// # Returns
//     ///
//     /// a list
//     ///  of `gobject::Object`<!-- -->s, or `None`. The objects are owned by the
//     ///  `Script` instance. Use `glib::List::free` on the returned list when
//     ///  done.
//     fn list_objects(&self) -> Vec<glib::Object>;

//     /// Loads the definitions from `data` into `self` and merges with
//     /// the currently loaded ones, if any.
//     /// ## `data`
//     /// a buffer containing the definitions
//     /// ## `length`
//     /// the length of the buffer, or -1 if `data` is a NUL-terminated
//     ///  buffer
//     ///
//     /// # Returns
//     ///
//     /// on error, zero is returned and `error` is set
//     ///  accordingly. On success, the merge id for the UI definitions is
//     ///  returned. You can use the merge id with `ScriptExt::unmerge_objects`.
//     fn load_from_data(&self, data: &str) -> Result<(), glib::Error>;

//     /// Loads the definitions from `filename` into `self` and merges with
//     /// the currently loaded ones, if any.
//     /// ## `filename`
//     /// the full path to the definition file
//     ///
//     /// # Returns
//     ///
//     /// on error, zero is returned and `error` is set
//     ///  accordingly. On success, the merge id for the UI definitions is
//     ///  returned. You can use the merge id with `ScriptExt::unmerge_objects`.
//     fn load_from_file(&self, filename: &str) -> Result<(), glib::Error>;

//     /// Loads the definitions from a resource file into `self` and merges with
//     /// the currently loaded ones, if any.
//     /// ## `resource_path`
//     /// the resource path of the file to parse
//     ///
//     /// # Returns
//     ///
//     /// on error, zero is returned and `error` is set
//     ///  accordingly. On success, the merge id for the UI definitions is
//     ///  returned. You can use the merge id with `ScriptExt::unmerge_objects`.
//     fn load_from_resource(&self, resource_path: &str) -> Result<(), glib::Error>;

//     /// Looks up `filename` inside the search paths of `self`. If `filename`
//     /// is found, its full path will be returned .
//     /// ## `filename`
//     /// the name of the file to lookup
//     ///
//     /// # Returns
//     ///
//     /// the full path of `filename` or `None` if no path was
//     ///  found.
//     fn lookup_filename(&self, filename: &str) -> Option<GString>;

//     /// Sets the translation domain for `self`.
//     /// ## `domain`
//     /// the translation domain, or `None`
//     fn set_translation_domain(&self, domain: Option<&str>);

//     /// Unmerges the objects identified by `merge_id`.
//     /// ## `merge_id`
//     /// merge id returned when loading a UI definition
//     fn unmerge_objects(&self, merge_id: u32);

//     /// The path of the currently parsed file. If `Script:filename-set`
//     /// is `false` then the value of this property is undefined.
//     fn get_property_filename(&self) -> Option<GString>;

//     /// Whether the `Script:filename` property is set. If this property
//     /// is `true` then the currently parsed data comes from a file, and the
//     /// file name is stored inside the `Script:filename` property.
//     fn get_property_filename_set(&self) -> bool;

//     fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

//     fn connect_property_filename_set_notify<F: Fn(&Self) + 'static>(&self, f: F)
//         -> SignalHandlerId;

//     fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId;
// }

// impl<O: Is<Script>> ScriptExt for O {
//     fn add_search_paths(&self, paths: &[&str]) {
//         let n_paths = paths.len() as usize;
//         unsafe {
//             ffi::clutter_script_add_search_paths(
//                 self.as_ref().to_glib_none().0,
//                 paths.to_glib_none().0,
//                 n_paths,
//             );
//         }
//     }

//     //fn connect_signals(&self, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//     //    unsafe { TODO: call clutter_sys:clutter_script_connect_signals() }
//     //}

//     //fn connect_signals_full(&self, func: /*Unimplemented*/FnMut(&Script, &glib::Object, &str, &str, &glib::Object, /*Ignored*/glib::ConnectFlags), user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//     //    unsafe { TODO: call clutter_sys:clutter_script_connect_signals_full() }
//     //}

//     fn ensure_objects(&self) {
//         unsafe {
//             ffi::clutter_script_ensure_objects(self.as_ref().to_glib_none().0);
//         }
//     }

//     fn get_object(&self, name: &str) -> Option<glib::Object> {
//         unsafe {
//             from_glib_none(ffi::clutter_script_get_object(
//                 self.as_ref().to_glib_none().0,
//                 name.to_glib_none().0,
//             ))
//         }
//     }

//     //fn get_objects(&self, first_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> i32 {
//     //    unsafe { TODO: call clutter_sys:clutter_script_get_objects() }
//     //}

//     fn get_translation_domain(&self) -> Option<GString> {
//         unsafe {
//             from_glib_none(ffi::clutter_script_get_translation_domain(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn get_type_from_name(&self, type_name: &str) -> glib::types::Type {
//         unsafe {
//             from_glib(ffi::clutter_script_get_type_from_name(
//                 self.as_ref().to_glib_none().0,
//                 type_name.to_glib_none().0,
//             ))
//         }
//     }

//     fn list_objects(&self) -> Vec<glib::Object> {
//         unsafe {
//             FromGlibPtrContainer::from_glib_container(ffi::clutter_script_list_objects(
//                 self.as_ref().to_glib_none().0,
//             ))
//         }
//     }

//     fn load_from_data(&self, data: &str) -> Result<(), glib::Error> {
//         let length = data.len() as isize;
//         unsafe {
//             let mut error = ptr::null_mut();
//             let _ = ffi::clutter_script_load_from_data(
//                 self.as_ref().to_glib_none().0,
//                 data.to_glib_none().0,
//                 length,
//                 &mut error,
//             );
//             if error.is_null() {
//                 Ok(())
//             } else {
//                 Err(from_glib_full(error))
//             }
//         }
//     }

//     fn load_from_file(&self, filename: &str) -> Result<(), glib::Error> {
//         unsafe {
//             let mut error = ptr::null_mut();
//             let _ = ffi::clutter_script_load_from_file(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//                 &mut error,
//             );
//             if error.is_null() {
//                 Ok(())
//             } else {
//                 Err(from_glib_full(error))
//             }
//         }
//     }

//     fn load_from_resource(&self, resource_path: &str) -> Result<(), glib::Error> {
//         unsafe {
//             let mut error = ptr::null_mut();
//             let _ = ffi::clutter_script_load_from_resource(
//                 self.as_ref().to_glib_none().0,
//                 resource_path.to_glib_none().0,
//                 &mut error,
//             );
//             if error.is_null() {
//                 Ok(())
//             } else {
//                 Err(from_glib_full(error))
//             }
//         }
//     }

//     fn lookup_filename(&self, filename: &str) -> Option<GString> {
//         unsafe {
//             from_glib_full(ffi::clutter_script_lookup_filename(
//                 self.as_ref().to_glib_none().0,
//                 filename.to_glib_none().0,
//             ))
//         }
//     }

//     fn set_translation_domain(&self, domain: Option<&str>) {
//         unsafe {
//             ffi::clutter_script_set_translation_domain(
//                 self.as_ref().to_glib_none().0,
//                 domain.to_glib_none().0,
//             );
//         }
//     }

//     fn unmerge_objects(&self, merge_id: u32) {
//         unsafe {
//             ffi::clutter_script_unmerge_objects(self.as_ref().to_glib_none().0, merge_id);
//         }
//     }

//     fn get_property_filename(&self) -> Option<GString> {
//         unsafe {
//             let mut value = Value::from_type(<GString as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"filename\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `filename` getter")
//         }
//     }

//     fn get_property_filename_set(&self) -> bool {
//         unsafe {
//             let mut value = Value::from_type(<bool as StaticType>::static_type());
//             gobject_sys::g_object_get_property(
//                 self.to_glib_none().0 as *mut gobject_sys::GObject,
//                 b"filename-set\0".as_ptr() as *const _,
//                 value.to_glib_none_mut().0,
//             );
//             value
//                 .get()
//                 .expect("Return Value for property `filename-set` getter")
//                 .unwrap()
//         }
//     }

//     fn connect_property_filename_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
//         unsafe extern "C" fn notify_filename_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ClutterScript,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Script>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Script::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::filename\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_filename_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_filename_set_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_filename_set_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ClutterScript,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Script>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Script::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::filename-set\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_filename_set_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }

//     fn connect_property_translation_domain_notify<F: Fn(&Self) + 'static>(
//         &self,
//         f: F,
//     ) -> SignalHandlerId {
//         unsafe extern "C" fn notify_translation_domain_trampoline<P, F: Fn(&P) + 'static>(
//             this: *mut ffi::ClutterScript,
//             _param_spec: glib_sys::gpointer,
//             f: glib_sys::gpointer,
//         ) where
//             P: Is<Script>,
//         {
//             let f: &F = &*(f as *const F);
//             f(&Script::from_glib_borrow(this).unsafe_cast_ref())
//         }
//         unsafe {
//             let f: Box_<F> = Box_::new(f);
//             connect_raw(
//                 self.as_ptr() as *mut _,
//                 b"notify::translation-domain\0".as_ptr() as *const _,
//                 Some(transmute::<_, unsafe extern "C" fn()>(
//                     notify_translation_domain_trampoline::<Self, F> as *const (),
//                 )),
//                 Box_::into_raw(f),
//             )
//         }
//     }
// }

// impl fmt::Display for Script {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Script")
//     }
// }
