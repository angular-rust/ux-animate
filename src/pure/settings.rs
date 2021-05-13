use super::HandlerId;
use once_cell::sync::OnceCell;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Settings {}

impl Settings {
    fn new() -> Self {
        println!("CREATE THEME INSTANCE");

        // let mut definitions = IntMap::new();
        // definitions.insert(StyleClass::MdcButton.into(), style::mdc_button());

        Self {
            // styles: Mutex::new(definitions),
        }
    }

    /// Retrieves the singleton instance of `Settings`
    ///
    /// # Returns
    ///
    /// the instance of `Settings`. The
    ///  returned object is owned by internals and it should not be unreferenced
    ///  directly
    pub fn global() -> &'static Self {
        static SETTINGS_INSTANCE: OnceCell<Settings> = OnceCell::new();
        SETTINGS_INSTANCE.get_or_init(Self::new)
    }

    /// The default distance that the cursor of a pointer device
    /// should travel before a drag operation should start.
    pub fn get_dnd_drag_threshold(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"dnd-drag-threshold\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `dnd-drag-threshold` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The default distance that the cursor of a pointer device
    /// should travel before a drag operation should start.
    pub fn set_dnd_drag_threshold(&self, dnd_drag_threshold: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"dnd-drag-threshold\0".as_ptr() as *const _,
        //         Value::from(&dnd_drag_threshold).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The maximum distance, in pixels, between button-press events that
    /// determines whether or not to increase the click count by 1.
    pub fn get_double_click_distance(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"double-click-distance\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `double-click-distance` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The maximum distance, in pixels, between button-press events that
    /// determines whether or not to increase the click count by 1.
    pub fn set_double_click_distance(&self, double_click_distance: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"double-click-distance\0".as_ptr() as *const _,
        //         Value::from(&double_click_distance).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The time, in milliseconds, that should elapse between button-press
    /// events in order to increase the click count by 1.
    pub fn get_double_click_time(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"double-click-time\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `double-click-time` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The time, in milliseconds, that should elapse between button-press
    /// events in order to increase the click count by 1.
    pub fn set_double_click_time(&self, double_click_time: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"double-click-time\0".as_ptr() as *const _,
        //         Value::from(&double_click_time).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// Whether or not to use antialiasing when rendering text; a value
    /// of 1 enables it unconditionally; a value of 0 disables it
    /// unconditionally; and -1 will use the system's default.
    pub fn get_font_antialias(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-antialias\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-antialias` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// Whether or not to use antialiasing when rendering text; a value
    /// of 1 enables it unconditionally; a value of 0 disables it
    /// unconditionally; and -1 will use the system's default.
    pub fn set_font_antialias(&self, font_antialias: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-antialias\0".as_ptr() as *const _,
        //         Value::from(&font_antialias).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The DPI used when rendering text, as a value of 1024 * dots/inch.
    ///
    /// If set to -1, the system's default will be used instead
    pub fn get_font_dpi(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-dpi\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-dpi` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// The DPI used when rendering text, as a value of 1024 * dots/inch.
    ///
    /// If set to -1, the system's default will be used instead
    pub fn set_font_dpi(&self, font_dpi: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-dpi\0".as_ptr() as *const _,
        //         Value::from(&font_dpi).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The style of the hinting used when rendering text. Valid values
    /// are:
    ///
    ///  - hintnone
    ///  - hintslight
    ///  - hintmedium
    ///  - hintfull
    pub fn get_font_hint_style(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<GString as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-hint-style\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-hint-style` getter")
        // }
        unimplemented!()
    }

    /// The style of the hinting used when rendering text. Valid values
    /// are:
    ///
    ///  - hintnone
    ///  - hintslight
    ///  - hintmedium
    ///  - hintfull
    pub fn set_font_hint_style(&self, font_hint_style: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-hint-style\0".as_ptr() as *const _,
        //         Value::from(font_hint_style).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// Whether or not to use hinting when rendering text; a value of 1
    /// unconditionally enables it; a value of 0 unconditionally disables
    /// it; and a value of -1 will use the system's default.
    pub fn get_font_hinting(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-hinting\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-hinting` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// Whether or not to use hinting when rendering text; a value of 1
    /// unconditionally enables it; a value of 0 unconditionally disables
    /// it; and a value of -1 will use the system's default.
    pub fn set_font_hinting(&self, font_hinting: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-hinting\0".as_ptr() as *const _,
        //         Value::from(&font_hinting).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The default font name that should be used by text actors, as
    /// a string that can be passed to `pango::FontDescription::from_string`.
    pub fn get_font_name(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<GString as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-name\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-name` getter")
        // }
        unimplemented!()
    }

    /// The default font name that should be used by text actors, as
    /// a string that can be passed to `pango::FontDescription::from_string`.
    pub fn set_font_name(&self, font_name: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-name\0".as_ptr() as *const _,
        //         Value::from(font_name).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// The type of sub-pixel antialiasing used when rendering text. Valid
    /// values are:
    ///
    ///  - none
    ///  - rgb
    ///  - bgr
    ///  - vrgb
    ///  - vbgr
    pub fn get_font_subpixel_order(&self) -> Option<String> {
        // unsafe {
        //     let mut value = Value::from_type(<GString as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-subpixel-order\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `font-subpixel-order` getter")
        // }
        unimplemented!()
    }

    /// The type of sub-pixel antialiasing used when rendering text. Valid
    /// values are:
    ///
    ///  - none
    ///  - rgb
    ///  - bgr
    ///  - vrgb
    ///  - vbgr
    pub fn set_font_subpixel_order(&self, font_subpixel_order: Option<&str>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"font-subpixel-order\0".as_ptr() as *const _,
        //         Value::from(font_subpixel_order).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn set_fontconfig_timestamp(&self, fontconfig_timestamp: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"fontconfig-timestamp\0".as_ptr() as *const _,
        //         Value::from(&fontconfig_timestamp).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// Sets the minimum duration for a press to be recognized as a long press
    /// gesture. The duration is expressed in milliseconds.
    ///
    /// See also `ClickAction:long-press-duration`.
    pub fn get_long_press_duration(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"long-press-duration\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `long-press-duration` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    /// Sets the minimum duration for a press to be recognized as a long press
    /// gesture. The duration is expressed in milliseconds.
    ///
    /// See also `ClickAction:long-press-duration`.
    pub fn set_long_press_duration(&self, long_press_duration: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"long-press-duration\0".as_ptr() as *const _,
        //         Value::from(&long_press_duration).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn get_password_hint_time(&self) -> u32 {
        // unsafe {
        //     let mut value = Value::from_type(<u32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"password-hint-time\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `password-hint-time` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    pub fn set_password_hint_time(&self, password_hint_time: u32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"password-hint-time\0".as_ptr() as *const _,
        //         Value::from(&password_hint_time).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn set_unscaled_font_dpi(&self, unscaled_font_dpi: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"unscaled-font-dpi\0".as_ptr() as *const _,
        //         Value::from(&unscaled_font_dpi).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn get_window_scaling_factor(&self) -> i32 {
        // unsafe {
        //     let mut value = Value::from_type(<i32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"window-scaling-factor\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `window-scaling-factor` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    pub fn set_window_scaling_factor(&self, window_scaling_factor: i32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.as_ptr() as *mut gobject_sys::GObject,
        //         b"window-scaling-factor\0".as_ptr() as *const _,
        //         Value::from(&window_scaling_factor).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    pub fn connect_dnd_drag_threshold_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_double_click_distance_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_double_click_time_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_antialias_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_dpi_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_hint_style_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_hinting_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_name_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_font_subpixel_order_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_fontconfig_timestamp_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_long_press_duration_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_password_hint_time_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_unscaled_font_dpi_notify<F: Fn(&Settings) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_window_scaling_factor_notify<F: Fn(&Settings) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Settings")
    }
}
