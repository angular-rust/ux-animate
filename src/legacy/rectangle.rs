// Scriptable
use crate::{Actor, Animatable, Container, InternalColor, Color, RgbaColor};
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
    pub struct Rectangle(Object<ffi::ClutterRectangle, ffi::ClutterRectangleClass, RectangleClass>) @extends Actor, gobject::InitiallyUnowned, @implements Animatable, Container;

    match fn {
        get_type => || ffi::clutter_rectangle_get_type(),
    }
}

/// Trait containing all `Rectangle` methods.
///
/// # Implementors
///
/// [`Rectangle`](struct.Rectangle.html)
pub trait RectangleExt: 'static {
    /// The color of the border of the rectangle.
    fn get_property_border_color(&self) -> Option<InternalColor>;

    /// The color of the border of the rectangle.
    fn set_property_border_color(&self, border_color: Option<Color>);

    /// The width of the border of the rectangle, in pixels.
    fn get_property_border_width(&self) -> u32;

    /// The width of the border of the rectangle, in pixels.
    fn set_property_border_width(&self, border_width: u32);

    /// The color of the rectangle.
    fn get_property_color(&self) -> Option<InternalColor>;

    /// The color of the rectangle.
    fn set_property_color(&self, color: Option<Color>);

    /// Whether the `Rectangle` should be displayed with a border.
    fn get_property_has_border(&self) -> bool;

    /// Whether the `Rectangle` should be displayed with a border.
    fn set_property_has_border(&self, has_border: bool);

    fn connect_property_border_color_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_border_width_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Rectangle>> RectangleExt for O {
    fn get_property_border_color(&self) -> Option<InternalColor> {
        unsafe {
            let mut value = Value::from_type(<InternalColor as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"border-color\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `border-color` getter")
        }
    } 

    fn set_property_border_color(&self, color: Option<Color>) {
        let color = match color {
            Some(value) => {
                let RgbaColor {
                    red,
                    green,
                    blue,
                    alpha,
                } = value.into();
                Some(InternalColor::new(red, green, blue, alpha))
            }
            None => None,
        };
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"border-color\0".as_ptr() as *const _,
                Value::from(color.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_border_width(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"border-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `border-width` getter")
                .unwrap()
        }
    }

    fn set_property_border_width(&self, border_width: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"border-width\0".as_ptr() as *const _,
                Value::from(&border_width).to_glib_none().0,
            );
        }
    }

    fn get_property_color(&self) -> Option<InternalColor> {
        unsafe {
            let mut value = Value::from_type(<InternalColor as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"color\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `color` getter")
        }
    }

    fn set_property_color(&self, color: Option<Color>) {
        let color = match color {
            Some(value) => {
                let RgbaColor {
                    red,
                    green,
                    blue,
                    alpha,
                } = value.into();
                Some(InternalColor::new(red, green, blue, alpha))
            }
            None => None,
        };
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"color\0".as_ptr() as *const _,
                Value::from(color.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_has_border(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"has-border\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-border` getter")
                .unwrap()
        }
    }

    fn set_property_has_border(&self, has_border: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"has-border\0".as_ptr() as *const _,
                Value::from(&has_border).to_glib_none().0,
            );
        }
    }

    fn connect_property_border_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_border_color_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterRectangle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Rectangle>,
        {
            let f: &F = &*(f as *const F);
            f(&Rectangle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::border-color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_border_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_border_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_border_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterRectangle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Rectangle>,
        {
            let f: &F = &*(f as *const F);
            f(&Rectangle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::border-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_border_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_color_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterRectangle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Rectangle>,
        {
            let f: &F = &*(f as *const F);
            f(&Rectangle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::color\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_color_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_has_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_border_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterRectangle,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Rectangle>,
        {
            let f: &F = &*(f as *const F);
            f(&Rectangle::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-border\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_border_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle")
    }
}
