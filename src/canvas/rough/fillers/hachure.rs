#![allow(clippy::if_same_then_else)]
use super::polygon_hachure_lines;
use crate::{
    canvas::rough::{
        geometry::{do_intersect, is_point_in_polygon, line_intersection, line_length},
        Line, Op, OpSet, OpSetType, PatternFiller, RenderHelper, RoughOptions,
    },
    Point,
};

pub struct IntersectionInfo {
    point: Point<f64>,
    distance: f64,
}

pub struct HachureFiller<H: RenderHelper> {
    helper: H,
}

// implements PatternFiller
impl<H: RenderHelper> HachureFiller<H> {
    pub fn new(helper: H) -> Self {
        Self { helper }
    }

    fn fill_polygon(&self, points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        self._fill_polygon(points, options, false)
    }

    fn _fill_polygon(
        &self,
        points: Vec<Point<f64>>,
        options: &RoughOptions,
        connect_ends: bool,
    ) -> OpSet {
        // connect_ends: boolean = false
        let mut lines = polygon_hachure_lines(&points, options);
        if connect_ends {
            let mut connecting_lines = self.connecting_lines(&points, &lines);
            lines.append(connecting_lines.as_mut());
        }
        let ops = self.render_lines(&lines, options);
        OpSet {
            kind: OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }

    fn render_lines(&self, lines: &[Line<f64>], options: &RoughOptions) -> Vec<Op> {
        let mut ops: Vec<Op> = Vec::new();
        for line in lines.iter() {
            let mut append =
                self.helper
                    .double_line_ops(line.start.x, line.start.y, line.end.x, line.end.y, options);
            ops.append(append.as_mut());
        }
        ops
    }

    fn connecting_lines(
        &self,
        polygon: &[Point<f64>],
        lines: &[Line<f64>],
    ) -> Vec<Line<f64>> {
        let result: Vec<Line<f64>> = Vec::new();
        if lines.len() > 1 {
            for i in 1..lines.len() {
                let prev = lines.get(i - 1).unwrap();
                if line_length(prev) < 3.0 {
                    continue;
                }
                let current = lines.get(i);
                // let segment: Line = [current[0], prev[1]];
                // if (line_length(segment) > 3) {
                //     let segSplits = self.splitOnIntersections(polygon, segment);
                //     result.push(...segSplits);
                // }
            }
        }
        result
    }

    fn mid_point_in_polygon(polygon: &[Point<f64>], segment: Line<f64>) -> bool {
        is_point_in_polygon(
            polygon,
            (segment.start.x + segment.end.x) / 2.0,
            (segment.start.y + segment.end.y) / 2.0,
        )
    }

    fn split_on_intersections(polygon: &[Point<f64>], segment: &Line<f64>) -> Vec<Line<f64>> {
        let error = 5_f64.max(line_length(segment) * 0.1);
        let intersections: Vec<IntersectionInfo> = Vec::new();
        for i in 0..polygon.len() {
            let p1 = polygon.get(i).unwrap();
            let p2 = polygon.get((i + 1) % polygon.len()).unwrap();
            //   if (doIntersect(p1, p2, ...segment)) {
            //     let ip = lineIntersection(p1, p2, segment[0], segment[1]);
            //     if (ip) {
            //       let d0 = line_length([ip, segment[0]]);
            //       let d1 = line_length([ip, segment[1]]);
            //       if (d0 > error && d1 > error) {
            //         intersections.push({
            //           point: ip,
            //           distance: d0
            //         });
            //       }
            //     }
            //   }
        }
        if intersections.len() > 1 {
            //   let ips = intersections.sort((a, b) => a.distance - b.distance).map<Point>((d) => d.point);
            //   if (!isPointInPolygon(polygon, ...segment[0])) {
            //     ips.shift();
            //   }
            //   if (!isPointInPolygon(polygon, ...segment[1])) {
            //     ips.pop();
            //   }
            //   if (ips.length <= 1) {
            //     if (self.midPointInPolygon(polygon, segment)) {
            //       return [segment];
            //     } else {
            //       return [];
            //     }
            //   }
            //   let spoints = [segment[0], ...ips, segment[1]];
            //   let slines: Line[] = [];
            //   for (let i = 0; i < (spoints.length - 1); i += 2) {
            //     let subSegment: Line = [spoints[i], spoints[i + 1]];
            //     if (self.midPointInPolygon(polygon, subSegment)) {
            //       slines.push(subSegment);
            //     }
            //   }
            //   return slines;
            // } else if (self.midPointInPolygon(polygon, segment)) {
            //   return [segment];
        } else {
            //   return [];
        }
        unimplemented!()
    }
}
