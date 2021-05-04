use super::RenderHelper;
use crate::{
  canvas::rough::{geometry::line_length, Line, Op, OpSet, RoughOptions},
  Point,
};

//implements PatternFiller
pub struct ZigZagLineFiller<H: RenderHelper> {
  helper: H
}

impl<H: RenderHelper> ZigZagLineFiller<H> {
  pub fn new(helper: H) -> Self {
    Self { helper }
  }

  fn fill_polygon(points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
    // let gap = o.hachureGap < 0 ? (o.strokeWidth * 4) : o.hachureGap;
    // let zo = o.zigzagOffset < 0 ? gap : o.zigzagOffset;
    // o = Object.assign({}, o, { hachureGap: gap + zo });
    // let lines = polygonHachureLines(points, o);
    // return { type: 'fillSketch', ops: self.zigzagLines(lines, zo, o) };
    unimplemented!()
  }

  fn zigzag_lines(lines: Vec<Line<f64>>, zo: f64, options: &RoughOptions) -> Vec<Op> {
    // let ops: Op[] = [];
    // lines.forEach((line) => {
    //   let length = line_length(line);
    //   let count = Math.round(length / (2 * zo));
    //   let p1 = line[0];
    //   let p2 = line[1];
    //   if (p1[0] > p2[0]) {
    //     p1 = line[1];
    //     p2 = line[0];
    //   }
    //   let alpha = Math.atan((p2[1] - p1[1]) / (p2[0] - p1[0]));
    //   for (let i = 0; i < count; i++) {
    //     let lstart = i * 2 * zo;
    //     let lend = (i + 1) * 2 * zo;
    //     let dz = Math.sqrt(2 * Math.pow(zo, 2));
    //     let start: Point = [p1[0] + (lstart * Math.cos(alpha)), p1[1] + lstart * Math.sin(alpha)];
    //     let end: Point = [p1[0] + (lend * Math.cos(alpha)), p1[1] + (lend * Math.sin(alpha))];
    //     let middle: Point = [start[0] + dz * Math.cos(alpha + Math.PI / 4), start[1] + dz * Math.sin(alpha + Math.PI / 4)];
    //     ops.push(
    //       ...self.helper.doubleLineOps(start[0], start[1], middle[0], middle[1], o),
    //       ...self.helper.doubleLineOps(middle[0], middle[1], end[0], end[1], o)
    //     );
    //   }
    // });
    // return ops;
    unimplemented!()
  }
}