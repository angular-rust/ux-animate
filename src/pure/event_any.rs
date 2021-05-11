use super::{Actor, EventFlags, EventType, ModifierType, Stage};

// * ClutterAnyEvent:
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @source: event source actor
// *
// * Common members for a #ClutterEvent
// *
#[derive(Debug, Clone)]
pub struct AnyEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,
}

impl AnyEvent {
    pub fn get_time(&self) -> u32 {
        unimplemented!()
    }

    pub fn get_state(&self) -> ModifierType {
        unimplemented!()
    }

    // pub fn get_keyval(&self) -> keys::Key {
    //     unimplemented!()
    // }

    pub fn get_length(&self) -> u32 {
        unimplemented!()
    }

    pub fn get_hardware_keycode(&self) -> u16 {
        unimplemented!()
    }

    pub fn get_group(&self) -> u8 {
        unimplemented!()
    }

    pub fn get_is_modifier(&self) -> bool {
        unimplemented!()
    }
}
