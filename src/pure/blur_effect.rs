use super::Actor;
use std::fmt;

// * @short_description: A blur effect
// * @see_also: #Effect, #OffscreenEffect
// *
// * #BlurEffect is a sub-class of #Effect that allows blurring a
// * actor and its contents.
// *
// * #BlurEffect is available since  1.4
// @extends OffscreenEffect, Effect, ActorMeta,
#[derive(Default)]
pub struct BlurEffect {
    // parent_instance: OffscreenEffect,

    // a back pointer to our actor, so that we can query it
    actor: Option<Actor>,

    pixel_step_uniform: i32,

    tex_width: u32,
    tex_height: u32,

    pipeline: Option<dx::pure::Pipeline>,
}

impl BlurEffect {
    /// Creates a new `BlurEffect` to be used with
    /// `ActorExt::add_effect`
    ///
    /// # Returns
    ///
    /// the newly created `BlurEffect` or `None`
    pub fn new() -> BlurEffect {
        Self {
            actor: None,
            pixel_step_uniform: 0,
            tex_width: 0,
            tex_height: 0,
            pipeline: None,
        }
    }
}

impl fmt::Display for BlurEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BlurEffect")
    }
}
