use super::HandlerId;
use crate::{Color, RgbaColor};
use std::fmt;

// * SECTION:clutter-colorize-effect
// * @short_description: A colorization effect
// * @see_also: #Effect, #OffscreenEffect
// *
// * #ColorizeEffect is a sub-class of #Effect that
// * colorizes an actor with the given tint.

// @extends OffscreenEffect, Effect, ActorMeta
pub struct ColorizeEffect {
    // OffscreenEffect parent_instance;

    /* the tint of the colorization */
    tint: Color,

    tint_uniform: i32,

    tex_width: i32,
    tex_height: i32,

    pipeline: Option<dx::pure::Pipeline>,
}

impl ColorizeEffect {
    /// Creates a new `ColorizeEffect` to be used with
    /// `ActorExt::add_effect`
    /// ## `tint`
    /// the color to be used
    ///
    /// # Returns
    ///
    /// the newly created `ColorizeEffect` or `None`
    pub fn new(tint: Color) -> ColorizeEffect {
        let RgbaColor {
            red,
            green,
            blue,
            alpha,
        } = tint.into();
        unimplemented!()
    }

    /// Retrieves the tint used by `self`
    /// ## `tint`
    /// return location for the color used
    pub fn get_tint(&self) -> Color {
        self.tint
    }

    /// Sets the tint to be used when colorizing
    /// ## `tint`
    /// the color to be used
    pub fn set_tint(&self, tint: Color) {
        let RgbaColor {
            red,
            green,
            blue,
            alpha,
        } = tint.into();
        unimplemented!()
    }

    pub fn connect_property_tint_notify<F: Fn(&ColorizeEffect) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ColorizeEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorizeEffect")
    }
}
