// Scriptable
use crate::{Actor, Animatable, Container};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

// TODO: implements atk::ImplementorIface, Scriptable
glib_wrapper! {
    pub struct Texture(Object<ffi::ClutterTexture, ffi::ClutterTextureClass, TextureClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_texture_get_type(),
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

impl<O: IsA<Texture>> TextureExt for O {
    fn get_property_disable_slicing(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"disable-slicing\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `disable-slicing` getter")
                .unwrap()
        }
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
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"keep-aspect-ratio\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `keep-aspect-ratio` getter")
                .unwrap()
        }
    }

    fn set_property_keep_aspect_ratio(&self, keep_aspect_ratio: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"keep-aspect-ratio\0".as_ptr() as *const _,
                Value::from(&keep_aspect_ratio).to_glib_none().0,
            );
        }
    }

    fn get_property_pick_with_alpha(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pick-with-alpha\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pick-with-alpha` getter")
                .unwrap()
        }
    }

    fn set_property_pick_with_alpha(&self, pick_with_alpha: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pick-with-alpha\0".as_ptr() as *const _,
                Value::from(&pick_with_alpha).to_glib_none().0,
            );
        }
    }

    fn get_property_pixel_format(&self) -> dx::PixelFormat {
        unsafe {
            let mut value = Value::from_type(<dx::PixelFormat as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pixel-format\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pixel-format` getter")
                .unwrap()
        }
    }

    fn get_property_repeat_x(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"repeat-x\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `repeat-x` getter")
                .unwrap()
        }
    }

    fn set_property_repeat_x(&self, repeat_x: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"repeat-x\0".as_ptr() as *const _,
                Value::from(&repeat_x).to_glib_none().0,
            );
        }
    }

    fn get_property_repeat_y(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"repeat-y\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `repeat-y` getter")
                .unwrap()
        }
    }

    fn set_property_repeat_y(&self, repeat_y: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"repeat-y\0".as_ptr() as *const _,
                Value::from(&repeat_y).to_glib_none().0,
            );
        }
    }

    fn get_property_sync_size(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"sync-size\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `sync-size` getter")
                .unwrap()
        }
    }

    fn set_property_sync_size(&self, sync_size: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"sync-size\0".as_ptr() as *const _,
                Value::from(&sync_size).to_glib_none().0,
            );
        }
    }

    fn get_property_tile_waste(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"tile-waste\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tile-waste` getter")
                .unwrap()
        }
    }

    fn connect_property_filter_quality_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_quality_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter-quality\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_quality_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_keep_aspect_ratio_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_keep_aspect_ratio_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::keep-aspect-ratio\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_keep_aspect_ratio_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pick_with_alpha_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pick_with_alpha_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pick-with-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pick_with_alpha_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pixel_format_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixel_format_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pixel-format\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pixel_format_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_repeat_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_repeat_x_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::repeat-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_repeat_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_repeat_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_repeat_y_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::repeat-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_repeat_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_sync_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sync_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sync-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sync_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_tile_waste_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tile_waste_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTexture,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Texture>,
        {
            let f: &F = &*(f as *const F);
            f(&Texture::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tile-waste\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tile_waste_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture")
    }
}
