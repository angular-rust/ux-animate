use crate::Content;
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct ActorCanvas(Object<ffi::ClutterCanvas, ffi::ClutterCanvasClass, CanvasClass>) @implements Content;

    match fn {
        get_type => || ffi::clutter_canvas_get_type(),
    }
}

impl ActorCanvas {
    /// Creates a new instance of `Canvas`.
    ///
    /// You should call `CanvasExt::set_size` to set the size of the canvas.
    ///
    /// You should call `Content::invalidate` every time you wish to
    /// draw the contents of the canvas.
    ///
    /// # Returns
    ///
    /// The newly allocated instance of
    ///  `Canvas`. Use `gobject::ObjectExt::unref` when done.
    pub fn new() -> Option<Content> {
        unsafe { from_glib_full(ffi::clutter_canvas_new()) }
    }
}

// impl Default for Canvas {
//     fn default() -> Self {
//         Self::new()
//     }
// }

/// Trait containing all `Canvas` methods.
///
/// # Implementors
///
/// [`Canvas`](struct.Canvas.html)
pub trait ActorCanvasExt: 'static {
    /// Retrieves the scaling factor of `self`, as set using
    /// `CanvasExt::set_scale_factor`.
    ///
    /// # Returns
    ///
    /// the scaling factor, or -1 if the `self`
    ///  uses the default from `Settings`
    fn get_scale_factor(&self) -> i32;

    /// Sets the scaling factor for the Cairo surface used by `self`.
    ///
    /// This function should rarely be used.
    ///
    /// The default scaling factor of a `Canvas` content uses the
    /// `Settings:window-scaling-factor` property, which is set by
    /// the windowing system. By using this function it is possible to
    /// override that setting.
    ///
    /// Changing the scale factor will invalidate the `self`.
    /// ## `scale`
    /// the scale factor, or -1 for the default
    fn set_scale_factor(&self, scale: i32);

    /// Sets the size of the `self`, and invalidates the content.
    ///
    /// This function will cause the `self` to be invalidated only
    /// if the size of the canvas surface has changed.
    ///
    /// If you want to invalidate the contents of the `self` when setting
    /// the size, you can use the return value of the function to conditionally
    /// call `Content::invalidate`:
    ///
    ///
    /// ```text
    ///   if (!canvas_set_size (canvas, width, height))
    ///     content_invalidate (CONTENT (canvas));
    /// ```
    /// ## `width`
    /// the width of the canvas, in pixels
    /// ## `height`
    /// the height of the canvas, in pixels
    ///
    /// # Returns
    ///
    /// this function returns `true` if the size change
    ///  caused a content invalidation, and `false` otherwise
    fn set_size(&self, width: i32, height: i32) -> bool;

    /// The height of the canvas.
    fn get_property_height(&self) -> i32;

    /// The height of the canvas.
    fn set_property_height(&self, height: i32);

    /// Whether the `Canvas:scale-factor` property is set.
    ///
    /// If the `Canvas:scale-factor-set` property is `false`
    /// then `Canvas` will use the `Settings:window-scaling-factor`
    /// property.
    fn get_property_scale_factor_set(&self) -> bool;

    /// The width of the canvas.
    fn get_property_width(&self) -> i32;

    /// The width of the canvas.
    fn set_property_width(&self, width: i32);

    /// The `Canvas::draw` signal is emitted each time a canvas is
    /// invalidated.
    ///
    /// It is safe to connect multiple handlers to this signal: each
    /// handler invocation will be automatically protected by `cairo_save`
    /// and `cairo_restore` pairs.
    /// ## `cr`
    /// the Cairo context used to draw
    /// ## `width`
    /// the width of the `canvas`
    /// ## `height`
    /// the height of the `canvas`
    ///
    /// # Returns
    ///
    /// `true` if the signal emission should stop, and
    ///  `false` otherwise
    fn connect_draw<F: Fn(&Self, &cairo::Context, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_scale_factor_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActorCanvas>> ActorCanvasExt for O {
    fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::clutter_canvas_get_scale_factor(self.as_ref().to_glib_none().0) }
    }

    fn set_scale_factor(&self, scale: i32) {
        unsafe {
            ffi::clutter_canvas_set_scale_factor(self.as_ref().to_glib_none().0, scale);
        }
    }

    fn set_size(&self, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::clutter_canvas_set_size(
                self.as_ref().to_glib_none().0,
                width,
                height,
            ))
        }
    }

    fn get_property_height(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `height` getter")
                .unwrap()
        }
    }

    fn set_property_height(&self, height: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"height\0".as_ptr() as *const _,
                Value::from(&height).to_glib_none().0,
            );
        }
    }

    fn get_property_scale_factor_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"scale-factor-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `scale-factor-set` getter")
                .unwrap()
        }
    }

    fn get_property_width(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `width` getter")
                .unwrap()
        }
    }

    fn set_property_width(&self, width: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"width\0".as_ptr() as *const _,
                Value::from(&width).to_glib_none().0,
            );
        }
    }

    fn connect_draw<F: Fn(&Self, &cairo::Context, i32, i32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn draw_trampoline<
            P,
            F: Fn(&P, &cairo::Context, i32, i32) -> bool + 'static,
        >(
            this: *mut ffi::ClutterCanvas,
            cr: *mut cairo_sys::cairo_t,
            width: libc::c_int,
            height: libc::c_int,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ActorCanvas>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ActorCanvas::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(cr),
                width,
                height,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"draw\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    draw_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterCanvas,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorCanvas>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorCanvas::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterCanvas,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorCanvas>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorCanvas::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_scale_factor_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterCanvas,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorCanvas>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorCanvas::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterCanvas,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ActorCanvas>,
        {
            let f: &F = &*(f as *const F);
            f(&ActorCanvas::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ActorCanvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Canvas")
    }
}
