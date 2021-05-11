use crate::prelude::*;
use super::{ActorMeta, Effect, OffscreenEffect};
// use glib::{object as gobject, object::Cast, translate::*};
use std::fmt;

// * @short_description: A blur effect
// * @see_also: #ClutterEffect, #ClutterOffscreenEffect
// *
// * #ClutterBlurEffect is a sub-class of #ClutterEffect that allows blurring a
// * actor and its contents.
// *
// * #ClutterBlurEffect is available since Clutter 1.4
// @extends OffscreenEffect, Effect, ActorMeta,
pub struct BlurEffect {
//     ClutterOffscreenEffect parent_instance;

//     /* a back pointer to our actor, so that we can query it */
//     ClutterActor *actor;
  
//     gint pixel_step_uniform;
  
//     gint tex_width;
//     gint tex_height;
  
//     CoglPipeline *pipeline;
}

impl BlurEffect {
    /// Creates a new `BlurEffect` to be used with
    /// `ActorExt::add_effect`
    ///
    /// # Returns
    ///
    /// the newly created `BlurEffect` or `None`
    pub fn new() -> BlurEffect {
        unimplemented!()
    }
}

impl Default for BlurEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for BlurEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlurEffect")
    }
}
