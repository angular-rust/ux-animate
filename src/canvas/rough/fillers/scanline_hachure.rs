use crate::{
    canvas::rough::{
        geometry::{rotate_lines, rotate_points},
        Line, RoughOptions,
    },
    Point,
};

pub struct EdgeEntry {
  ymin: f64,
  ymax: f64,
  x: f64,
  islope: f64,
}

pub struct ActiveEdgeEntry {
  s: f64,
  edge: EdgeEntry,
}

pub fn polygon_hachure_lines(points: &[Point<f64>], options: &RoughOptions) -> Vec<Line<f64>> {
    let rotation_center = Point::new(0.0, 0.0);
    let angle = (options.hachure_angle + 90.0).round();
    if angle != 0.0 {
      rotate_points(points, rotation_center, angle);
    }
    let lines = straight_hachure_lines(points, options);
    if angle != 0.0 {
      rotate_points(points, rotation_center, -angle);
      rotate_lines(&lines, rotation_center, -angle);
    }
    
    lines
}

fn straight_hachure_lines(points: &[Point<f64>], options: &RoughOptions) -> Vec<Line<f64>> {
    // let vertices = [...points];
    // if (vertices[0].join(',') !== vertices[vertices.length - 1].join(',')) {
    //   vertices.push([vertices[0][0], vertices[0][1]]);
    // }
    // let lines: Line[] = [];
    // if (vertices && vertices.length > 2) {
    //   let gap = o.hachureGap;
    //   if (gap < 0) {
    //     gap = o.strokeWidth * 4;
    //   }
    //   gap = Math.max(gap, 0.1);

    //   // Create sorted edges table
    //   let edges: EdgeEntry[] = [];
    //   for (let i = 0; i < vertices.length - 1; i++) {
    //     let p1 = vertices[i];
    //     let p2 = vertices[i + 1];
    //     if (p1[1] !== p2[1]) {
    //       let ymin = Math.min(p1[1], p2[1]);
    //       edges.push({
    //         ymin,
    //         ymax: Math.max(p1[1], p2[1]),
    //         x: ymin === p1[1] ? p1[0] : p2[0],
    //         islope: (p2[0] - p1[0]) / (p2[1] - p1[1])
    //       });
    //     }
    //   }
    //   edges.sort((e1, e2) => {
    //     if (e1.ymin < e2.ymin) {
    //       return -1;
    //     }
    //     if (e1.ymin > e2.ymin) {
    //       return 1;
    //     }
    //     if (e1.x < e2.x) {
    //       return -1;
    //     }
    //     if (e1.x > e2.x) {
    //       return 1;
    //     }
    //     if (e1.ymax === e2.ymax) {
    //       return 0;
    //     }
    //     return (e1.ymax - e2.ymax) / Math.abs((e1.ymax - e2.ymax));
    //   });
    //   if (!edges.length) {
    //     return lines;
    //   }

    //   // Start scanning
    //   let activeEdges: ActiveEdgeEntry[] = [];
    //   let y = edges[0].ymin;
    //   while (activeEdges.length || edges.length) {
    //     if (edges.length) {
    //       let ix = -1;
    //       for (let i = 0; i < edges.length; i++) {
    //         if (edges[i].ymin > y) {
    //           break;
    //         }
    //         ix = i;
    //       }
    //       let removed = edges.splice(0, ix + 1);
    //       removed.forEach((edge) => {
    //         activeEdges.push({ s: y, edge });
    //       });
    //     }
    //     activeEdges = activeEdges.filter((ae) => {
    //       if (ae.edge.ymax <= y) {
    //         return false;
    //       }
    //       return true;
    //     });
    //     activeEdges.sort((ae1, ae2) => {
    //       if (ae1.edge.x === ae2.edge.x) {
    //         return 0;
    //       }
    //       return (ae1.edge.x - ae2.edge.x) / Math.abs((ae1.edge.x - ae2.edge.x));
    //     });

    //     // fill between the edges
    //     if (activeEdges.length > 1) {
    //       for (let i = 0; i < activeEdges.length; i = i + 2) {
    //         let nexti = i + 1;
    //         if (nexti >= activeEdges.length) {
    //           break;
    //         }
    //         let ce = activeEdges[i].edge;
    //         let ne = activeEdges[nexti].edge;
    //         lines.push([
    //           [Math.round(ce.x), y],
    //           [Math.round(ne.x), y]
    //         ]);
    //       }
    //     }

    //     y += gap;
    //     activeEdges.forEach((ae) => {
    //       ae.edge.x = ae.edge.x + (gap * ae.edge.islope);
    //     });
    //   }
    // }
    // return lines;
    unimplemented!()
}
