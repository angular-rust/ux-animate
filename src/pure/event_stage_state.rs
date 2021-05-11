use super::{ModifierType, EventSequence, InputDevice, EventType, StageState, EventFlags, Stage, Actor};

// * ClutterStageStateEvent:
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @stage: event source stage
// * @source: event source actor (unused)
// * @changed_mask: bitwise OR of the changed flags
// * @new_state: bitwise OR of the current state flags
// *
// * Event signalling a change in the #ClutterStage state.
// *
#[derive(Debug, Clone)]
pub struct StageStateEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>, /* XXX: should probably be the stage itself */
  
    changed_mask: StageState,
    new_state: StageState,
}

impl StageStateEvent {
    pub fn get_time(&self) -> u32 {
        unimplemented!()
    }

    pub fn get_state(&self) -> ModifierType {
        unimplemented!()
    }

    // pub fn get_keyval(&self) -> keys::Key {
    //     from_glib(self.as_ref().keyval)
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
