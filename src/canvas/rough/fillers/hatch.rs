use crate::{Point, canvas::rough::{RoughOptions, OpSet}};
use super::HachureFiller;

//extends HachureFiller
pub struct HatchFiller {

}

impl HatchFiller  {
  fn fill_polygon(points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
    // let set = self._fill_polygon(points, o);
    // let o2 = Object.assign({}, o, { hachureAngle: o.hachureAngle + 90 });
    // let set2 = self._fill_polygon(points, o2);
    // set.ops = set.ops.concat(set2.ops);
    // return set;
    unimplemented!()
  }
}