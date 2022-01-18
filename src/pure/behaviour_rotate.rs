// use super::{Alpha, HandlerId, RotateAxis, RotateDirection};
// use crate::prelude::*;
// use std::fmt;

// // @short_description: A behaviour controlling rotation
// //
// // A #BehaviourRotate rotate actors between a starting and ending
// // angle on a given axis.
// //
// // The #BehaviourRotate is available since version 0.4.
// //
// // Deprecated: 1.6: Use the #Actor rotation properties and
// //   actor_animate(), or #Animator, or #State
// //   instead.
// #[derive(Debug, Clone)]
// pub struct BehaviourRotate {
//     angle_start: f64,
//     angle_end: f64,

//     axis: RotateAxis,
//     direction: RotateDirection,

//     center_x: i32,
//     center_y: i32,
//     center_z: i32,
// }

// impl BehaviourRotate {
//     pub fn new<P: Is<Alpha>>(
//         alpha: Option<&P>,
//         axis: RotateAxis,
//         direction: RotateDirection,
//         angle_start: f64,
//         angle_end: f64,
//     ) -> BehaviourRotate {
//         unimplemented!()
//     }
// }

// impl Object for BehaviourRotate {}

// pub trait BehaviourRotateExt: 'static {
//     fn get_axis(&self) -> RotateAxis;

//     fn get_bounds(&self) -> (f64, f64);

//     fn get_center(&self) -> (i32, i32, i32);

//     fn get_direction(&self) -> RotateDirection;

//     fn set_axis(&self, axis: RotateAxis);

//     fn set_bounds(&self, angle_start: f64, angle_end: f64);

//     fn set_center(&self, x: i32, y: i32, z: i32);

//     fn set_direction(&self, direction: RotateDirection);

//     fn get_property_angle_end(&self) -> f64;

//     fn set_property_angle_end(&self, angle_end: f64);

//     fn get_property_angle_start(&self) -> f64;

//     fn set_property_angle_start(&self, angle_start: f64);

//     fn get_property_center_x(&self) -> i32;

//     fn set_property_center_x(&self, center_x: i32);

//     fn get_property_center_y(&self) -> i32;

//     fn set_property_center_y(&self, center_y: i32);

//     fn get_property_center_z(&self) -> i32;

//     fn set_property_center_z(&self, center_z: i32);

//     fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_center_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_center_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_center_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

//     fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
// }

// impl<O: Is<BehaviourRotate>> BehaviourRotateExt for O {
//     fn get_axis(&self) -> RotateAxis {
//         let behaviour = self.as_ref();
//         behaviour.axis
//     }

//     fn get_bounds(&self) -> (f64, f64) {
//         let behaviour = self.as_ref();
//         (behaviour.angle_start, behaviour.angle_end)
//     }

//     fn get_center(&self) -> (i32, i32, i32) {
//         // let behaviour = self.as_ref();
//         // (behaviour.x, behaviour.y, behaviour.z)
//         unimplemented!()
//     }

//     fn get_direction(&self) -> RotateDirection {
//         let behaviour = self.as_ref();
//         behaviour.direction
//     }

//     fn set_axis(&self, axis: RotateAxis) {
//         unimplemented!()
//     }

//     fn set_bounds(&self, angle_start: f64, angle_end: f64) {
//         unimplemented!()
//     }

//     fn set_center(&self, x: i32, y: i32, z: i32) {
//         unimplemented!()
//     }

//     fn set_direction(&self, direction: RotateDirection) {
//         unimplemented!()
//     }

//     fn get_property_angle_end(&self) -> f64 {
//         let behaviour = self.as_ref();
//         behaviour.angle_end
//     }

//     fn set_property_angle_end(&self, angle_end: f64) {
//         unimplemented!()
//     }

//     fn get_property_angle_start(&self) -> f64 {
//         let behaviour = self.as_ref();
//         behaviour.angle_start
//     }

//     fn set_property_angle_start(&self, angle_start: f64) {
//         unimplemented!()
//     }

//     fn get_property_center_x(&self) -> i32 {
//         let behaviour = self.as_ref();
//         behaviour.center_x
//     }

//     fn set_property_center_x(&self, center_x: i32) {
//         unimplemented!()
//     }

//     fn get_property_center_y(&self) -> i32 {
//         let behaviour = self.as_ref();
//         behaviour.center_y
//     }

//     fn set_property_center_y(&self, center_y: i32) {
//         unimplemented!()
//     }

//     fn get_property_center_z(&self) -> i32 {
//         let behaviour = self.as_ref();
//         behaviour.center_z
//     }

//     fn set_property_center_z(&self, center_z: i32) {
//         unimplemented!()
//     }

//     fn connect_property_angle_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_angle_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_center_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_center_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_center_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }

//     fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
//         unimplemented!()
//     }
// }

// impl fmt::Display for BehaviourRotate {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "BehaviourRotate")
//     }
// }
