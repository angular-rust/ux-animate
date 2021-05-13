use crate::prelude::*;
use crate::Color;
use glib::signal::SignalHandlerId;
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable
// @extends Actor,
#[derive(Debug, Clone)]
pub struct Rectangle {}

impl Object for Rectangle {}
impl Is<Rectangle> for Rectangle {}

impl AsRef<Rectangle> for Rectangle {
    fn as_ref(&self) -> &Rectangle {
        self
    }
}
/// Trait containing all `Rectangle` methods.
///
/// # Implementors
///
/// [`Rectangle`](struct.Rectangle.html)
pub trait RectangleExt: 'static {
    /// The color of the border of the rectangle.
    fn get_property_border_color(&self) -> Option<Color>;

    /// The color of the border of the rectangle.
    fn set_property_border_color(&self, border_color: Option<Color>);

    /// The width of the border of the rectangle, in pixels.
    fn get_property_border_width(&self) -> u32;

    /// The width of the border of the rectangle, in pixels.
    fn set_property_border_width(&self, border_width: u32);

    /// The color of the rectangle.
    fn get_property_color(&self) -> Option<Color>;

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

impl<O: Is<Rectangle>> RectangleExt for O {
    fn get_property_border_color(&self) -> Option<Color> {
        // unsafe {
        //     let mut value = Value::from_type(<Color as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-color\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-color` getter")
        // }
        unimplemented!()
    }

    fn set_property_border_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(Color::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-color\0".as_ptr() as *const _,
        //         Value::from(color.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_border_width(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `border-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_border_width(&self, border_width: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"border-width\0".as_ptr() as *const _,
        //         Value::from(&border_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_color(&self) -> Option<Color> {
        // unsafe {
        //     let mut value = Value::from_type(<Color as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"color\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `color` getter")
        // }
        unimplemented!()
    }

    fn set_property_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(Color::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"color\0".as_ptr() as *const _,
        //         Value::from(color.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_has_border(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"has-border\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `has-border` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_has_border(&self, has_border: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"has-border\0".as_ptr() as *const _,
        //         Value::from(&has_border).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_border_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_border_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_has_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle")
    }
}
