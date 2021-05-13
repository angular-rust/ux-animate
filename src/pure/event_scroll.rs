use super::{
    Actor, EventFlags, EventType, InputDevice, ModifierType, ScrollDirection, ScrollFinishFlags,
    ScrollSource, Stage,
};

// * ScrollEvent:
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @stage: event source stage
// * @source: event source actor
// * @x: event X coordinate
// * @y: event Y coordinate
// * @direction: direction of the scrolling
// * @modifier_state: button modifiers
// * @axes: reserved for future use
// * @device: the device that originated the event. If you want the physical
// * device the event originated from, use clutter_event_get_source_device()
// * @scroll_source: the source of scroll events. This field is available since 1.26
// * @finish_flags: the axes that were stopped in this event. This field is available since 1.26
// *
// * Scroll wheel (or similar device) event
pub struct ScrollEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    x: f32,
    y: f32,
    direction: ScrollDirection,
    modifier_state: ModifierType,
    axes: Option<f64>, /* future use */
    device: Option<InputDevice>,
    scroll_source: ScrollSource,
    finish_flags: ScrollFinishFlags,
}

impl ScrollEvent {
    pub fn get_time(&self) -> u32 {
        // self.as_ref().time
        unimplemented!()
    }

    pub fn get_position(&self) -> (f64, f64) {
        // let x = self.as_ref().x;
        // let y = self.as_ref().y;
        // (x, y)
        unimplemented!()
    }

    pub fn get_state(&self) -> ModifierType {
        // from_glib(self.as_ref().state)
        unimplemented!()
    }

    pub fn get_device(&self) -> Option<InputDevice> {
        // unsafe { from_glib_none(self.as_ref().device) }
        unimplemented!()
    }

    pub fn get_direction(&self) -> ScrollDirection {
        // from_glib(self.as_ref().direction)
        unimplemented!()
    }

    pub fn get_root(&self) -> (f64, f64) {
        // let x_root = self.as_ref().x_root;
        // let y_root = self.as_ref().y_root;
        // (x_root, y_root)
        unimplemented!()
    }

    pub fn get_delta(&self) -> (f64, f64) {
        // let dx = self.as_ref().delta_x;
        // let dy = self.as_ref().delta_y;
        // (dx, dy)
        unimplemented!()
    }

    pub fn get_is_stop(&self) -> bool {
        // self.as_ref().is_stop & 1 != 0
        unimplemented!()
    }
}
