use super::{
    Actor, EventFlags, EventType, ModifierType, Stage,
    TouchpadGesturePhase,
};

// * TouchpadSwipeEvent
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @stage: event source stage
// * @source: event source actor (unused)
// * @phase: the current phase of the gesture
// * @n_fingers: the number of fingers triggering the swipe
// * @x: the X coordinate of the pointer, relative to the stage
// * @y: the Y coordinate of the pointer, relative to the stage
// * @dx: movement delta of the pinch focal point in the X axis
// * @dy: movement delta of the pinch focal point in the Y axis
// *
// * Used for touchpad swipe gesture events. The current state of the
// * gesture will be determined by the @phase field.
// *
pub struct TouchpadSwipeEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    phase: TouchpadGesturePhase,
    n_fingers: u32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl TouchpadSwipeEvent {
    pub fn get_time(&self) -> u32 {
        // self.as_ref().time
        unimplemented!()
    }

    pub fn get_state(&self) -> ModifierType {
        // from_glib(self.as_ref().state)
        unimplemented!()
    }

    // pub fn get_keyval(&self) -> keys::Key {
    //     from_glib(self.as_ref().keyval)
    // }

    pub fn get_length(&self) -> u32 {
        // let length = self.as_ref().length;
        // assert!(length >= 0, "Unexpected negative value");
        // length as u32
        unimplemented!()
    }

    pub fn get_hardware_keycode(&self) -> u16 {
        // self.as_ref().hardware_keycode
        unimplemented!()
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
