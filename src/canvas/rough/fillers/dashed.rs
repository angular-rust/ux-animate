use super::{polygon_hachure_lines, PatternFiller, RenderHelper};
use crate::{
    canvas::rough::{geometry::line_length, Line, Op, OpSet, OpSetType, RoughOptions},
    Point,
};

pub struct DashedFiller<H: RenderHelper> {
    helper: H,
}

// implements PatternFiller
impl<H: RenderHelper> DashedFiller<H> {
    fn new(helper: H) -> Self {
        Self { helper }
    }

    fn fill_polygon(&self, points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        let lines = polygon_hachure_lines(&points, options);
        OpSet {
            kind: OpSetType::FillSketch,
            ops: self.dashed_line(lines, options),
            size: None,
            path: None,
        }
    }

    fn dashed_line(&self, lines: Vec<Line<f64>>, options: &RoughOptions) -> Vec<Op> {
        let offset = if options.dash_offset < 0.0 {
            if options.hachure_gap < 0.0 {
                options.stroke_width * 4.0
            } else {
                options.hachure_gap
            }
        } else {
            options.dash_offset
        };

        let gap = if options.dash_gap < 0.0 {
            if options.hachure_gap < 0.0 {
                options.stroke_width * 4.0
            } else {
                options.hachure_gap
            }
        } else {
            options.dash_gap
        };

        let mut ops: Vec<Op> = Vec::new();
        for line in lines.iter() {
            let length = line_length(line);
            let count = (length / (offset + gap)).floor() as usize;
            let start_offset = (length + gap - (count as f64 * (offset + gap))) / 2.0;
            let mut p1 = line.start;
            let mut p2 = line.end;
            if p1.x > p2.x {
                p1 = line.end;
                p2 = line.start;
            }
            let alpha = ((p2.y - p1.y) / (p2.x - p1.x)).atan();
            for i in 0..count {
                let lstart = i as f64 * (offset + gap);
                let lend = lstart + offset;
                let start: Point<f64> = Point::new(
                    p1.x + lstart * alpha.cos() + start_offset * alpha.cos(),
                    p1.y + lstart * alpha.sin() + start_offset * alpha.sin(),
                );
                let end: Point<f64> = Point::new(
                    p1.x + lend * alpha.cos() + start_offset * alpha.cos(),
                    p1.y + lend * alpha.sin() + start_offset * alpha.sin(),
                );
                let mut append = self.helper.double_line_ops(start.x, start.y, end.x, end.y, options);
                ops.append(append.as_mut());
            }
        }
        ops
    }
}
