use super::{Alpha, Knot, RotateAxis, RotateDirection};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// * @Title: ClutterBehaviourEllipse
// * @short_description: A behaviour interpolating position along an ellipse
// * @Deprecated: 1.6: Use clutter_actor_animate() instead
// *
// * #ClutterBehaviourEllipse interpolates actors along a path defined by
// *  an ellipse.
// *
// * When applying an ellipse behaviour to an actor, the
// * behaviour will update the actor's position and depth and set them
// * to what is dictated by the ellipses initial position.
// *
// * Deprecated: 1.6: Use clutter_actor_animate(), #ClutterPath and a
// *   #ClutterPathConstraint instead.
// @implements Scriptable @extends Behaviour
#[derive(Debug, Clone)]
pub struct BehaviourEllipse {
    center: Knot,

    /* a = width / 2 */
    a: u32,

    /* b = height / 2 */
    b: u32,

    angle_start: f64,
    angle_end: f64,

    angle_tilt_x: f64,
    angle_tilt_y: f64,
    angle_tilt_z: f64,

    direction: RotateDirection,
}

impl BehaviourEllipse {
    pub fn new<P: Is<Alpha>>(
        alpha: Option<&P>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        direction: RotateDirection,
        start: f64,
        end: f64,
    ) -> BehaviourEllipse {
        // Self {

        // }
        unimplemented!()
    }
}

impl Object for BehaviourEllipse {}

pub trait BehaviourEllipseExt: 'static {
    fn get_angle_end(&self) -> f64;

    fn get_angle_start(&self) -> f64;

    fn get_angle_tilt(&self, axis: RotateAxis) -> f64;

    fn get_center(&self) -> (i32, i32);

    fn get_direction(&self) -> RotateDirection;

    fn get_height(&self) -> i32;

    fn get_tilt(&self) -> (f64, f64, f64);

    fn get_width(&self) -> i32;

    fn set_angle_end(&self, angle_end: f64);

    fn set_angle_start(&self, angle_start: f64);

    fn set_angle_tilt(&self, axis: RotateAxis, angle_tilt: f64);

    fn set_center(&self, x: i32, y: i32);

    fn set_direction(&self, direction: RotateDirection);

    fn set_height(&self, height: i32);

    fn set_tilt(&self, angle_tilt_x: f64, angle_tilt_y: f64, angle_tilt_z: f64);

    fn set_width(&self, width: i32);

    fn get_property_angle_tilt_x(&self) -> f64;

    fn set_property_angle_tilt_x(&self, angle_tilt_x: f64);

    fn get_property_angle_tilt_y(&self) -> f64;

    fn set_property_angle_tilt_y(&self, angle_tilt_y: f64);

    fn get_property_angle_tilt_z(&self) -> f64;

    fn set_property_angle_tilt_z(&self, angle_tilt_z: f64);

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_angle_tilt_x_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_angle_tilt_y_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_angle_tilt_z_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_center_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<BehaviourEllipse>> BehaviourEllipseExt for O {
    fn get_angle_end(&self) -> f64 {
        let behaviour = self.as_ref();
        behaviour.angle_end
    }

    fn get_angle_start(&self) -> f64 {
        let behaviour = self.as_ref();
        behaviour.angle_start
    }

    fn get_angle_tilt(&self, axis: RotateAxis) -> f64 {
        let behaviour = self.as_ref();
        match axis {
            RotateAxis::XAxis => behaviour.angle_tilt_x,
            RotateAxis::YAxis => behaviour.angle_tilt_y,
            RotateAxis::ZAxis => behaviour.angle_tilt_z,
        }
    }

    fn get_center(&self) -> (i32, i32) {
        // let behaviour = self.as_ref();
        // (behaviour.center.x, behaviour.center.y)
        unimplemented!()
    }

    fn get_direction(&self) -> RotateDirection {
        let behaviour = self.as_ref();
        behaviour.direction
    }

    fn get_height(&self) -> i32 {
        // let behaviour = self.as_ref();
        // behaviour.height
        unimplemented!()
    }

    fn get_tilt(&self) -> (f64, f64, f64) {
        let behaviour = self.as_ref();
        (
            behaviour.angle_tilt_x,
            behaviour.angle_tilt_y,
            behaviour.angle_tilt_z,
        )
    }

    fn get_width(&self) -> i32 {
        // let behaviour = self.as_ref();
        // behaviour.width
        unimplemented!()
    }

    fn set_angle_end(&self, angle_end: f64) {
        unimplemented!()
    }

    fn set_angle_start(&self, angle_start: f64) {
        unimplemented!()
    }

    fn set_angle_tilt(&self, axis: RotateAxis, angle_tilt: f64) {
        unimplemented!()
    }

    fn set_center(&self, x: i32, y: i32) {
        unimplemented!()
    }

    fn set_direction(&self, direction: RotateDirection) {
        unimplemented!()
    }

    fn set_height(&self, height: i32) {
        unimplemented!()
    }

    fn set_tilt(&self, angle_tilt_x: f64, angle_tilt_y: f64, angle_tilt_z: f64) {
        unimplemented!()
    }

    fn set_width(&self, width: i32) {
        unimplemented!()
    }

    fn get_property_angle_tilt_x(&self) -> f64 {
        unimplemented!()
    }

    fn set_property_angle_tilt_x(&self, angle_tilt_x: f64) {
        unimplemented!()
    }

    fn get_property_angle_tilt_y(&self) -> f64 {
        unimplemented!()
    }

    fn set_property_angle_tilt_y(&self, angle_tilt_y: f64) {
        unimplemented!()
    }

    fn get_property_angle_tilt_z(&self) -> f64 {
        unimplemented!()
    }

    fn set_property_angle_tilt_z(&self, angle_tilt_z: f64) {
        unimplemented!()
    }

    fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_angle_tilt_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_angle_tilt_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_angle_tilt_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_center_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for BehaviourEllipse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BehaviourEllipse")
    }
}
