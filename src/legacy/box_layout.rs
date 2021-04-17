use crate::{LayoutManager, Orientation};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct BoxLayout(Object<ffi::ClutterBoxLayout, ffi::ClutterBoxLayoutClass, BoxLayoutClass>) @extends LayoutManager, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_box_layout_get_type(),
    }
}

impl BoxLayout {
    /// Creates a new `BoxLayout` layout manager
    ///
    /// # Returns
    ///
    /// the newly created `BoxLayout`
    pub fn new() -> BoxLayout {
        unsafe { LayoutManager::from_glib_none(ffi::clutter_box_layout_new()).unsafe_cast() }
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `BoxLayout` methods.
///
/// # Implementors
///
/// [`BoxLayout`](struct.BoxLayout.html)
pub trait BoxLayoutExt: 'static {
    /// Retrieves if the children sizes are allocated homogeneously.
    ///
    /// # Returns
    ///
    /// `true` if the `BoxLayout` is arranging its children
    ///  homogeneously, and `false` otherwise
    fn get_homogeneous(&self) -> bool;

    /// Retrieves the orientation of the `self`.
    ///
    /// # Returns
    ///
    /// the orientation of the layout
    fn get_orientation(&self) -> Orientation;

    /// Retrieves the value set using `BoxLayoutExt::set_pack_start`
    ///
    /// # Returns
    ///
    /// `true` if the `BoxLayout` should pack children
    ///  at the beginning of the layout, and `false` otherwise
    fn get_pack_start(&self) -> bool;

    /// Retrieves the spacing set using `BoxLayoutExt::set_spacing`
    ///
    /// # Returns
    ///
    /// the spacing between children of the `BoxLayout`
    fn get_spacing(&self) -> u32;

    /// Sets whether the size of `self` children should be
    /// homogeneous
    /// ## `homogeneous`
    /// `true` if the layout should be homogeneous
    fn set_homogeneous(&self, homogeneous: bool);

    /// Sets the orientation of the `BoxLayout` layout manager.
    /// ## `orientation`
    /// the orientation of the `BoxLayout`
    fn set_orientation(&self, orientation: Orientation);

    /// Sets whether children of `self` should be layed out by appending
    /// them or by prepending them
    /// ## `pack_start`
    /// `true` if the `self` should pack children at the
    ///  beginning of the layout
    fn set_pack_start(&self, pack_start: bool);

    /// Sets the spacing between children of `self`
    /// ## `spacing`
    /// the spacing between children of the layout, in pixels
    fn set_spacing(&self, spacing: u32);

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pack_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<BoxLayout>> BoxLayoutExt for O {
    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_box_layout_get_homogeneous(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::clutter_box_layout_get_orientation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pack_start(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_box_layout_get_pack_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_spacing(&self) -> u32 {
        unsafe { ffi::clutter_box_layout_get_spacing(self.as_ref().to_glib_none().0) }
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::clutter_box_layout_set_homogeneous(
                self.as_ref().to_glib_none().0,
                homogeneous.to_glib(),
            );
        }
    }

    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::clutter_box_layout_set_orientation(
                self.as_ref().to_glib_none().0,
                orientation.to_glib(),
            );
        }
    }

    fn set_pack_start(&self, pack_start: bool) {
        unsafe {
            ffi::clutter_box_layout_set_pack_start(
                self.as_ref().to_glib_none().0,
                pack_start.to_glib(),
            );
        }
    }

    fn set_spacing(&self, spacing: u32) {
        unsafe {
            ffi::clutter_box_layout_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_homogeneous_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBoxLayout,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoxLayout>,
        {
            let f: &F = &*(f as *const F);
            f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::homogeneous\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_homogeneous_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_orientation_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBoxLayout,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoxLayout>,
        {
            let f: &F = &*(f as *const F);
            f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::orientation\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_orientation_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pack_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pack_start_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBoxLayout,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoxLayout>,
        {
            let f: &F = &*(f as *const F);
            f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pack-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pack_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterBoxLayout,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<BoxLayout>,
        {
            let f: &F = &*(f as *const F);
            f(&BoxLayout::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxLayout")
    }
}
