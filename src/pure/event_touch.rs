use super::{Actor, EventFlags, EventSequence, EventType, InputDevice, ModifierType, Stage};

// * ClutterTouchEvent:
// * @type: event type
// * @time: event time
// * @flags: event flags
// * @stage: event source stage
// * @source: event source actor (unused)
// * @x: the X coordinate of the pointer, relative to the stage
// * @y: the Y coordinate of the pointer, relative to the stage
// * @sequence: the event sequence that this event belongs to
// * @modifier_state: (type ClutterModifierType): a bit-mask representing the state
// *   of modifier keys (e.g. Control, Shift, and Alt) and the pointer
// *   buttons. See #ClutterModifierType
// * @axes: reserved
// * @device: the device that originated the event. If you want the physical
// * device the event originated from, use clutter_event_get_source_device()
// *
// * Used for touch events.
// *
// * The @type field will be one of %CLUTTER_TOUCH_BEGIN, %CLUTTER_TOUCH_END,
// * %CLUTTER_TOUCH_UPDATE, or %CLUTTER_TOUCH_CANCEL.
// *
// * Touch events are grouped into sequences; each touch sequence will begin
// * with a %CLUTTER_TOUCH_BEGIN event, progress with %CLUTTER_TOUCH_UPDATE
// * events, and end either with a %CLUTTER_TOUCH_END event or with a
// * %CLUTTER_TOUCH_CANCEL event.
// *
// * With multi-touch capable devices there can be multiple event sequence
// * running at the same time.
// *
#[derive(Debug, Clone)]
pub struct TouchEvent {
    kind: EventType,
    time: u32,
    flags: EventFlags,
    stage: Option<Stage>,
    source: Option<Actor>,

    x: f32,
    y: f32,
    sequence: Option<EventSequence>,
    modifier_state: ModifierType,
    axes: Option<f64>, /* reserved */
    device: Option<InputDevice>,
}

impl TouchEvent {
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

    pub fn is_emulating_pointer(&self) -> bool {
        // from_glib(self.as_ref().emulating_pointer)
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

    pub fn get_event_sequence(&self) -> Option<EventSequence> {
        // unsafe { from_glib_none(self.as_ref().sequence) }
        unimplemented!()
    }
}
