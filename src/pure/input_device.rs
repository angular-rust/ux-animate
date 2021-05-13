use super::{
    Actor, Backend, DeviceManager, Event, EventSequence, HandlerId, InputAxis, InputDeviceType,
    InputMode, ModifierType, Stage,
};
use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InputDevice {}

impl InputDevice {
    /// Retrieves a pointer to the `InputDevice` that has been
    /// associated to `self`.
    ///
    /// If the `InputDevice:device-mode` property of `self` is
    /// set to `InputMode::Master`, this function will return
    /// `None`.
    ///
    /// # Returns
    ///
    /// a `InputDevice`, or `None`
    pub fn get_associated_device(&self) -> Option<InputDevice> {
        // unsafe {
        //     from_glib_none(ffi::clutter_input_device_get_associated_device(
        //         self.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    /// Retrieves the type of axis on `self` at the given index.
    /// ## `index_`
    /// the index of the axis
    ///
    /// # Returns
    ///
    /// the axis type
    pub fn get_axis(&self, index_: u32) -> InputAxis {
        // unsafe {
        //     from_glib(ffi::clutter_input_device_get_axis(
        //         self.to_glib_none().0,
        //         index_,
        //     ))
        // }
        unimplemented!()
    }

    //pub fn get_axis_value(&self, axes: &[f64], axis: InputAxis) -> Option<f64> {
    //    unsafe { TODO: call clutter_sys:clutter_input_device_get_axis_value() }
    //}

    // /// Retrieves the latest coordinates of a pointer or touch point of
    // /// `self`.
    // /// ## `sequence`
    // /// a `EventSequence`, or `None` if
    // ///  the device is not touch-based
    // /// ## `point`
    // /// return location for the pointer
    // ///  or touch point
    // ///
    // /// # Returns
    // ///
    // /// `false` if the device's sequence hasn't been found,
    // ///  and `true` otherwise.
    // pub fn get_coords(&self, sequence: Option<&mut EventSequence>) -> Option<Point> {
    //     unimplemented!()
    // }

    /// Retrieves the unique identifier of `self`
    ///
    /// # Returns
    ///
    /// the identifier of the device
    pub fn get_device_id(&self) -> i32 {
        unimplemented!()
    }

    /// Retrieves the `InputMode` of `self`.
    ///
    /// # Returns
    ///
    /// the device mode
    pub fn get_device_mode(&self) -> InputMode {
        unimplemented!()
    }

    /// Retrieves the name of the `self`
    ///
    /// # Returns
    ///
    /// the name of the device, or `None`. The returned string
    ///  is owned by the `InputDevice` and should never be modified
    ///  or freed
    pub fn get_device_name(&self) -> Option<String> {
        unimplemented!()
    }

    /// Retrieves the type of `self`
    ///
    /// # Returns
    ///
    /// the type of the device
    pub fn get_device_type(&self) -> InputDeviceType {
        unimplemented!()
    }

    /// Retrieves whether `self` is enabled.
    ///
    /// # Returns
    ///
    /// `true` if the device is enabled
    pub fn get_enabled(&self) -> bool {
        unimplemented!()
    }

    /// Retrieves a pointer to the `Actor` currently grabbing all
    /// the events coming from `self`.
    ///
    /// # Returns
    ///
    /// a `Actor`, or `None`
    pub fn get_grabbed_actor(&self) -> Option<Actor> {
        unimplemented!()
    }

    /// Retrieves whether `self` has a pointer that follows the
    /// device motion.
    ///
    /// # Returns
    ///
    /// `true` if the device has a cursor
    pub fn get_has_cursor(&self) -> bool {
        unimplemented!()
    }

    /// Retrieves the key set using `InputDevice::set_key`
    /// ## `index_`
    /// the index of the key
    /// ## `keyval`
    /// return location for the keyval at `index_`
    /// ## `modifiers`
    /// return location for the modifiers at `index_`
    ///
    /// # Returns
    ///
    /// `true` if a key was set at the given index
    pub fn get_key(&self, index_: u32) -> Option<(u32, ModifierType)> {
        unimplemented!()
    }

    /// Retrieves the current modifiers state of the device, as seen
    /// by the last event processed.
    ///
    /// # Returns
    ///
    /// the last known modifier state
    pub fn get_modifier_state(&self) -> ModifierType {
        unimplemented!()
    }

    /// Retrieves the number of axes available on `self`.
    ///
    /// # Returns
    ///
    /// the number of axes on the device
    pub fn get_n_axes(&self) -> u32 {
        unimplemented!()
    }

    /// Retrieves the number of keys registered for `self`.
    ///
    /// # Returns
    ///
    /// the number of registered keys
    pub fn get_n_keys(&self) -> u32 {
        unimplemented!()
    }

    /// Retrieves the `Actor` underneath the pointer of `self`
    ///
    /// # Returns
    ///
    /// a pointer to the `Actor` or `None`
    pub fn get_pointer_actor(&self) -> Option<Actor> {
        unimplemented!()
    }

    /// Retrieves the `Stage` underneath the pointer of `self`
    ///
    /// # Returns
    ///
    /// a pointer to the `Stage` or `None`
    pub fn get_pointer_stage(&self) -> Option<Stage> {
        unimplemented!()
    }

    /// Gets the product ID of this device.
    ///
    /// # Returns
    ///
    /// the product ID
    pub fn get_product_id(&self) -> Option<String> {
        unimplemented!()
    }

    /// Retrieves the slave devices attached to `self`.
    ///
    /// # Returns
    ///
    /// a
    ///  list of `InputDevice`, or `None`. The contents of the list are
    ///  owned by the device. Use `glib::List::free` when done
    pub fn get_slave_devices(&self) -> Vec<InputDevice> {
        unimplemented!()
    }

    /// Gets the vendor ID of this device.
    ///
    /// # Returns
    ///
    /// the vendor ID
    pub fn get_vendor_id(&self) -> Option<String> {
        unimplemented!()
    }

    /// Acquires a grab on `actor` for the given `self`.
    ///
    /// Any event coming from `self` will be delivered to `actor`, bypassing
    /// the usual event delivery mechanism, until the grab is released by
    /// calling `InputDevice::ungrab`.
    ///
    /// The grab is client-side: even if the windowing system used by the
    /// backend has the concept of "device grabs", it will not use them.
    ///
    /// Only `InputDevice` of types `InputDeviceType::PointerDevice` and
    /// `InputDeviceType::KeyboardDevice` can hold a grab.
    /// ## `actor`
    /// a `Actor`
    pub fn grab<P: Is<Actor>>(&self, actor: &P) {
        unimplemented!()
    }

    // /// Translates a hardware keycode from a `KeyEvent` to the
    // /// equivalent evdev keycode. Note that depending on the input backend
    // /// used by this function can fail if there is no obvious
    // /// mapping between the key codes. The hardware keycode can be taken
    // /// from the `KeyEvent.hardware_keycode` member of `KeyEvent`.
    // /// ## `hardware_keycode`
    // /// The hardware keycode from a `KeyEvent`
    // /// ## `evdev_keycode`
    // /// The return location for the evdev keycode
    // ///
    // /// # Returns
    // ///
    // /// `true` if the conversion succeeded, `false` otherwise.
    // pub fn keycode_to_evdev(&self, hardware_keycode: u32, evdev_keycode: u32) -> bool {
    //     unsafe {
    //         from_glib(ffi::clutter_input_device_keycode_to_evdev(
    //             self.to_glib_none().0,
    //             hardware_keycode,
    //             evdev_keycode,
    //         ))
    //     }
    // }

    /// Retrieves a pointer to the `Actor` currently grabbing the
    /// touch events coming from `self` given the `sequence`.
    /// ## `sequence`
    /// a `EventSequence`
    ///
    /// # Returns
    ///
    /// a `Actor`, or `None`
    pub fn sequence_get_grabbed_actor(&self, sequence: &mut EventSequence) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_input_device_sequence_get_grabbed_actor(
        //         self.to_glib_none().0,
        //         sequence.to_glib_none_mut().0,
        //     ))
        // }
        unimplemented!()
    }

    /// Acquires a grab on `actor` for the given `self` and the given touch
    /// `sequence`.
    ///
    /// Any touch event coming from `self` and from `sequence` will be
    /// delivered to `actor`, bypassing the usual event delivery mechanism,
    /// until the grab is released by calling
    /// `InputDevice::sequence_ungrab`.
    ///
    /// The grab is client-side: even if the windowing system used by the
    /// backend has the concept of "device grabs", it will not use them.
    /// ## `sequence`
    /// a `EventSequence`
    /// ## `actor`
    /// a `Actor`
    pub fn sequence_grab<P: Is<Actor>>(&self, sequence: &mut EventSequence, actor: &P) {
        // unsafe {
        //     ffi::clutter_input_device_sequence_grab(
        //         self.to_glib_none().0,
        //         sequence.to_glib_none_mut().0,
        //         actor.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// Releases the grab on the `self` for the given `sequence`, if one is
    /// in place.
    /// ## `sequence`
    /// a `EventSequence`
    pub fn sequence_ungrab(&self, sequence: &mut EventSequence) {
        // unsafe {
        //     ffi::clutter_input_device_sequence_ungrab(
        //         self.to_glib_none().0,
        //         sequence.to_glib_none_mut().0,
        //     );
        // }
        unimplemented!()
    }

    /// Enables or disables a `InputDevice`.
    ///
    /// Only devices with a `InputDevice:device-mode` property set
    /// to `InputMode::Slave` or `InputMode::Floating` can
    /// be disabled.
    /// ## `enabled`
    /// `true` to enable the `self`
    pub fn set_enabled(&self, enabled: bool) {
        // unsafe {
        //     ffi::clutter_input_device_set_enabled(self.to_glib_none().0, enabled.to_glib());
        // }
        unimplemented!()
    }

    /// Sets the keyval and modifiers at the given `index_` for `self`.
    ///
    /// will use the keyval and modifiers set when filling out
    /// an event coming from the same input device.
    /// ## `index_`
    /// the index of the key
    /// ## `keyval`
    /// the keyval
    /// ## `modifiers`
    /// a bitmask of modifiers
    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ModifierType) {
        // unsafe {
        //     ffi::clutter_input_device_set_key(
        //         self.to_glib_none().0,
        //         index_,
        //         keyval,
        //         modifiers.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// Releases the grab on the `self`, if one is in place.
    pub fn ungrab(&self) {
        // unsafe {
        //     ffi::clutter_input_device_ungrab(self.to_glib_none().0);
        // }
        unimplemented!()
    }

    /// Forcibly updates the state of the `self` using a `Event`
    ///
    /// This function should never be used by applications: it is meant
    /// for integration with embedding toolkits, like clutter-gtk
    ///
    /// Embedding toolkits that disable the event collection inside
    /// need to use this function to update the state of input devices depending
    /// on a `Event` that they are going to submit to the event handling code
    /// in it though `do_event`. Since the input devices hold the state
    /// that is going to be used to fill in fields like the `ButtonEvent`
    /// click count, or to emit synthesized events like `EventType::Enter` and
    /// `EventType::Leave`, it is necessary for embedding toolkits to also be
    /// responsible of updating the input device state.
    ///
    /// For instance, this might be the code to translate an embedding toolkit
    /// native motion notification into a `MotionEvent` and ask
    /// to process it:
    ///
    ///
    /// ```text
    ///   Event c_event;
    ///
    ///   translate_native_event_to_clutter (native_event, &c_event);
    ///
    ///   do_event (&c_event);
    /// ```
    ///
    /// Before letting `do_event` process the event, it is necessary to call
    /// `InputDevice::update_from_event`:
    ///
    ///
    /// ```text
    ///   Event c_event;
    ///   DeviceManager *manager;
    ///   InputDevice *device;
    ///
    ///   translate_native_event_to_clutter (native_event, &c_event);
    ///
    ///   // get the device manager
    ///   manager = device_manager_get_default ();
    ///
    ///   // use the default Core Pointer that backends register by default
    ///   device = device_manager_get_core_device (manager, %POINTER_DEVICE);
    ///
    ///   // update the state of the input device
    ///   input_device_update_from_event (device, &c_event, FALSE);
    ///
    ///   do_event (&c_event);
    /// ```
    ///
    /// The `update_stage` boolean argument should be used when the input device
    /// enters and leaves a `Stage`; it will use the `Stage` field
    /// of the passed `event` to update the stage associated to the input device.
    /// ## `event`
    /// a `Event`
    /// ## `update_stage`
    /// whether to update the `Stage` of the `self`
    ///  using the stage of the event
    pub fn update_from_event(&self, event: &mut Event, update_stage: bool) {
        // unsafe {
        //     ffi::clutter_input_device_update_from_event(
        //         self.to_glib_none().0,
        //         event.to_glib_none_mut().0,
        //         update_stage.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    /// The `Backend` that created the device.
    pub fn get_property_backend(&self) -> Option<Backend> {
        unimplemented!()
    }

    /// The `DeviceManager` instance which owns the device
    pub fn get_property_device_manager(&self) -> Option<DeviceManager> {
        unimplemented!()
    }

    /// The unique identifier of the device
    pub fn get_property_id(&self) -> i32 {
        unimplemented!()
    }

    /// The name of the device
    pub fn get_property_name(&self) -> Option<String> {
        unimplemented!()
    }

    pub fn connect_property_enabled_notify<F: Fn(&InputDevice) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_n_axes_notify<F: Fn(&InputDevice) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputDevice")
    }
}
