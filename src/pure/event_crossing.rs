use super::{Actor, EventFlags, EventType, InputDevice, ModifierType, Stage};

// Window, NotifyType, CrossingMode

// CrossingEvent:
// @type: event type
// @time: event time
// @flags: event flags
// @stage: event source stage
// @source: event source actor
// @x: event X coordinate
// @y: event Y coordinate
// @related: actor related to the crossing
// @device: the device that originated the event. If you want the physical
// device the event originated from, use event_get_source_device()
//
// Event for the movement of the pointer across different actors
//
pub struct CrossingEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    x: f32,
    y: f32,
    device: Option<InputDevice>,
    related: Option<Actor>,
}

impl CrossingEvent {
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    // pub fn get_subwindow(&self) -> Option<Window> {
    //     unimplemented!()
    // }

    // pub fn get_mode(&self) -> CrossingMode {
    //     unimplemented!()
    // }

    // pub fn get_detail(&self) -> NotifyType {
    //     unimplemented!()
    // }

    pub fn get_state(&self) -> ModifierType {
        unimplemented!()
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }

    pub fn get_root(&self) -> (f64, f64) {
        // let x_root = self.as_ref().x_root;
        // let y_root = self.as_ref().y_root;
        // (x_root, y_root)
        unimplemented!()
    }

    pub fn get_focus(&self) -> bool {
        unimplemented!()
    }
}
