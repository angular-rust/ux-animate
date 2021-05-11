use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable, Animatable, Container
// @extends Actor
#[derive(Debug, Clone)]
pub struct Texture {}

impl Object for Texture {}
impl Is<Texture> for Texture {}

impl AsRef<Texture> for Texture {
    fn as_ref(&self) -> &Texture {
        self
    }
}

/// Trait containing all `Texture` methods.
///
/// # Implementors
///
/// [`Texture`](struct.Texture.html)
pub trait TextureExt: 'static {
    fn get_property_disable_slicing(&self) -> bool;

    //fn get_property_filter_quality(&self) -> /*Ignored*/TextureQuality;

    //fn set_property_filter_quality(&self, filter_quality: /*Ignored*/TextureQuality);

    fn get_property_keep_aspect_ratio(&self) -> bool;

    fn set_property_keep_aspect_ratio(&self, keep_aspect_ratio: bool);

    fn get_property_pick_with_alpha(&self) -> bool;

    fn set_property_pick_with_alpha(&self, pick_with_alpha: bool);

    fn get_property_pixel_format(&self) -> dx::PixelFormat;

    fn get_property_repeat_x(&self) -> bool;

    fn set_property_repeat_x(&self, repeat_x: bool);

    fn get_property_repeat_y(&self) -> bool;

    fn set_property_repeat_y(&self, repeat_y: bool);

    fn get_property_sync_size(&self) -> bool;

    fn set_property_sync_size(&self, sync_size: bool);

    fn get_property_tile_waste(&self) -> i32;

    fn connect_property_filter_quality_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pick_with_alpha_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pixel_format_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_repeat_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_repeat_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sync_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_tile_waste_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Texture>> TextureExt for O {
    fn get_property_disable_slicing(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"disable-slicing\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `disable-slicing` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    //fn get_property_filter_quality(&self) -> /*Ignored*/TextureQuality {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-quality\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `filter-quality` getter").unwrap()
    //    }
    //}

    //fn set_property_filter_quality(&self, filter_quality: /*Ignored*/TextureQuality) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-quality\0".as_ptr() as *const _, Value::from(&filter_quality).to_glib_none().0);
    //    }
    //}

    fn get_property_keep_aspect_ratio(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"keep-aspect-ratio\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `keep-aspect-ratio` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"keep-aspect-ratio\0".as_ptr() as *const _,
        //         Value::from(&keep_aspect_ratio).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_pick_with_alpha(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"pick-with-alpha\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `pick-with-alpha` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_pick_with_alpha(&self, pick_with_alpha: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"pick-with-alpha\0".as_ptr() as *const _,
        //         Value::from(&pick_with_alpha).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_pixel_format(&self) -> dx::PixelFormat {
        // unsafe {
        //     let mut value = Value::from_type(<dx::PixelFormat as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"pixel-format\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `pixel-format` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_repeat_x(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"repeat-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `repeat-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_repeat_x(&self, repeat_x: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"repeat-x\0".as_ptr() as *const _,
        //         Value::from(&repeat_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_repeat_y(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"repeat-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `repeat-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_repeat_y(&self, repeat_y: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"repeat-y\0".as_ptr() as *const _,
        //         Value::from(&repeat_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_sync_size(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"sync-size\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `sync-size` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_sync_size(&self, sync_size: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"sync-size\0".as_ptr() as *const _,
        //         Value::from(&sync_size).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_tile_waste(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"tile-waste\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `tile-waste` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn connect_property_filter_quality_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_pick_with_alpha_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_pixel_format_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_repeat_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_repeat_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_sync_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_tile_waste_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture")
    }
}
