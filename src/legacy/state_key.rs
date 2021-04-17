glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StateKey(Boxed<ffi::ClutterStateKey>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_state_key_get_type(), ptr as *mut _) as *mut ffi::ClutterStateKey,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_state_key_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_state_key_get_type(),
    }
}
