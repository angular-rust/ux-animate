use crate::prelude::*;
use super::{Actor, ActorMeta, AlignAxis, Constraint};
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};


// * ClutterAlignConstraint:
// *
// * #ClutterAlignConstraint is an opaque structure
// * whose members cannot be directly accesses
// * @Short_Description: A constraint aligning the position of an actor
// *
// * #ClutterAlignConstraint is a #ClutterConstraint that aligns the position
// * of the #ClutterActor to which it is applied to the size of another
// * #ClutterActor using an alignment factor
// @extends Constraint, ActorMeta,
#[derive(Debug, Clone)]
pub struct AlignConstraint {
    // parent_instance: Constraint,

    actor: Option<Actor>,
    source: Option<Actor>,
    align_axis: AlignAxis,
    factor: f32,
}

impl AlignConstraint {
    /// Creates a new constraint, aligning a `Actor`'s position with
    /// regards of the size of the actor to `source`, with the given
    /// alignment `factor`
    /// ## `source`
    /// the `Actor` to use as the source of the
    ///  alignment, or `None`
    /// ## `axis`
    /// the axis to be used to compute the alignment
    /// ## `factor`
    /// the alignment factor, between 0.0 and 1.0
    ///
    /// # Returns
    ///
    /// the newly created `AlignConstraint`
    pub fn new<P: Is<Actor>>(source: Option<&P>, axis: AlignAxis, factor: f32) -> AlignConstraint {
        // Self {
        //     actor: None,
        //     source,
        //     align_axis: axis,
        //     factor
        // }
        unimplemented!()
    }

    /// Retrieves the value set using `AlignConstraint::set_align_axis`
    ///
    /// # Returns
    ///
    /// the alignment axis
    pub fn get_align_axis(&self) -> AlignAxis {
        self.align_axis
    }

    /// Retrieves the factor set using `AlignConstraint::set_factor`
    ///
    /// # Returns
    ///
    /// the alignment factor
    pub fn get_factor(&self) -> f32 {
        self.factor
    }

    /// Retrieves the source of the alignment
    ///
    /// # Returns
    ///
    /// the `Actor` used as the source
    ///  of the alignment
    pub fn get_source(&self) -> Option<Actor> {
        // self.source
        unimplemented!()
    }

    /// Sets the axis to which the alignment refers to
    /// ## `axis`
    /// the axis to which the alignment refers to
    pub fn set_align_axis(&mut self, axis: AlignAxis) {
        self.align_axis = axis;
    }

    /// Sets the alignment factor of the constraint
    ///
    /// The factor depends on the `AlignConstraint:align-axis` property
    /// and it is a value between 0.0 (meaning left, when
    /// `AlignConstraint:align-axis` is set to `AlignAxis::XAxis`; or
    /// meaning top, when `AlignConstraint:align-axis` is set to
    /// `AlignAxis::YAxis`) and 1.0 (meaning right, when
    /// `AlignConstraint:align-axis` is set to `AlignAxis::XAxis`; or
    /// meaning bottom, when `AlignConstraint:align-axis` is set to
    /// `AlignAxis::YAxis`). A value of 0.5 aligns in the middle in either
    /// cases
    /// ## `factor`
    /// the alignment factor, between 0.0 and 1.0
    pub fn set_factor(&mut self, factor: f32) {
        self.factor = factor;
    }

    /// Sets the source of the alignment constraint
    /// ## `source`
    /// a `Actor`, or `None` to unset the source
    pub fn set_source<P: Is<Actor>>(&self, source: Option<&P>) {
        // self.source = source;
        unimplemented!()
    }
}

impl fmt::Display for AlignConstraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AlignConstraint")
    }
}
