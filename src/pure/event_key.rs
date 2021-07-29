use super::{Actor, EventFlags, EventType, InputDevice, ModifierType, Stage};

// KeyEvent:
// @type: event type
// @time: event time
// @flags: event flags
// @stage: event source stage
// @source: event source actor
// @modifier_state: key modifiers
// @keyval: raw key value
// @hardware_keycode: raw hardware key value
// @unicode_value: Unicode representation
// @device: the device that originated the event. If you want the physical
// device the event originated from, use clutter_event_get_source_device()
//
// Key event
pub struct KeyEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    modifier_state: ModifierType,
    keyval: u32,
    hardware_keycode: u16,
    unicode_value: u64, // gunichar
    device: Option<InputDevice>,
}

impl KeyEvent {
    pub fn get_time(&self) -> u32 {
        unimplemented!()
    }

    pub fn get_state(&self) -> ModifierType {
        unimplemented!()
    }

    pub fn get_keyval(&self) -> u32 {
        self.keyval
    }

    pub fn get_length(&self) -> u32 {
        // let length = self.as_ref().length;
        // assert!(length >= 0, "Unexpected negative value");
        // length as u32
        unimplemented!()
    }

    pub fn get_hardware_keycode(&self) -> u16 {
        self.hardware_keycode
    }

    pub fn get_group(&self) -> u8 {
        // self.as_ref().group
        unimplemented!()
    }

    pub fn get_is_modifier(&self) -> bool {
        // self.as_ref().is_modifier & 1 != 0
        unimplemented!()
    }
}
