// use super::RenderHelper;
use crate::{
  canvas::rough::{OpSet, RoughOptions},
  Point,
};

// geometry::line_length, Line, Op,

//extends HachureFiller
pub struct ZigZagFiller;

impl ZigZagFiller {
  fn fill_polygon(&self, points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
    // self._fillPolygon(points, o, true)
    unimplemented!()
  }
}