use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrossingEvent(crate::Event);

event_wrapper!(CrossingEvent, ClutterCrossingEvent);
event_subtype!(CrossingEvent, ffi::CLUTTER_ENTER | ffi::CLUTTER_LEAVE);

impl CrossingEvent {
    // pub fn get_position(&self) -> (f64, f64) {
    //     let x = self.as_ref().x;
    //     let y = self.as_ref().y;
    //     (x, y)
    // }

    // pub fn get_subwindow(&self) -> Option<::Window> {
    //     unsafe { from_glib_none(self.as_ref().subwindow) }
    // }

    // pub fn get_mode(&self) -> ::CrossingMode {
    //     from_glib(self.as_ref().mode)
    // }

    // pub fn get_detail(&self) -> ::NotifyType {
    //     from_glib(self.as_ref().detail)
    // }

    // pub fn get_state(&self) -> ::ModifierType {
    //     from_glib(self.as_ref().state)
    // }

    // pub fn get_time(&self) -> u32 {
    //     self.as_ref().time
    // }

    // pub fn get_root(&self) -> (f64, f64) {
    //     let x_root = self.as_ref().x_root;
    //     let y_root = self.as_ref().y_root;
    //     (x_root, y_root)
    // }

    // pub fn get_focus(&self) -> bool {
    //     from_glib(self.as_ref().focus)
    // }
}
