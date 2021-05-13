use glib::signal::SignalHandlerId;
use std::fmt;

// * @short_description: Increase/decrease brightness and/or contrast of actor.
// * @see_also: #Effect, #OffscreenEffect
// *
// * #BrightnessContrastEffect is a sub-class of #Effect that
// * changes the overall brightness of a #Actor.
// *
// * #BrightnessContrastEffect is available since  1.10
// @extends OffscreenEffect, Effect, ActorMeta
pub struct BrightnessContrastEffect {
    // OffscreenEffect parent_instance;

    /* Brightness and contrast changes. */
    brightness_red: f32,
    brightness_green: f32,
    brightness_blue: f32,

    contrast_red: f32,
    contrast_green: f32,
    contrast_blue: f32,

    brightness_multiplier_uniform: i32,
    brightness_offset_uniform: i32,
    contrast_uniform: i32,

    tex_width: i32,
    tex_height: i32,

    pipeline: Option<dx::pure::Pipeline>,
}

impl BrightnessContrastEffect {
    /// Creates a new `BrightnessContrastEffect` to be used with
    /// `ActorExt::add_effect`
    ///
    /// # Returns
    ///
    /// the newly created
    ///  `BrightnessContrastEffect` or `None`. Use `gobject::ObjectExt::unref` when
    ///  done.
    pub fn new() -> BrightnessContrastEffect {
        unimplemented!()
    }

    /// Retrieves the change in brightness used by `self`.
    /// ## `red`
    /// return location for red component of the
    ///  change in brightness
    /// ## `green`
    /// return location for green component of the
    ///  change in brightness
    /// ## `blue`
    /// return location for blue component of the
    ///  change in brightness
    pub fn get_brightness(&self) -> (f32, f32, f32) {
        (
            self.brightness_red,
            self.brightness_green,
            self.brightness_blue,
        )
    }

    /// Retrieves the contrast value used by `self`.
    /// ## `red`
    /// return location for red component of the
    ///  change in contrast
    /// ## `green`
    /// return location for green component of the
    ///  change in contrast
    /// ## `blue`
    /// return location for blue component of the
    ///  change in contrast
    pub fn get_contrast(&self) -> (f32, f32, f32) {
        (self.contrast_red, self.contrast_green, self.contrast_blue)
    }

    /// The range of `brightness` is [-1.0, 1.0], where 0.0 designates no change;
    /// a value below 0.0 indicates a decrease in brightness; and a value
    /// above 0.0 indicates an increase of brightness.
    /// ## `brightness`
    /// the brightness change for all three components (r, g, b)
    pub fn set_brightness(&self, brightness: f32) {
        unimplemented!()
    }

    /// The range for each component is [-1.0, 1.0] where 0.0 designates no change,
    /// values below 0.0 mean a decrease in brightness, and values above indicate
    /// an increase.
    /// ## `red`
    /// red component of the change in brightness
    /// ## `green`
    /// green component of the change in brightness
    /// ## `blue`
    /// blue component of the change in brightness
    pub fn set_brightness_full(&self, red: f32, green: f32, blue: f32) {
        unimplemented!()
    }

    /// The range for `contrast` is [-1.0, 1.0], where 0.0 designates no change;
    /// a value below 0.0 indicates a decrease in contrast; and a value above
    /// 0.0 indicates an increase.
    /// ## `contrast`
    /// contrast change for all three channels
    pub fn set_contrast(&self, contrast: f32) {
        unimplemented!()
    }

    /// The range for each component is [-1.0, 1.0] where 0.0 designates no change,
    /// values below 0.0 mean a decrease in contrast, and values above indicate
    /// an increase.
    /// ## `red`
    /// red component of the change in contrast
    /// ## `green`
    /// green component of the change in contrast
    /// ## `blue`
    /// blue component of the change in contrast
    pub fn set_contrast_full(&self, red: f32, green: f32, blue: f32) {
        unimplemented!()
    }

    pub fn connect_property_brightness_notify<F: Fn(&BrightnessContrastEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    pub fn connect_property_contrast_notify<F: Fn(&BrightnessContrastEffect) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }
}

impl Default for BrightnessContrastEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BrightnessContrastEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BrightnessContrastEffect")
    }
}
