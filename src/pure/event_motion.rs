use super::{Actor, EventFlags, EventType, InputDevice, ModifierType, Stage};

// * ClutterMotionEvent:
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @stage: event source stage
// * @source: event source actor
// * @x: event X coordinate
// * @y: event Y coordinate
// * @modifier_state: button modifiers
// * @axes: reserved for future use
// * @device: the device that originated the event. If you want the physical
// * device the event originated from, use clutter_event_get_source_device()
// *
// * Event for the pointer motion
#[derive(Debug, Clone)]
pub struct MotionEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    x: f32,
    y: f32,
    modifier_state: ModifierType,
    axes: Option<f64>, /* Future use */
    device: Option<InputDevice>,
}

impl MotionEvent {
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

    pub fn get_time(&self) -> u32 {
        // self.as_ref().time
        unimplemented!()
    }

    pub fn request_motions(&self) {
        // unsafe { ffi::gdk_event_request_motions(self.as_ref()) }
        unimplemented!()
    }

    pub fn get_device(&self) -> Option<InputDevice> {
        // unsafe { from_glib_none(self.as_ref().device) }
        unimplemented!()
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

    pub fn get_is_hint(&self) -> bool {
        // from_glib(self.as_ref().is_hint as _)
        unimplemented!()
    }
}
