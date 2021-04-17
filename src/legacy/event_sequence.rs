glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventSequence(Boxed<ffi::ClutterEventSequence>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_event_sequence_get_type(), ptr as *mut _) as *mut ffi::ClutterEventSequence,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_event_sequence_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_event_sequence_get_type(),
    }
}
