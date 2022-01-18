use super::{Actor, BindCoordinate, HandlerId};
use crate::prelude::*;
use std::fmt;

// @Title: BindConstraint
// @Short_Description: A constraint binding the position or size of an actor
//
// #BindConstraint is a #Constraint that binds the
// position or the size of the #Actor to which it is applied
// to the the position or the size of another #Actor, or
// "source".
//
// An offset can be applied to the constraint, to avoid overlapping. The offset
// can also be animated. For instance, the following code will set up three
// actors to be bound to the same origin:
//
// ```
// // source
// rect[0] = rectangle_new_with_color (&red_color);
// actor_set_position (rect[0], x_pos, y_pos);
// actor_set_size (rect[0], 100, 100);
//
// // second rectangle
// rect[1] = rectangle_new_with_color (&green_color);
// actor_set_size (rect[1], 100, 100);
// actor_set_opacity (rect[1], 0);
//
// constraint = bind_constraint_new (rect[0], BIND_X, 0.0);
// actor_add_constraint_with_name (rect[1], "green-x", constraint);
// constraint = bind_constraint_new (rect[0], BIND_Y, 0.0);
// actor_add_constraint_with_name (rect[1], "green-y", constraint);
//
// // third rectangle
// rect[2] = rectangle_new_with_color (&blue_color);
// actor_set_size (rect[2], 100, 100);
// actor_set_opacity (rect[2], 0);
//
// constraint = bind_constraint_new (rect[0], BIND_X, 0.0);
// actor_add_constraint_with_name (rect[2], "blue-x", constraint);
// constraint = bind_constraint_new (rect[0], BIND_Y, 0.0);
// actor_add_constraint_with_name (rect[2], "blue-y", constraint);
// ```
//
// The following code animates the second and third rectangles to "expand"
// them horizontally from underneath the first rectangle:
//
// ```
// actor_animate (rect[1], EASE_OUT_CUBIC, 250,
//                        "@constraints.green-x.offset", 100.0,
//                        "opacity", 255,
//                        NULL);
// actor_animate (rect[2], EASE_OUT_CUBIC, 250,
//                        "@constraints.blue-x.offset", 200.0,
//                        "opacity", 255,
//                        NULL);
// ```
//
// #BindConstraint is available since  1.4
// @extends Constraint, ActorMeta
#[derive(Debug, Clone)]
pub struct BindConstraint {
    // parent_instance: Constraint,
    actor: Option<Actor>,
    source: Option<Actor>,
    coordinate: BindCoordinate,
    offset: f32,
}

impl BindConstraint {
    /// Creates a new constraint, binding a `Actor`'s position to
    /// the given `coordinate` of the position of `source`
    /// ## `source`
    /// the `Actor` to use as the source of
    ///  the binding, or `None`
    /// ## `coordinate`
    /// the coordinate to bind
    /// ## `offset`
    /// the offset to apply to the binding, in pixels
    ///
    /// # Returns
    ///
    /// the newly created `BindConstraint`
    pub fn new<P: Is<Actor>>(
        source: Option<&P>,
        coordinate: BindCoordinate,
        offset: f32,
    ) -> BindConstraint {
        // Self {}
        unimplemented!()
    }

    /// Retrieves the bound coordinate of the constraint
    ///
    /// # Returns
    ///
    /// the bound coordinate
    pub fn get_coordinate(&self) -> BindCoordinate {
        self.coordinate
    }

    /// Retrieves the offset set using `BindConstraint::set_offset`
    ///
    /// # Returns
    ///
    /// the offset, in pixels
    pub fn get_offset(&self) -> f32 {
        self.offset
    }

    /// Retrieves the `Actor` set using `BindConstraint::set_source`
    ///
    /// # Returns
    ///
    /// a pointer to the source actor
    pub fn get_source(&self) -> Option<Actor> {
        // self.source
        unimplemented!()
    }

    /// Sets the coordinate to bind in the constraint
    /// ## `coordinate`
    /// the coordinate to bind
    pub fn set_coordinate(&self, coordinate: BindCoordinate) {
        unimplemented!()
    }

    /// Sets the offset to be applied to the constraint
    /// ## `offset`
    /// the offset to apply, in pixels
    pub fn set_offset(&self, offset: f32) {
        unimplemented!()
    }

    /// Sets the source `Actor` for the constraint
    /// ## `source`
    /// a `Actor`, or `None` to unset the source
    pub fn set_source<P: Is<Actor>>(&self, source: Option<&P>) {
        unimplemented!()
    }

    pub fn connect_property_coordinate_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_offset_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    pub fn connect_property_source_notify<F: Fn(&BindConstraint) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for BindConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BindConstraint")
    }
}
