use super::Line;
use crate::{Point, Rectangle};
use std::f64::consts::PI;

pub fn rotate_points(points: &[Point<f64>], center: Point<f64>, degrees: f64) {
    if !points.is_empty() {
        let angle = (PI / 180.0) * degrees;
        let cos = angle.cos();
        let sin = angle.sin();
        //   points.forEach((p) => {
        //     let [x, y] = p;
        //     p.x = ((x - cx) * cos) - ((y - cy) * sin) + cx;
        //     p.y = ((x - cx) * sin) + ((y - cy) * cos) + cy;
        //   });
    }
}

pub fn rotate_lines(lines: &[Line<f64>], center: Point<f64>, degrees: f64) {
    let points: Vec<Point<f64>> = Vec::new();
    // lines.forEach((line) => points.push(...line));
    // rotate_points(points, center, degrees);
    unimplemented!()
}

pub fn line_length(line: &Line<f64>) -> f64 {
    let p1 = line.start;
    let p2 = line.end;
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn line_intersection(
    a: Point<f64>,
    b: Point<f64>,
    c: Point<f64>,
    d: Point<f64>,
) -> Option<Point<f64>> {
    let a1 = b.y - a.y;
    let b1 = a.x - b.x;
    let c1 = a1 * (a.x) + b1 * (a.y);
    let a2 = d.y - c.y;
    let b2 = c.x - d.x;
    let c2 = a2 * (c.x) + b2 * (c.y);
    let determinant = a1 * b2 - a2 * b1;
    if determinant != 0.0 {
        return Some(Point::new(
            (b2 * c1 - b1 * c2) / determinant,
            (a1 * c2 - a2 * c1) / determinant,
        ));
    }
    None
}

pub fn is_point_in_polygon(points: &[Point<f64>], x: f64, y: f64) -> bool {
    let vertices = points.len();

    // There must be at least 3 vertices in polygon
    if vertices < 3 {
        return false;
    }

    // let extreme: Point = [Number.MAX_SAFE_INTEGER, y];
    // let p: Point = [x, y];
    // let count = 0;
    // for i in 0..vertices {
    //   let current = points[i];
    //   let next = points[(i + 1) % vertices];
    //   if (doIntersect(current, next, p, extreme)) {
    //     if (orientation(current, p, next) == 0) {
    //       return onSegment(current, p, next);
    //     }
    //     count += 1;
    //   }
    // }
    // // true if count is off
    // return count % 2 == 1;
    unimplemented!()
}

// Check if q lies on the line segment pr
fn on_segment(p: Point<f64>, q: Point<f64>, r: Point<f64>) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

// For the ordered points p, q, r, return
// 0 if p, q, r are collinear
// 1 if Clockwise
// 2 if counterclickwise
fn orientation(p: Point<f64>, q: Point<f64>, r: Point<f64>) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0.0 {
        return 0;
    }

    if val > 0.0 {
        1
    } else {
        2
    }
}

// Check is p1q1 intersects with p2q2
pub fn do_intersect(p1: Point<f64>, q1: Point<f64>, p2: Point<f64>, q2: Point<f64>) -> bool {
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    // p1, q1 and p2 are colinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and p2 are colinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    false
}
