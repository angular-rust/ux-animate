use crate::{
    Actor, Backend, DeviceManager, Event, EventSequence, InputAxis, InputDeviceType, InputMode,
    ModifierType, Stage,
};
use glib::{
    object::{IsA, ObjectType as ObjectType_},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString, StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};
// InternalPoint

glib_wrapper! {
    pub struct InputDevice(Object<ffi::ClutterInputDevice, ffi::ClutterInputDeviceClass, InputDeviceClass>);

    match fn {
        get_type => || ffi::clutter_input_device_get_type(),
    }
}

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
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_associated_device(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the type of axis on `self` at the given index.
    /// ## `index_`
    /// the index of the axis
    ///
    /// # Returns
    ///
    /// the axis type
    pub fn get_axis(&self, index_: u32) -> InputAxis {
        unsafe {
            from_glib(ffi::clutter_input_device_get_axis(
                self.to_glib_none().0,
                index_,
            ))
        }
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
    //     unsafe {
    //         let mut point = Point::uninitialized();
    //         let ret = from_glib(ffi::clutter_input_device_get_coords(
    //             self.to_glib_none().0,
    //             sequence.to_glib_none_mut().0,
    //             point.to_glib_none_mut().0,
    //         ));
    //         if ret {
    //             Some(point)
    //         } else {
    //             None
    //         }
    //     }
    // }

    /// Retrieves the unique identifier of `self`
    ///
    /// # Returns
    ///
    /// the identifier of the device
    pub fn get_device_id(&self) -> i32 {
        unsafe { ffi::clutter_input_device_get_device_id(self.to_glib_none().0) }
    }

    /// Retrieves the `InputMode` of `self`.
    ///
    /// # Returns
    ///
    /// the device mode
    pub fn get_device_mode(&self) -> InputMode {
        unsafe {
            from_glib(ffi::clutter_input_device_get_device_mode(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the name of the `self`
    ///
    /// # Returns
    ///
    /// the name of the device, or `None`. The returned string
    ///  is owned by the `InputDevice` and should never be modified
    ///  or freed
    pub fn get_device_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_device_name(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the type of `self`
    ///
    /// # Returns
    ///
    /// the type of the device
    pub fn get_device_type(&self) -> InputDeviceType {
        unsafe {
            from_glib(ffi::clutter_input_device_get_device_type(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves whether `self` is enabled.
    ///
    /// # Returns
    ///
    /// `true` if the device is enabled
    pub fn get_enabled(&self) -> bool {
        unsafe { from_glib(ffi::clutter_input_device_get_enabled(self.to_glib_none().0)) }
    }

    /// Retrieves a pointer to the `Actor` currently grabbing all
    /// the events coming from `self`.
    ///
    /// # Returns
    ///
    /// a `Actor`, or `None`
    pub fn get_grabbed_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_grabbed_actor(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves whether `self` has a pointer that follows the
    /// device motion.
    ///
    /// # Returns
    ///
    /// `true` if the device has a cursor
    pub fn get_has_cursor(&self) -> bool {
        unsafe {
            from_glib(ffi::clutter_input_device_get_has_cursor(
                self.to_glib_none().0,
            ))
        }
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
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::clutter_input_device_get_key(
                self.to_glib_none().0,
                index_,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            let keyval = keyval.assume_init();
            let modifiers = modifiers.assume_init();
            if ret {
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    /// Retrieves the current modifiers state of the device, as seen
    /// by the last event processed.
    ///
    /// # Returns
    ///
    /// the last known modifier state
    pub fn get_modifier_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::clutter_input_device_get_modifier_state(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the number of axes available on `self`.
    ///
    /// # Returns
    ///
    /// the number of axes on the device
    pub fn get_n_axes(&self) -> u32 {
        unsafe { ffi::clutter_input_device_get_n_axes(self.to_glib_none().0) }
    }

    /// Retrieves the number of keys registered for `self`.
    ///
    /// # Returns
    ///
    /// the number of registered keys
    pub fn get_n_keys(&self) -> u32 {
        unsafe { ffi::clutter_input_device_get_n_keys(self.to_glib_none().0) }
    }

    /// Retrieves the `Actor` underneath the pointer of `self`
    ///
    /// # Returns
    ///
    /// a pointer to the `Actor` or `None`
    pub fn get_pointer_actor(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_pointer_actor(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the `Stage` underneath the pointer of `self`
    ///
    /// # Returns
    ///
    /// a pointer to the `Stage` or `None`
    pub fn get_pointer_stage(&self) -> Option<Stage> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_pointer_stage(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the product ID of this device.
    ///
    /// # Returns
    ///
    /// the product ID
    pub fn get_product_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_product_id(
                self.to_glib_none().0,
            ))
        }
    }

    /// Retrieves the slave devices attached to `self`.
    ///
    /// # Returns
    ///
    /// a
    ///  list of `InputDevice`, or `None`. The contents of the list are
    ///  owned by the device. Use `glib::List::free` when done
    pub fn get_slave_devices(&self) -> Vec<InputDevice> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::clutter_input_device_get_slave_devices(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the vendor ID of this device.
    ///
    /// # Returns
    ///
    /// the vendor ID
    pub fn get_vendor_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_input_device_get_vendor_id(
                self.to_glib_none().0,
            ))
        }
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
    pub fn grab<P: IsA<Actor>>(&self, actor: &P) {
        unsafe {
            ffi::clutter_input_device_grab(self.to_glib_none().0, actor.as_ref().to_glib_none().0);
        }
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
        unsafe {
            from_glib_none(ffi::clutter_input_device_sequence_get_grabbed_actor(
                self.to_glib_none().0,
                sequence.to_glib_none_mut().0,
            ))
        }
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
    pub fn sequence_grab<P: IsA<Actor>>(&self, sequence: &mut EventSequence, actor: &P) {
        unsafe {
            ffi::clutter_input_device_sequence_grab(
                self.to_glib_none().0,
                sequence.to_glib_none_mut().0,
                actor.as_ref().to_glib_none().0,
            );
        }
    }

    /// Releases the grab on the `self` for the given `sequence`, if one is
    /// in place.
    /// ## `sequence`
    /// a `EventSequence`
    pub fn sequence_ungrab(&self, sequence: &mut EventSequence) {
        unsafe {
            ffi::clutter_input_device_sequence_ungrab(
                self.to_glib_none().0,
                sequence.to_glib_none_mut().0,
            );
        }
    }

    /// Enables or disables a `InputDevice`.
    ///
    /// Only devices with a `InputDevice:device-mode` property set
    /// to `InputMode::Slave` or `InputMode::Floating` can
    /// be disabled.
    /// ## `enabled`
    /// `true` to enable the `self`
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::clutter_input_device_set_enabled(self.to_glib_none().0, enabled.to_glib());
        }
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
        unsafe {
            ffi::clutter_input_device_set_key(
                self.to_glib_none().0,
                index_,
                keyval,
                modifiers.to_glib(),
            );
        }
    }

    /// Releases the grab on the `self`, if one is in place.
    pub fn ungrab(&self) {
        unsafe {
            ffi::clutter_input_device_ungrab(self.to_glib_none().0);
        }
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
        unsafe {
            ffi::clutter_input_device_update_from_event(
                self.to_glib_none().0,
                event.to_glib_none_mut().0,
                update_stage.to_glib(),
            );
        }
    }

    /// The `Backend` that created the device.
    pub fn get_property_backend(&self) -> Option<Backend> {
        unsafe {
            let mut value = Value::from_type(<Backend as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"backend\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `backend` getter")
        }
    }

    /// The `DeviceManager` instance which owns the device
    pub fn get_property_device_manager(&self) -> Option<DeviceManager> {
        unsafe {
            let mut value = Value::from_type(<DeviceManager as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"device-manager\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `device-manager` getter")
        }
    }

    /// The unique identifier of the device
    pub fn get_property_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `id` getter")
                .unwrap()
        }
    }

    /// The name of the device
    pub fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.as_ptr() as *mut gobject_sys::GObject,
                b"name\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `name` getter")
        }
    }

    pub fn connect_property_enabled_notify<F: Fn(&InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<F: Fn(&InputDevice) + 'static>(
            this: *mut ffi::ClutterInputDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_n_axes_notify<F: Fn(&InputDevice) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_axes_trampoline<F: Fn(&InputDevice) + 'static>(
            this: *mut ffi::ClutterInputDevice,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-axes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_n_axes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for InputDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputDevice")
    }
}
