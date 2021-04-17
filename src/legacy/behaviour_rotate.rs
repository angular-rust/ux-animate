// Scriptable
use crate::{Alpha, Behaviour, RotateAxis, RotateDirection};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

// TODO: , @implements Scriptable
glib_wrapper! {
    pub struct BehaviourRotate(Object<ffi::ClutterBehaviourRotate, ffi::ClutterBehaviourRotateClass, BehaviourRotateClass>) @extends Behaviour;

    match fn {
        get_type => || ffi::clutter_behaviour_rotate_get_type(),
    }
}

impl BehaviourRotate {
    pub fn new<P: IsA<Alpha>>(
        alpha: Option<&P>,
        axis: RotateAxis,
        direction: RotateDirection,
        angle_start: f64,
        angle_end: f64,
    ) -> BehaviourRotate {
        unsafe {
            Behaviour::from_glib_full(ffi::clutter_behaviour_rotate_new(
                alpha.map(|p| p.as_ref()).to_glib_none().0,
                axis.to_glib(),
                direction.to_glib(),
                angle_start,
                angle_end,
            ))
            .unsafe_cast()
        }
    }
}

pub trait BehaviourRotateExt: 'static {
    fn get_axis(&self) -> RotateAxis;

    fn get_bounds(&self) -> (f64, f64);

    fn get_center(&self) -> (i32, i32, i32);

    fn get_direction(&self) -> RotateDirection;

    fn set_axis(&self, axis: RotateAxis);

    fn set_bounds(&self, angle_start: f64, angle_end: f64);

    fn set_center(&self, x: i32, y: i32, z: i32);

    fn set_direction(&self, direction: RotateDirection);

    fn get_property_angle_end(&self) -> f64;

    fn set_property_angle_end(&self, angle_end: f64);

    fn get_property_angle_start(&self) -> f64;

    fn set_property_angle_start(&self, angle_start: f64);

    fn get_property_center_x(&self) -> i32;

    fn set_property_center_x(&self, center_x: i32);

    fn get_property_center_y(&self) -> i32;

    fn set_property_center_y(&self, center_y: i32);

    fn get_property_center_z(&self) -> i32;

    fn set_property_center_z(&self, center_z: i32);

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_center_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_center_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_center_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BehaviourRotate>> BehaviourRotateExt for O {
    fn get_axis(&self) -> RotateAxis {
        unsafe {
            from_glib(ffi::clutter_behaviour_rotate_get_axis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_bounds(&self) -> (f64, f64) {
        unsafe {
            let mut angle_start = mem::MaybeUninit::uninit();
            let mut angle_end = mem::MaybeUninit::uninit();
            ffi::clutter_behaviour_rotate_get_bounds(
                self.as_ref().to_glib_none().0,
                angle_start.as_mut_ptr(),
                angle_end.as_mut_ptr(),
            );
            let angle_start = angle_start.assume_init();
            let angle_end = angle_end.assume_init();
            (angle_start, angle_end)
        }
    }

    fn get_center(&self) -> (i32, i32, i32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let mut z = mem::MaybeUninit::uninit();
            ffi::clutter_behaviour_rotate_get_center(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
                z.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            let z = z.assume_init();
            (x, y, z)
        }
    }

    fn get_direction(&self) -> RotateDirection {
        unsafe {
            from_glib(ffi::clutter_behaviour_rotate_get_direction(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_axis(&self, axis: RotateAxis) {
        unsafe {
            ffi::clutter_behaviour_rotate_set_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        }
    }

    fn set_bounds(&self, angle_start: f64, angle_end: f64) {
        unsafe {
            ffi::clutter_behaviour_rotate_set_bounds(
                self.as_ref().to_glib_none().0,
                angle_start,
                angle_end,
            );
        }
    }

    fn set_center(&self, x: i32, y: i32, z: i32) {
        unsafe {
            ffi::clutter_behaviour_rotate_set_center(self.as_ref().to_glib_none().0, x, y, z);
        }
    }

    fn set_direction(&self, direction: RotateDirection) {
        unsafe {
            ffi::clutter_behaviour_rotate_set_direction(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
            );
        }
    }

    fn get_property_angle_end(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-end\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `angle-end` getter")
                .unwrap()
        }
    }

    fn set_property_angle_end(&self, angle_end: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-end\0".as_ptr() as *const _,
                Value::from(&angle_end).to_glib_none().0,
            );
        }
    }

    fn get_property_angle_start(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-start\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `angle-start` getter")
                .unwrap()
        }
    }

    fn set_property_angle_start(&self, angle_start: f64) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"angle-start\0".as_ptr() as *const _,
                Value::from(&angle_start).to_glib_none().0,
            );
        }
    }

    fn get_property_center_x(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-x\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `center-x` getter")
                .unwrap()
        }
    }

    fn set_property_center_x(&self, center_x: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-x\0".as_ptr() as *const _,
                Value::from(&center_x).to_glib_none().0,
            );
        }
    }

    fn get_property_center_y(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-y\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `center-y` getter")
                .unwrap()
        }
    }

    fn set_property_center_y(&self, center_y: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-y\0".as_ptr() as *const _,
                Value::from(&center_y).to_glib_none().0,
            );
        }
    }

    fn get_property_center_z(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-z\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `center-z` getter")
                .unwrap()
        }
    }

    fn set_property_center_z(&self, center_z: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"center-z\0".as_ptr() as *const _,
                Value::from(&center_z).to_glib_none().0,
            );
        }
    }

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_angle_end_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
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
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_axis_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::axis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_axis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_center_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_x_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::center-x\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_center_x_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_center_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_y_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::center-y\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_center_y_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_center_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_z_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::center-z\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_center_z_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBehaviourRotate,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BehaviourRotate>,
        {
            let f: &F = &*(f as *const F);
            f(&BehaviourRotate::from_glib_borrow(this).unsafe_cast_ref())
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
}

impl fmt::Display for BehaviourRotate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BehaviourRotate")
    }
}
