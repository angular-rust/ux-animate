// Scriptable
use crate::{Alpha, Behaviour, RotateAxis, RotateDirection};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::mem::transmute;
use std::{fmt, mem};

// TODO: @implements Scriptable
glib_wrapper! {
    pub struct BehaviourEllipse(Object<ffi::ClutterBehaviourEllipse, ffi::ClutterBehaviourEllipseClass, BehaviourEllipseClass>) @extends Behaviour;

    match fn {
        get_type => || ffi::clutter_behaviour_ellipse_get_type(),
    }
}

impl BehaviourEllipse {
    pub fn new<P: IsA<Alpha>>(
        alpha: Option<&P>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        direction: RotateDirection,
        start: f64,
        end: f64,
    ) -> BehaviourEllipse {
        unsafe {
            Behaviour::from_glib_full(ffi::clutter_behaviour_ellipse_new(
                alpha.map(|p| p.as_ref()).to_glib_none().0,
                x,
                y,
                width,
                height,
                direction.to_glib(),
                start,
                end,
            ))
            .unsafe_cast()
        }
    }
}

pub trait BehaviourEllipseExt: 'static {
    fn get_angle_end(&self) -> f64;

    fn get_angle_start(&self) -> f64;

    fn get_angle_tilt(&self, axis: RotateAxis) -> f64;

    fn get_center(&self) -> (i32, i32);

    fn get_direction(&self) -> RotateDirection;

    fn get_height(&self) -> i32;

    fn get_tilt(&self) -> (f64, f64, f64);

    fn get_width(&self) -> i32;

    fn set_angle_end(&self, angle_end: f64);

    fn set_angle_start(&self, angle_start: f64);

    fn set_angle_tilt(&self, axis: RotateAxis, angle_tilt: f64);

    fn set_center(&self, x: i32, y: i32);

    fn set_direction(&self, direction: RotateDirection);

    fn set_height(&self, height: i32);

    fn set_tilt(&self, angle_tilt_x: f64, angle_tilt_y: f64, angle_tilt_z: f64);

    fn set_width(&self, width: i32);

    fn get_property_angle_tilt_x(&self) -> f64;

    fn set_property_angle_tilt_x(&self, angle_tilt_x: f64);

    fn get_property_angle_tilt_y(&self) -> f64;

    fn set_property_angle_tilt_y(&self, angle_tilt_y: f64);

    fn get_property_angle_tilt_z(&self) -> f64;

    fn set_property_angle_tilt_z(&self, angle_tilt_z: f64);

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_angle_tilt_x_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_angle_tilt_y_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_angle_tilt_z_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_center_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BehaviourEllipse>> BehaviourEllipseExt for O {
    fn get_angle_end(&self) -> f64 {
        unsafe { ffi::clutter_behaviour_ellipse_get_angle_end(self.as_ref().to_glib_none().0) }
    }

    fn get_angle_start(&self) -> f64 {
        unsafe { ffi::clutter_behaviour_ellipse_get_angle_start(self.as_ref().to_glib_none().0) }
    }

    fn get_angle_tilt(&self, axis: RotateAxis) -> f64 {
        unsafe {
            ffi::clutter_behaviour_ellipse_get_angle_tilt(
                self.as_ref().to_glib_none().0,
                axis.to_glib(),
            )
        }
    }

    fn get_center(&self) -> (i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::clutter_behaviour_ellipse_get_center(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    fn get_direction(&self) -> RotateDirection {
        unsafe {
            from_glib(ffi::clutter_behaviour_ellipse_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_height(&self) -> i32 {
        unsafe { ffi::clutter_behaviour_ellipse_get_height(self.as_ref().to_glib_none().0) }
    }

    fn get_tilt(&self) -> (f64, f64, f64) {
        unsafe {
            let mut angle_tilt_x = mem::MaybeUninit::uninit();
            let mut angle_tilt_y = mem::MaybeUninit::uninit();
            let mut angle_tilt_z = mem::MaybeUninit::uninit();
            ffi::clutter_behaviour_ellipse_get_tilt(
                self.as_ref().to_glib_none().0,
                angle_tilt_x.as_mut_ptr(),
                angle_tilt_y.as_mut_ptr(),
                angle_tilt_z.as_mut_ptr(),
            );
            let angle_tilt_x = angle_tilt_x.assume_init();
            let angle_tilt_y = angle_tilt_y.assume_init();
            let angle_tilt_z = angle_tilt_z.assume_init();
            (angle_tilt_x, angle_tilt_y, angle_tilt_z)
        }
    }

    fn get_width(&self) -> i32 {
        unsafe { ffi::clutter_behaviour_ellipse_get_width(self.as_ref().to_glib_none().0) }
    }

    fn set_angle_end(&self, angle_end: f64) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_angle_end(self.as_ref().to_glib_none().0, angle_end);
        }
    }

    fn set_angle_start(&self, angle_start: f64) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_angle_start(
                self.as_ref().to_glib_none().0,
                angle_start,
            );
        }
    }

    fn set_angle_tilt(&self, axis: RotateAxis, angle_tilt: f64) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_angle_tilt(
                self.as_ref().to_glib_none().0,
                axis.to_glib(),
                angle_tilt,
            );
        }
    }

    fn set_center(&self, x: i32, y: i32) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_center(self.as_ref().to_glib_none().0, x, y);
        }
    }

    fn set_direction(&self, direction: RotateDirection) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_direction(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
            );
        }
    }

    fn set_height(&self, height: i32) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_tilt(&self, angle_tilt_x: f64, angle_tilt_y: f64, angle_tilt_z: f64) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_tilt(
                self.as_ref().to_glib_none().0,
                angle_tilt_x,
                angle_tilt_y,
                angle_tilt_z,
            );
        }
    }

    fn set_width(&self, width: i32) {
        unsafe {
            ffi::clutter_behaviour_ellipse_set_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn get_property_angle_tilt_x(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-x\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `angle-tilt-x` getter")
                .unwrap()
        }
    }

    fn set_property_angle_tilt_x(&self, angle_tilt_x: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-x\0".as_ptr() as *const _,
                Value::from(&angle_tilt_x).to_glib_none().0,
            );
        }
    }

    fn get_property_angle_tilt_y(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-y\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `angle-tilt-y` getter")
                .unwrap()
        }
    }

    fn set_property_angle_tilt_y(&self, angle_tilt_y: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-y\0".as_ptr() as *const _,
                Value::from(&angle_tilt_y).to_glib_none().0,
            );
        }
    }

    fn get_property_angle_tilt_z(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-z\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `angle-tilt-z` getter")
                .unwrap()
        }
    }

    fn set_property_angle_tilt_z(&self, angle_tilt_z: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-tilt-z\0".as_ptr() as *const _,
                Value::from(&angle_tilt_z).to_glib_none().0,
            );
        }
    }

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_end_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::angle-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_start_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::angle-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_angle_tilt_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_tilt_x_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::angle-tilt-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_tilt_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_angle_tilt_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_tilt_y_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::angle-tilt-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_tilt_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_angle_tilt_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_tilt_z_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::angle-tilt-z\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_angle_tilt_z_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_center_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::center\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_center_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourEllipse,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourEllipse>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourEllipse::from_glib_borrow(this).unsafe_cast_ref())
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

impl fmt::Display for BehaviourEllipse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BehaviourEllipse")
    }
}
