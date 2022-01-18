use super::{Backend, HandlerId, InputDevice, InputDeviceType};
use crate::prelude::*;
use std::fmt;

// @short_description: Maintains the list of input devices
//
// #DeviceManager is a singleton object, owned by , which
// maintains the list of #InputDevice<!-- -->s.
//
// Depending on the backend used by  it is possible to use the
// #DeviceManager::device-added and
// #DeviceManager::device-removed to monitor addition and removal
// of devices.
pub struct DeviceManager {
    // back-pointer to the backend
    backend: Option<Backend>,
}

impl DeviceManager {
    /// Retrieves the device manager singleton
    ///
    /// # Returns
    ///
    /// the `DeviceManager` singleton.
    ///  The returned instance is owned by  and it should not be
    ///  modified or freed
    pub fn get_default() -> Option<DeviceManager> {
        unimplemented!()
    }
}

impl Object for DeviceManager {}
impl Is<DeviceManager> for DeviceManager {}

impl AsRef<DeviceManager> for DeviceManager {
    fn as_ref(&self) -> &DeviceManager {
        self
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
    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> HandlerId;

    /// The ::device-removed signal is emitted each time a device has been
    /// removed from the `DeviceManager`
    /// ## `device`
    /// the removed `InputDevice`
    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<DeviceManager>> DeviceManagerExt for O {
    fn get_core_device(&self, device_type: InputDeviceType) -> Option<InputDevice> {
        unimplemented!()
    }

    fn get_device(&self, device_id: i32) -> Option<InputDevice> {
        unimplemented!()
    }

    fn list_devices(&self) -> Vec<InputDevice> {
        unimplemented!()
    }

    fn peek_devices(&self) -> Vec<InputDevice> {
        unimplemented!()
    }

    fn get_property_backend(&self) -> Option<Backend> {
        unimplemented!()
    }

    fn connect_device_added<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_device_removed<F: Fn(&Self, &InputDevice) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DeviceManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceManager")
    }
}
