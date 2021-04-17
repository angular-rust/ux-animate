glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Geometry(Boxed<ffi::ClutterGeometry>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ffi::clutter_geometry_get_type(), ptr as *mut _) as *mut ffi::ClutterGeometry,
        free => |ptr| gobject_sys::g_boxed_free(ffi::clutter_geometry_get_type(), ptr as *mut _),
        get_type => || ffi::clutter_geometry_get_type(),
    }
}
