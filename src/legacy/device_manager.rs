use crate::{Backend, InputDevice, InputDeviceType};
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct DeviceManager(Object<ffi::ClutterDeviceManager, ffi::ClutterDeviceManagerClass, DeviceManagerClass>);

    match fn {
        get_type => || ffi::clutter_device_manager_get_type(),
    }
}

impl DeviceManager {
    /// Retrieves the device manager singleton
    ///
    /// # Returns
    ///
    /// the `DeviceManager` singleton.
    ///  The returned instance is owned by Clutter and it should not be
    ///  modified or freed
    pub fn get_default() -> Option<DeviceManager> {
        unsafe { from_glib_none(ffi::clutter_device_manager_get_default()) }
    }
}

/// Trait containing all `DeviceManager` methods.
///
/// # Implementors
///
/// [`DeviceManager`](struct.DeviceManager.html)
pub trait DeviceManagerExt: 'static {
    /// Retrieves the core `InputDevice` of type `device_type`
    ///
    /// Core devices are devices created automatically by the default
    /// backend
    /// ## `device_type`
    /// the type of the core device
    ///
    /// # Returns
    ///
    /// a `InputDevice` or `None`. The
    ///  returned device is owned by the `DeviceManager` and should
    ///  not be modified or freed
    fn get_core_device(&self, device_type: InputDeviceType) -> Option<InputDevice>;

    /// Retrieves the `InputDevice` with the given `device_id`
    /// ## `device_id`
    /// the integer id of a device
    ///
    /// # Returns
    ///
    /// a `InputDevice` or `None`. The
    ///  returned device is owned by the `DeviceManager` and should
    ///  never be modified or freed
    fn get_device(&self, device_id: i32) -> Option<InputDevice>;

    /// Lists all currently registered input devices
    ///
    /// # Returns
    ///
    ///
    ///  a newly allocated list of `InputDevice` objects. Use
    ///  `glib::SList::free` to deallocate it when done
    fn list_devices(&self) -> Vec<InputDevice>;

    /// Lists all currently registered input devices
    ///
    /// # Returns
    ///
    ///
    ///  a pointer to the internal list of `InputDevice` objects. The
    ///  returned list is owned by the `DeviceManager` and should never
    ///  be modified or freed
    fn peek_devices(&self) -> Vec<InputDevice>;

    fn get_property_backend(&self) -> Option<Backend>;

    /// The ::device-added signal is emitted each time a device has been
    /// added to the `DeviceManager`
    /// ## `device`
    /// the newly added `InputDevice`
    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::device-removed signal is emitted each time a device has been
    /// removed from the `DeviceManager`
    /// ## `device`
    /// the removed `InputDevice`
    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F)
        -> SignalHandlerId;
}

impl<O: IsA<DeviceManager>> DeviceManagerExt for O {
    fn get_core_device(&self, device_type: InputDeviceType) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_device_manager_get_core_device(
                self.as_ref().to_glib_none().0,
                device_type.to_glib(),
            ))
        }
    }

    fn get_device(&self, device_id: i32) -> Option<InputDevice> {
        unsafe {
            from_glib_none(ffi::clutter_device_manager_get_device(
                self.as_ref().to_glib_none().0,
                device_id,
            ))
        }
    }

    fn list_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::clutter_device_manager_list_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::clutter_device_manager_peek_devices(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_backend(&self) -> Option<Backend> {
        unsafe {
            let mut value = Value::from_type(<Backend as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<P, F: Fn(&P, &InputDevice) + 'static>(
            this: *mut ffi::ClutterDeviceManager,
            device: *mut ffi::ClutterInputDevice,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeviceManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<P, F: Fn(&P, &InputDevice) + 'static>(
            this: *mut ffi::ClutterDeviceManager,
            device: *mut ffi::ClutterInputDevice,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DeviceManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DeviceManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceManager")
    }
}
