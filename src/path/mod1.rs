use serde::ser::{Serialize, Serializer, SerializeStruct};

// "M15,15 L185,15 Z"
// "M15,15 l170,0 z"
// "M15,15L185,15Z"
// "M15 15 L 185 15 Z"
// "M15,15 L185,15 185,85 15,85 Z"
// "M15,15 L185,15 L185,85 L15,85 Z"
// "M15,15 L185,15 L185,85 L15,85 Z"
// "M15,15 l170,0 L185,85 l-170,0 z"
// "M15,15 H185 M185,15 V85 M185,85 H15 M15,85 V15"
// "M15,15 h170 M185,15 v70 M185,85 h-170 M15,85 v-70"
// "M100,10 L105.8,27.2 123.8,27.2 109.4,38 114.6,55.2 100,45 85.4,55.2 90.6,38 76.2,27.2 94.2,27.2"
// "M100,10 L121.6,22.5 121.6,47.5 100,60 78.4,47.52 78.4,22.5 Z"
// "M100,100 A50,25 0 1,0 150,125"
// "M100,100 A50,25 0 0,1 150,125"
// "M100,100 A50,25 0 0,0 150,125"
// "M100,100 A50,25 0 1,1 150,125"
// "M25,75 C25,15 175,15 175,75"
// "M25,75 c0,-60 150,-60 150,0"
// "M25,75 Q100,25 175,75"
// "M25,75 q75,-50 150,0 t 150,0"
// "M25,75 Q100,25 175,75 T 325,75 T475, 75 T 625,75"

pub enum Path {
    M{abs: bool, x: f64, y: f64}, // moveTo
    L{abs: bool, x: f64, y: f64}, // lineTo
    H{abs: bool, y: f64}, // horizontalLineTo
    V{abs: bool, x: f64}, // verticalLineTo
    A{abs: bool, rx: f64, ry: f64, x_axis_rotation: f64, large_arc_flag: f64, sweep_flag: f64, x: f64, y: f64,}, // elliptical arc
    C{abs: bool, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64}, // cubic bezier curve
    S{abs: bool, x2: f64, y2: f64, x: f64, y: f64}, // smooth cubic bezier curve
    Q{abs: bool, x1: f64, y1: f64, x: f64, y: f64}, // quadratic bezier curve
    T{abs: bool, x1: f64, y1: f64, x: f64, y: f64}, // smooth quadratic bezier curve
    Z{abs: bool}, // closePath
}

pub struct Polygon {
    pub commands: Vec<Path>,
}

impl Polygon {
    pub fn new(commands: Vec<Path>) -> Self {
        Self {
            commands
        }
    }
}