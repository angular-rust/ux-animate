use super::polygon_hachure_lines;
use crate::{
    canvas::rough::{
        geometry::line_length, Line, Op, OpSet, OpSetType, RenderHelper, RoughOptions,
    },
    Point,
};
// PatternFiller

pub struct DotFiller<H: RenderHelper> {
    helper: H,
}

// implements PatternFiller
impl<H: RenderHelper> DotFiller<H> {
    pub fn new(helper: H) -> Self {
        Self { helper }
    }

    fn fill_polygon(&self, points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        // o = Object.assign({}, o, { curveStepCount: 4, hachureAngle: 0, roughness: 1 });
        let lines = polygon_hachure_lines(&points, options);
        self.dots_on_lines(lines, options)
    }

    fn dots_on_lines(&self, lines: Vec<Line<f64>>, options: &RoughOptions) -> OpSet {
        let ops: Vec<Op> = Vec::new();

        let gap = if options.hachure_gap < 0.0 {
            (options.stroke_width * 4.0).max(0.1)
        } else {
            options.hachure_gap.max(0.1)
        };

        let mut fweight = options.fill_weight;
        if fweight < 0.0 {
            fweight = options.stroke_width / 2.0;
        }
        let ro = gap / 4.0;

        for line in lines.iter() {
            let length = line_length(line);
            let dl = length / gap;
            let count = (dl.ceil() - 1.0) as usize;
            let offset = length - (count as f64 * gap);
            let x = ((line.start.x + line.end.x) / 2.0) - (gap / 4.0);
            let min_y = line.start.y.min(line.end.y);

            for i in 0..count {
                let y = min_y + offset + (i as f64 * gap);
                let cx = self.helper.rand_offset_with_range(x - ro, x + ro, options);
                let cy = self.helper.rand_offset_with_range(y - ro, y + ro, options);
                let el = self.helper.ellipse(cx, cy, fweight, fweight, options);
                //     ops.push(...el.ops);
            }
        }

        OpSet {
            kind: OpSetType::FillSketch,
            ops,
            size: None,
            path: None,
        }
    }
}
