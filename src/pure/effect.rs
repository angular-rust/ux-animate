use crate::prelude::*;
use std::fmt;

// * SECTION:clutter-effect
// * @short_description: Base class for actor effects
// *
// * The #Effect class provides a default type and API for creating
// * effects for generic actors.
// *
// * Effects are a #ActorMeta sub-class that modify the way an actor
// * is painted in a way that is not part of the actor's implementation.
// *
// * Effects should be the preferred way to affect the paint sequence of an
// * actor without sub-classing the actor itself and overriding the
// * #ActorClass.paint()_ virtual function.
// *
// * ## Implementing a Effect
// *
// * Creating a sub-class of #Effect requires overriding the
// * #EffectClass.paint() method. The implementation of the function should look
// * something like this:
// *
// * |[
// * void effect_paint (Effect *effect, EffectPaintFlags flags)
// * {
// *   // Set up initialisation of the paint such as binding a
// *   // CoglOffscreen or other operations
// *
// *   // Chain to the next item in the paint sequence. This will either call
// *   // ‘paint’ on the next effect or just paint the actor if this is
// *   // the last effect.
// *   Actor *actor =
// *     clutter_actor_meta_get_actor (CLUTTER_ACTOR_META (effect));
// *
// *   clutter_actor_continue_paint (actor);
// *
// *   // perform any cleanup of state, such as popping the CoglOffscreen
// * }
// * ]|
// *
// * The effect can optionally avoid calling clutter_actor_continue_paint() to skip any
// * further stages of the paint sequence. This is useful for example if the effect
// * contains a cached image of the actor. In that case it can optimise painting by
// * avoiding the actor paint and instead painting the cached image.
// *
// * The %CLUTTER_EFFECT_PAINT_ACTOR_DIRTY flag is useful in this case.  will set
// * this flag when a redraw has been queued on the actor since it was last painted. The
// * effect can use this information to decide if the cached image is still valid.
// *
// * ## A simple Effect implementation
// *
// * The example below creates two rectangles: one will be painted "behind" the actor,
// * while another will be painted "on top" of the actor.
// *
// * The #ActorMetaClass.set_actor() implementation will create the two materials
// * used for the two different rectangles; the #EffectClass.paint() implementation
// * will paint the first material using dx_rectangle(), before continuing and then it
// * will paint paint the second material after.
// *
// *  |[
// *  typedef struct {
// *    Effect parent_instance;
// *
// *    CoglHandle rect_1;
// *    CoglHandle rect_2;
// *  } MyEffect;
// *
// *  typedef struct _EffectClass MyEffectClass;
// *
// *  G_DEFINE_TYPE (MyEffect, my_effect, CLUTTER_TYPE_EFFECT);
// *
// *  static void
// *  my_effect_set_actor (ActorMeta *meta,
// *                       Actor     *actor)
// *  {
// *    MyEffect *self = MY_EFFECT (meta);
// *
// *    // Clear the previous state //
// *    if (self->rect_1)
// *      {
// *        dx_handle_unref (self->rect_1);
// *        self->rect_1 = NULL;
// *      }
// *
// *    if (self->rect_2)
// *      {
// *        dx_handle_unref (self->rect_2);
// *        self->rect_2 = NULL;
// *      }
// *
// *    // Maintain a pointer to the actor
// *    self->actor = actor;
// *
// *    // If we've been detached by the actor then we should just bail out here
// *    if (self->actor == NULL)
// *      return;
// *
// *    // Create a red material
// *    self->rect_1 = dx_material_new ();
// *    dx_material_set_color4f (self->rect_1, 1.0, 0.0, 0.0, 1.0);
// *
// *    // Create a green material
// *    self->rect_2 = dx_material_new ();
// *    dx_material_set_color4f (self->rect_2, 0.0, 1.0, 0.0, 1.0);
// *  }
// *
// *  static gboolean
// *  my_effect_paint (Effect *effect)
// *  {
// *    MyEffect *self = MY_EFFECT (effect);
// *    gfloat width, height;
// *
// *    clutter_actor_get_size (self->actor, &width, &height);
// *
// *    // Paint the first rectangle in the upper left quadrant
// *    dx_set_source (self->rect_1);
// *    dx_rectangle (0, 0, width / 2, height / 2);
// *
// *    // Continue to the rest of the paint sequence
// *    clutter_actor_continue_paint (self->actor);
// *
// *    // Paint the second rectangle in the lower right quadrant
// *    dx_set_source (self->rect_2);
// *    dx_rectangle (width / 2, height / 2, width, height);
// *  }
// *
// *  static void
// *  my_effect_class_init (MyEffectClass *klass)
// *  {
// *    ActorMetaClas *meta_class = CLUTTER_ACTOR_META_CLASS (klass);
// *
// *    meta_class->set_actor = my_effect_set_actor;
// *
// *    klass->paint = my_effect_paint;
// *  }
// * ]|
// *
// @extends ActorMeta
#[derive(Debug, Clone)]
pub struct Effect {}

impl Object for Effect {}
impl Is<Effect> for Effect {}

impl AsRef<Effect> for Effect {
    fn as_ref(&self) -> &Effect {
        self
    }
}

/// Trait containing all `Effect` methods.
///
/// # Implementors
///
/// [`Effect`](struct.Effect.html), [`OffscreenEffect`](struct.OffscreenEffect.html)
pub trait EffectExt: 'static {
    /// Queues a repaint of the effect. The effect can detect when the ‘paint’
    /// method is called as a result of this function because it will not
    /// have the `EffectPaintFlags::ActorDirty` flag set. In that case the
    /// effect is free to assume that the actor has not changed its
    /// appearance since the last time it was painted so it doesn't need to
    /// call `ActorExt::continue_paint` if it can draw a cached
    /// image. This is mostly intended for effects that are using a
    /// `dx::Offscreen` to redirect the actor (such as
    /// `OffscreenEffect`). In that case the effect can save a bit of
    /// rendering time by painting the cached texture without causing the
    /// entire actor to be painted.
    ///
    /// This function can be used by effects that have their own animatable
    /// parameters. For example, an effect which adds a varying degree of a
    /// red tint to an actor by redirecting it through a CoglOffscreen
    /// might have a property to specify the level of tint. When this value
    /// changes, the underlying actor doesn't need to be redrawn so the
    /// effect can call `EffectExt::queue_repaint` to make sure the
    /// effect is repainted.
    ///
    /// Note however that modifying the position of the parent of an actor
    /// may change the appearance of the actor because its transformation
    /// matrix would change. In this case a redraw wouldn't be queued on
    /// the actor itself so the `EffectPaintFlags::ActorDirty` would still
    /// not be set. The effect can detect this case by keeping track of the
    /// last modelview matrix that was used to render the actor and
    /// veryifying that it remains the same in the next paint.
    ///
    /// Any other effects that are layered on top of the passed in effect
    /// will still be passed the `EffectPaintFlags::ActorDirty` flag. If
    /// anything queues a redraw on the actor without specifying an effect
    /// or with an effect that is lower in the chain of effects than this
    /// one then that will override this call. In that case this effect
    /// will instead be called with the `EffectPaintFlags::ActorDirty`
    /// flag set.
    fn queue_repaint(&self);
}

impl<O: Is<Effect>> EffectExt for O {
    fn queue_repaint(&self) {
        // unsafe {
        //     ffi::clutter_effect_queue_repaint(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Effect")
    }
}
