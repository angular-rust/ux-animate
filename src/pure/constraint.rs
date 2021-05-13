use crate::prelude::*;
use std::fmt;

// * SECTION:clutter-constraint
// * @Title: Constraint
// * @Short_Description: Abstract class for constraints on position or size
// * @See_Also: #Action
// *
// * #Constraint is a base abstract class for modifiers of a #Actor
// * position or size.
// *
// * A #Constraint sub-class should contain the logic for modifying
// * the position or size of the #Actor to which it is applied, by
// * updating the actor's allocation. Each #Constraint can change the
// * allocation of the actor to which they are applied by overriding the
// * #ConstraintClass.update_allocation() virtual function.
// *
// * #Constraint is available since  1.4
// *
// * ## Using Constraints
// *
// * Constraints can be used with fixed layout managers, like
// * #FixedLayout, or with actors implicitly using a fixed layout
// * manager, like #Group and #Stage.
// *
// * Constraints provide a way to build user interfaces by using
// * relations between #Actors, without explicit fixed
// * positioning and sizing, similarly to how fluid layout managers like
// * #BoxLayout and #TableLayout lay out their children.
// *
// * Constraints are attached to a #Actor, and are available
// * for inspection using clutter_actor_get_constraints().
// *
// *  provides different implementation of the #Constraint
// * abstract class, for instance:
// *
// *  - #AlignConstraint, a constraint that can be used to align
// *  an actor to another one on either the horizontal or the vertical
// *  axis, using a normalized value between 0 and 1.
// *  - #BindConstraint, a constraint binds the X, Y, width or height
// *  of an actor to the corresponding position or size of a source actor,
// *  with or without an offset.
// *  - #SnapConstraint, a constraint that "snaps" together the edges
// *  of two #Actors; if an actor uses two constraints on both its
// *  horizontal or vertical edges then it can also expand to fit the empty
// *  space.
// *
// * The [constraints example](https://git.gnome.org/browse/clutter/tree/examples/constraints.c?h=clutter-1.18)
// * uses various types of #Constraints to lay out three actors on a
// * resizable stage. Only the central actor has an explicit size, and no
// * actor has an explicit position.
// *
// *  - The #Actor with #Actor:name `layerA` is explicitly
// *  sized to 100 pixels by 25 pixels, and it's added to the #Stage
// *  - two #AlignConstraints are used to anchor `layerA` to the
// *  center of the stage, by using 0.5 as the alignment #AlignConstraint:factor on
// *  both the X and Y axis
// *  - the #Actor with #Actor:name `layerB` is added to the
// *  #Stage with no explicit size
// *  - the #Actor:x and #Actor:width of `layerB` are bound
// *  to the same properties of `layerA` using two #BindConstraint
// *  objects, thus keeping `layerB` aligned to `layerA`
// *  - the top edge of `layerB` is snapped together with the bottom edge
// *  of `layerA`; the bottom edge of `layerB` is also snapped together with
// *  the bottom edge of the #Stage; an offset is  given to the two
// *  #SnapConstraintss to allow for some padding; since `layerB` is
// *  snapped between two different #Actors, its height is stretched
// *  to match the gap
// *  - the #Actor with #Actor:name `layerC` mirrors `layerB`,
// *  snapping the top edge of the #Stage to the top edge of `layerC`
// *  and the top edge of `layerA` to the bottom edge of `layerC`
// *
// * You can try resizing interactively the #Stage and verify
// * that the three #Actors maintain the same position and
// * size relative to each other, and to the #Stage.
// *
// * It is important to note that  does not avoid loops or
// * competing constraints; if two or more #Constraints
// * are operating on the same positional or dimensional attributes of an
// * actor, or if the constraints on two different actors depend on each
// * other, then the behavior is undefined.
// *
// * ## Implementing a Constraint
// *
// * Creating a sub-class of #Constraint requires the
// * implementation of the #ConstraintClass.update_allocation()
// * virtual function.
// *
// * The `update_allocation()` virtual function is called during the
// * allocation sequence of a #Actor, and allows any #Constraint
// * attached to that actor to modify the allocation before it is passed to
// * the actor's #ActorClass.allocate() implementation.
// *
// * The #ActorBox passed to the `update_allocation()` implementation
// * contains the original allocation of the #Actor, plus the eventual
// * modifications applied by the other #Constraints, in the same order
// * the constraints have been applied to the actor.
// *
// * It is not necessary for a #Constraint sub-class to chain
// * up to the parent's implementation.
// *
// * If a #Constraint is parametrized - i.e. if it contains
// * properties that affect the way the constraint is implemented - it should
// * call clutter_actor_queue_relayout() on the actor to which it is attached
// * to whenever any parameter is changed. The actor to which it is attached
// * can be recovered at any point using clutter_actor_meta_get_actor().
// @extends ActorMeta,
#[derive(Debug, Clone)]
pub struct Constraint {
}

impl Constraint {}

impl Object for Constraint {}
impl Is<Constraint> for Constraint {}

impl AsRef<Constraint> for Constraint {
    fn as_ref(&self) -> &Constraint {
        self
    }
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Constraint")
    }
}
