use crate::{ActorMeta, Effect, OffscreenEffect};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct DeformEffect(Object<ffi::ClutterDeformEffect, ffi::ClutterDeformEffectClass, DeformEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_deform_effect_get_type(),
    }
}

/// Trait containing all `DeformEffect` methods.
///
/// # Implementors
///
/// [`DeformEffect`](struct.DeformEffect.html), [`PageTurnEffect`](struct.PageTurnEffect.html)
pub trait DeformEffectExt: 'static {
    //fn get_back_material(&self) -> /*Unimplemented*/Option<dx::Handle>;

    /// Retrieves the number of horizontal and vertical tiles used to sub-divide
    /// the actor's geometry during the effect
    /// ## `x_tiles`
    /// return location for the number of horizontal tiles,
    ///  or `None`
    /// ## `y_tiles`
    /// return location for the number of vertical tiles,
    ///  or `None`
    fn get_n_tiles(&self) -> (u32, u32);

    /// Invalidates the `self`<!-- -->'s vertices and, if it is associated
    /// to an actor, it will queue a redraw
    fn invalidate(&self);

    //fn set_back_material(&self, material: /*Unimplemented*/Option<dx::Handle>);

    /// Sets the number of horizontal and vertical tiles to be used
    /// when applying the effect
    ///
    /// More tiles allow a finer grained deformation at the expenses
    /// of computation
    /// ## `x_tiles`
    /// number of horizontal tiles
    /// ## `y_tiles`
    /// number of vertical tiles
    fn set_n_tiles(&self, x_tiles: u32, y_tiles: u32);

    /// The number of horizontal tiles. The bigger the number, the
    /// smaller the tiles
    fn get_property_x_tiles(&self) -> u32;

    /// The number of horizontal tiles. The bigger the number, the
    /// smaller the tiles
    fn set_property_x_tiles(&self, x_tiles: u32);

    /// The number of vertical tiles. The bigger the number, the
    /// smaller the tiles
    fn get_property_y_tiles(&self) -> u32;

    /// The number of vertical tiles. The bigger the number, the
    /// smaller the tiles
    fn set_property_y_tiles(&self, y_tiles: u32);

    fn connect_property_x_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeformEffect>> DeformEffectExt for O {
    //fn get_back_material(&self) -> /*Unimplemented*/Option<dx::Handle> {
    //    unsafe { TODO: call clutter_sys:clutter_deform_effect_get_back_material() }
    //}

    fn get_n_tiles(&self) -> (u32, u32) {
        unsafe {
            let mut x_tiles = mem::MaybeUninit::uninit();
            let mut y_tiles = mem::MaybeUninit::uninit();
            ffi::clutter_deform_effect_get_n_tiles(
                self.as_ref().to_glib_none().0,
                x_tiles.as_mut_ptr(),
                y_tiles.as_mut_ptr(),
            );
            let x_tiles = x_tiles.assume_init();
            let y_tiles = y_tiles.assume_init();
            (x_tiles, y_tiles)
        }
    }

    fn invalidate(&self) {
        unsafe {
            ffi::clutter_deform_effect_invalidate(self.as_ref().to_glib_none().0);
        }
    }

    //fn set_back_material(&self, material: /*Unimplemented*/Option<dx::Handle>) {
    //    unsafe { TODO: call clutter_sys:clutter_deform_effect_set_back_material() }
    //}

    fn set_n_tiles(&self, x_tiles: u32, y_tiles: u32) {
        unsafe {
            ffi::clutter_deform_effect_set_n_tiles(
                self.as_ref().to_glib_none().0,
                x_tiles,
                y_tiles,
            );
        }
    }

    fn get_property_x_tiles(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"x-tiles\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `x-tiles` getter")
                .unwrap()
        }
    }

    fn set_property_x_tiles(&self, x_tiles: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"x-tiles\0".as_ptr() as *const _,
                Value::from(&x_tiles).to_glib_none().0,
            );
        }
    }

    fn get_property_y_tiles(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"y-tiles\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `y-tiles` getter")
                .unwrap()
        }
    }

    fn set_property_y_tiles(&self, y_tiles: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"y-tiles\0".as_ptr() as *const _,
                Value::from(&y_tiles).to_glib_none().0,
            );
        }
    }

    fn connect_property_x_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_x_tiles_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDeformEffect,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeformEffect>,
        {
            let f: &F = &*(f as *const F);
            f(&DeformEffect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::x-tiles\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_x_tiles_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_y_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_y_tiles_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDeformEffect,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeformEffect>,
        {
            let f: &F = &*(f as *const F);
            f(&DeformEffect::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::y-tiles\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_y_tiles_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeformEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeformEffect")
    }
}
