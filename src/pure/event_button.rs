use super::{Actor, EventFlags, EventType, InputDevice, ModifierType, Stage};
// ButtonEvent:
// @type: event type
// @time: event time
// @flags: event flags
// @stage: event source stage
// @source: event source actor
// @x: event X coordinate, relative to the stage
// @y: event Y coordinate, relative to the stage
// @modifier_state: button modifiers
// @button: event button
// @click_count: number of button presses within the default time
//   and radius
// @axes: reserved for future use
// @device: the device that originated the event. If you want the physical
// device the event originated from, use event_get_source_device()
//
// Button event.
//
// The event coordinates are relative to the stage that received the
// event, and can be transformed into actor-relative coordinates by
// using actor_transform_stage_point().
pub struct ButtonEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    x: f32,
    y: f32,
    modifier_state: ModifierType,
    button: u32,
    click_count: u32,
    axes: Option<f64>, /* Future use */
    device: Option<InputDevice>,
}

impl ButtonEvent {
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn get_state(&self) -> ModifierType {
        self.modifier_state
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }

    pub fn get_button(&self) -> u32 {
        self.button
    }

    pub fn get_device(&self) -> &Option<InputDevice> {
        &self.device
    }

    pub fn get_axes(&self) -> Option<(f64, f64)> {
        // let axes = self.as_ref().axes;

        // if axes.is_null() {
        //     None
        // } else {
        //     unsafe { Some((*axes, *axes.offset(1))) }
        // }
        unimplemented!()
    }

    pub fn get_root(&self) -> (f64, f64) {
        // let x_root = self.as_ref().x_root;
        // let y_root = self.as_ref().y_root;
        // (x_root, y_root)
        unimplemented!()
    }
}
