use super::{get_filler, Op, OpSet, OpSetType, Random, RoughOptions};
use crate::{OpType, Point};
use std::f64::consts::PI;
// import { RenderHelper } from './fillers/filler-interface.js';
// import { parsePath, normalize, absolutize } from 'path-data-parser';

pub struct EllipseResult {
    pub opset: OpSet,
    pub estimated_points: Vec<Point<f64>>,
}

pub struct EllipseParams {
    pub rx: f64,
    pub ry: f64,
    pub increment: f64,
}

pub struct Renderer;
// let helper: RenderHelper = {
//   randOffset,
//   randOffsetWithRange,
//   ellipse,
//   doubleLineOps: doubleLineFillOps
// };

impl Renderer {
    pub fn line(x1: f64, y1: f64, x2: f64, y2: f64, options: &RoughOptions) -> OpSet {
        OpSet {
            kind: OpSetType::Path,
            ops: Renderer::_double_line(x1, y1, x2, y2, options, false),
            size: None,
            path: None,
        }
    }

    pub fn linear_path(points: &Vec<Point<f64>>, close: bool, options: &RoughOptions) -> OpSet {
        let len = points.len();
        if len > 2 {
            let mut ops: Vec<Op> = Vec::new();
            for idx in 0..len - 1 {
                let first = points.get(idx).unwrap();
                let next = points.get(idx + 1).unwrap();
                let mut op2 =
                    Renderer::_double_line(first.x, first.y, next.x, next.y, options, false);
                ops.append(op2.as_mut());
            }

            if close {
                let first = points.first().unwrap();
                let last = points.last().unwrap();
                let mut op2 =
                    Renderer::_double_line(last.x, last.y, first.x, first.y, options, false);
                ops.append(op2.as_mut());
            }

            return OpSet {
                kind: OpSetType::Path,
                ops,
                size: None,
                path: None,
            };
        } else if len == 2 {
            let first = points.first().unwrap();
            let last = points.last().unwrap();
            return Renderer::line(first.x, first.y, last.x, last.y, options);
        }

        OpSet {
            kind: OpSetType::Path,
            ops: Vec::new(),
            size: None,
            path: None,
        }
    }

    pub fn polygon(points: &Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        Renderer::linear_path(points, true, options)
    }

    pub fn rectangle(x: f64, y: f64, width: f64, height: f64, options: &RoughOptions) -> OpSet {
        let points: Vec<Point<f64>> = vec![
            Point::new(x, y),
            Point::new(x + width, y),
            Point::new(x + width, y + height),
            Point::new(x, y + height),
        ];

        Renderer::polygon(&points, options)
    }

    pub fn curve(points: &Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        let mut o1 =
            Renderer::_curve_with_offset(points, 1.0 * (1.0 + options.roughness * 0.2), options);
        if !options.disable_multi_stroke {
            let mut o2 = Renderer::_curve_with_offset(
                points,
                1.5 * (1.0 + options.roughness * 0.22),
                options, //Renderer::clone_options_alter_seed(options),
            );
            o1.append(o2.as_mut());
        }

        OpSet {
            kind: OpSetType::Path,
            ops: o1,
            size: None,
            path: None,
        }
    }

    pub fn ellipse(x: f64, y: f64, width: f64, height: f64, options: &RoughOptions) -> OpSet {
        let params = Renderer::generate_ellipse_params(width, height, options);
        Renderer::ellipse_with_params(x, y, options, &params).opset
    }

    pub fn generate_ellipse_params(width: f64, height: f64, options: &RoughOptions) -> EllipseParams {
        let psq =
            (PI * 2.0 * (((width / 2.0).powi(2) + (height / 2.0).powi(2)) / 2.0).sqrt()).sqrt();
        let step_count = options.curve_step_count.max(options.curve_step_count / 200_f64.sqrt()) * psq;
        let increment = (PI * 2.0) / step_count;
        let mut rx = (width / 2.0).abs();
        let mut ry = (height / 2.0).abs();
        let curve_fit_randomness = 1.0 - options.curve_fitting;
        rx += Renderer::_offset_opt(rx * curve_fit_randomness, options, 1.0);
        ry += Renderer::_offset_opt(ry * curve_fit_randomness, options, 1.0);
        EllipseParams { increment, rx, ry }
    }

    pub fn ellipse_with_params(
        x: f64,
        y: f64,
        o: &RoughOptions,
        ellipse_params: &EllipseParams,
    ) -> EllipseResult {
        let (ap1, cp1) = Renderer::_compute_ellipse_points(
            ellipse_params.increment,
            x,
            y,
            ellipse_params.rx,
            ellipse_params.ry,
            1.0,
            ellipse_params.increment
                * Renderer::_offset(0.1, Renderer::_offset(0.4, 1.0, o, 1.0), o, 1.0),
            o,
        );

        let mut o1 = Renderer::_curve(ap1, None, o);
        if !o.disable_multi_stroke {
            let (ap2, _) = Renderer::_compute_ellipse_points(
                ellipse_params.increment,
                x,
                y,
                ellipse_params.rx,
                ellipse_params.ry,
                1.5,
                0.0,
                o,
            );
            let mut o2 = Renderer::_curve(ap2, None, o);
            o1.append(o2.as_mut());
        }

        EllipseResult {
            estimated_points: cp1,
            opset: OpSet {
                kind: OpSetType::Path,
                ops: o1,
                size: None,
                path: None,
            },
        }
    }

    pub fn arc(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        start: f64,
        stop: f64,
        closed: bool,
        rough_closure: bool,
        options: &RoughOptions,
    ) -> OpSet {
        let cx = x;
        let cy = y;
        let rx =
            (width / 2.0).abs() + Renderer::_offset_opt((width / 2.0).abs() * 0.01, options, 1.0);
        let ry =
            (height / 2.0).abs() + Renderer::_offset_opt((height / 2.0).abs() * 0.01, options, 1.0);
        let mut strt = start;
        let mut stp = stop;
        while strt < 0.0 {
            strt += PI * 2.0;
            stp += PI * 2.0;
        }

        if (stp - strt) > PI * 2.0 {
            strt = 0.0;
            stp = PI * 2.0;
        }

        let ellipse_inc = PI * 2.0 / options.curve_step_count;
        let arc_inc = (ellipse_inc / 2.0).min((stp - strt) / 2.0);
        let mut ops = Renderer::_arc(arc_inc, cx, cy, rx, ry, strt, stp, 1.0, options);

        if !options.disable_multi_stroke {
            // println!("RENDERER ARC DISABLE MULTI STROKE");
            let mut o2 = Renderer::_arc(arc_inc, cx, cy, rx, ry, strt, stp, 1.5, options);
            ops.append(o2.as_mut());
        }

        if closed {
            // println!("RENDERER ARC CLOSED");
            if rough_closure {
                let mut op2 = Renderer::_double_line(
                    cx,
                    cy,
                    cx + rx * strt.cos(),
                    cy + ry * strt.sin(),
                    options,
                    false,
                );
                ops.append(op2.as_mut());

                let mut op2 = Renderer::_double_line(
                    cx,
                    cy,
                    cx + rx * stp.cos(),
                    cy + ry * stp.sin(),
                    options,
                    false,
                );
                ops.append(op2.as_mut());
            } else {
                ops.push(Op {
                    op: OpType::LineTo,
                    data: vec![cx, cy],
                });
                ops.push(Op {
                    op: OpType::LineTo,
                    data: vec![cx + rx * strt.cos(), cy + ry * strt.sin()],
                });
            }
        }

        OpSet {
            kind: OpSetType::Path,
            ops,
            size: None,
            path: None,
        }
    }

    // pub fn svg_path(path: String, o: &RoughOptions) -> OpSet {
    //     let segments = normalize(absolutize(parsePath(path)));
    //     let ops: Op[] = [];
    //     let first: Point = [0, 0];
    //     let current: Point = [0, 0];
    //     for (let { key, data } of segments) {
    //       switch (key) {
    //         case 'M': {
    //           let ro = 1 * (o.maxRandomnessOffset || 0);
    //           ops.push({ op: 'move', data: data.map((d) => d + _offset_opt(ro, o)) });
    //           current = [data[0], data[1]];
    //           first = [data[0], data[1]];
    //           break;
    //         }
    //         case 'L':
    //           ops.push(..._doubleLine(current[0], current[1], data[0], data[1], o));
    //           current = [data[0], data[1]];
    //           break;
    //         case 'C': {
    //           let [x1, y1, x2, y2, x, y] = data;
    //           ops.push(..._bezierTo(x1, y1, x2, y2, x, y, current, o));
    //           current = [x, y];
    //           break;
    //         }
    //         case 'Z':
    //           ops.push(..._doubleLine(current[0], current[1], first[0], first[1], o));
    //           current = [first[0], first[1]];
    //           break;
    //       }
    //     }
    //     OpSet {
    //         kind: OpSetType::Path,
    //         ops,
    //         size: None,
    //         path: None,
    //     }
    // }

    // Fills
    pub fn solid_fill_polygon(points: &Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        let mut ops: Vec<Op> = Vec::new();
        let len = points.len();
        if len > 0 {
            let offset = options.max_randomness_offset;
            if len > 2 {
                let first = points.first().unwrap();
                ops.push(Op {
                    op: OpType::Move,
                    data: vec![
                        first.x + Renderer::_offset_opt(offset, options, 1.0),
                        first.y + Renderer::_offset_opt(offset, options, 1.0),
                    ],
                });

                for i in 1..len {
                    let pt = points.get(i).unwrap();
                    ops.push(Op {
                        op: OpType::LineTo,
                        data: vec![
                            pt.x + Renderer::_offset_opt(offset, options, 1.0),
                            pt.y + Renderer::_offset_opt(offset, options, 1.0),
                        ],
                    });
                }
            }
        }
        OpSet {
            kind: OpSetType::FillPath,
            ops,
            size: None,
            path: None,
        }
    }

    pub fn pattern_fill_polygon(points: &Vec<Point<f64>>, options: &RoughOptions) -> OpSet {
        // get_filler(o, helper).fill_polygon(points, o)
        unimplemented!()
    }

    pub fn pattern_fill_arc(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        start: f64,
        stop: f64,
        o: &RoughOptions,
    ) -> OpSet {
        let cx = x;
        let cy = y;
        let mut rx = (width / 2.0).abs();
        let mut ry = (height / 2.0).abs();
        rx += Renderer::_offset_opt(rx * 0.01, o, 1.0);
        ry += Renderer::_offset_opt(ry * 0.01, o, 1.0);
        let mut strt = start;
        let mut stp = stop;
        while strt < 0.0 {
            strt += PI * 2.0;
            stp += PI * 2.0;
        }

        if (stp - strt) > PI * 2.0 {
            strt = 0.0;
            stp = PI * 2.0;
        }

        let increment = (stp - strt) / o.curve_step_count;
        let mut points: Vec<Point<f64>> = Vec::new();

        let mut angle = strt;
        while angle <= stp {
            points.push(Point::new(cx + rx * angle.cos(), cy + ry * angle.sin()));
            angle = angle + increment
        }

        points.push(Point::new(cx + rx * stp.cos(), cy + ry * stp.sin()));
        points.push(Point::new(cx, cy));
        Renderer::pattern_fill_polygon(&points, o)
    }

    pub fn rand_offset(x: f64, o: &RoughOptions) -> f64 {
        Renderer::_offset_opt(x, o, 1.0)
    }

    pub fn rand_offset_with_range(min: f64, max: f64, o: &RoughOptions) -> f64 {
        Renderer::_offset(min, max, o, 1.0)
    }

    pub fn double_line_fill_ops(x1: f64, y1: f64, x2: f64, y2: f64, o: &RoughOptions) -> Vec<Op> {
        Renderer::_double_line(x1, y1, x2, y2, o, true)
    }

    fn randomize(ops: &RoughOptions) -> f64 {
        // if (!ops.randomizer) {
        //   ops.randomizer = new Random(ops.seed || 0);
        // }
        // ops.randomizer.next()
        rand::random()
    }

    fn _offset(min: f64, max: f64, ops: &RoughOptions, roughness_gain: f64) -> f64 {
        // roughness_gain: i32 = 1
        ops.roughness * roughness_gain * ((Renderer::randomize(ops) * (max - min)) + min)
    }

    fn _offset_opt(x: f64, ops: &RoughOptions, roughness_gain: f64) -> f64 {
        // roughness_gain: i32 = 1
        Renderer::_offset(-x, x, ops, roughness_gain)
    }

    fn _double_line(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        o: &RoughOptions,
        filling: bool,
    ) -> Vec<Op> {
        // // filling: bool = false
        let single_stroke = if filling {
            o.disable_multi_stroke_fill
        } else {
            o.disable_multi_stroke
        };

        let mut result = Renderer::_line(x1, y1, x2, y2, o, true, false);
        if !single_stroke {
            let mut second = Renderer::_line(x1, y1, x2, y2, o, true, true);
            result.append(second.as_mut());
        }

        result
    }

    fn _line(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        o: &RoughOptions,
        ismove: bool,
        overlay: bool,
    ) -> Vec<Op> {
        let length_sq = (x1 - x2).powi(2) + (y1 - y2).powi(2);
        let length = length_sq.sqrt();

        // println!("LINE >>> {}:{} {}:{} length {}", x1, y1, x2, y2, length);
        let roughness_gain = if length < 200.0 {
            1.0
        } else if length > 500.0 {
            0.4
        } else {
            -0.0016668 * length + 1.233334
        };

        let mut offset = o.max_randomness_offset;
        if (offset * offset * 100.0) > length_sq {
            offset = length / 10.0;
        }
        let half_offset = offset / 2.0;
        let diverge_point = 0.2 + Renderer::randomize(o) as f64 * 0.2;

        let mut mid_disp_x = o.bowing * o.max_randomness_offset * (y2 - y1) / 200.0;
        let mut mid_disp_y = o.bowing * o.max_randomness_offset * (x1 - x2) / 200.0;
        mid_disp_x = Renderer::_offset_opt(mid_disp_x, o, roughness_gain);
        mid_disp_y = Renderer::_offset_opt(mid_disp_y, o, roughness_gain);
        let mut ops: Vec<Op> = Vec::new();

        if ismove {
            if overlay {
                ops.push(Op {
                    op: OpType::Move,
                    data: vec![
                        x1 + Renderer::_offset_opt(half_offset, o, roughness_gain),
                        y1 + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    ],
                });
            } else {
                ops.push(Op {
                    op: OpType::Move,
                    data: vec![
                        x1 + Renderer::_offset_opt(offset, o, roughness_gain),
                        y1 + Renderer::_offset_opt(offset, o, roughness_gain),
                    ],
                });
            }
        }

        if overlay {
            ops.push(Op {
                op: OpType::BCurveTo,
                data: vec![
                    mid_disp_x
                        + x1
                        + (x2 - x1) * diverge_point
                        + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    mid_disp_y
                        + y1
                        + (y2 - y1) * diverge_point
                        + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    mid_disp_x
                        + x1
                        + 2.0 * (x2 - x1) * diverge_point
                        + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    mid_disp_y
                        + y1
                        + 2.0 * (y2 - y1) * diverge_point
                        + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    x2 + Renderer::_offset_opt(half_offset, o, roughness_gain),
                    y2 + Renderer::_offset_opt(half_offset, o, roughness_gain),
                ],
            });
        } else {
            ops.push(Op {
                op: OpType::BCurveTo,
                data: vec![
                    mid_disp_x
                        + x1
                        + (x2 - x1) * diverge_point
                        + Renderer::_offset_opt(offset, o, roughness_gain),
                    mid_disp_y
                        + y1
                        + (y2 - y1) * diverge_point
                        + Renderer::_offset_opt(offset, o, roughness_gain),
                    mid_disp_x
                        + x1
                        + 2.0 * (x2 - x1) * diverge_point
                        + Renderer::_offset_opt(offset, o, roughness_gain),
                    mid_disp_y
                        + y1
                        + 2.0 * (y2 - y1) * diverge_point
                        + Renderer::_offset_opt(offset, o, roughness_gain),
                    x2 + Renderer::_offset_opt(offset, o, roughness_gain),
                    y2 + Renderer::_offset_opt(offset, o, roughness_gain),
                ],
            });
        }
        ops
    }

    fn _curve_with_offset(
        points: &Vec<Point<f64>>,
        offset: f64,
        options: &RoughOptions,
    ) -> Vec<Op> {
        let mut ps: Vec<Point<f64>> = Vec::new();
        let mut iter = points.iter();

        let first = iter.next().unwrap();
        ps.push(Point::new(
            first.x + Renderer::_offset_opt(offset, options, 1.0),
            first.y + Renderer::_offset_opt(offset, options, 1.0),
        ));

        ps.push(Point::new(
            first.x + Renderer::_offset_opt(offset, options, 1.0),
            first.y + Renderer::_offset_opt(offset, options, 1.0),
        ));

        let mut idx: usize = 1;
        let last_idx = points.len() - 1;
        // from second item
        for point in iter {
            ps.push(Point::new(
                point.x + Renderer::_offset_opt(offset, options, 1.0),
                point.y + Renderer::_offset_opt(offset, options, 1.0),
            ));

            if idx == last_idx {
                ps.push(Point::new(
                    point.x + Renderer::_offset_opt(offset, options, 1.0),
                    point.y + Renderer::_offset_opt(offset, options, 1.0),
                ));
            }
            idx += 1;
        }

        Renderer::_curve(ps, None, options)
    }

    fn _curve(
        points: Vec<Point<f64>>,
        close_point: Option<Point<f64>>,
        o: &RoughOptions,
    ) -> Vec<Op> {
        let len = points.len();
        let mut ops: Vec<Op> = Vec::new();
        if len > 3 {
            let s = 1.0 - o.curve_tightness;
            let pt = points.get(1).unwrap();
            ops.push(Op {
                op: OpType::Move,
                data: vec![pt.x, pt.y],
            });

            let mut i: usize = 1;
            while (i + 2) < len {
                let prev = points.get(i - 1).unwrap();
                let current = points.get(i).unwrap();
                let next = points.get(i + 1).unwrap();
                let next_one = points.get(i + 2).unwrap();

                let b0 = Point::new(current.x, current.y);
                let b1 = Point::new(
                    current.x + (s * next.x - s * prev.x) / 6.0,
                    current.y + (s * next.y - s * prev.y) / 6.0,
                );
                let b2 = Point::new(
                    next.x + (s * current.x - s * next_one.x) / 6.0,
                    next.y + (s * current.y - s * next_one.y) / 6.0,
                );
                let b3 = Point::new(next.x, next.y);

                ops.push(Op {
                    op: OpType::BCurveTo,
                    data: vec![b1.x, b1.y, b2.x, b2.y, b3.x, b3.y],
                });
                i += 1;
            }
            if let Some(close_point) = close_point {
                let ro = o.max_randomness_offset;
                ops.push(Op {
                    op: OpType::LineTo,
                    data: vec![
                        close_point.x + Renderer::_offset_opt(ro, o, 1.0),
                        close_point.y + Renderer::_offset_opt(ro, o, 1.0),
                    ],
                });
            }
        } else if len == 3 {
            let mid = points.get(1).unwrap();
            ops.push(Op {
                op: OpType::Move,
                data: vec![mid.x, mid.y],
            });
            let last = points.last().unwrap();
            ops.push(Op {
                op: OpType::BCurveTo,
                data: vec![mid.x, mid.y, last.x, last.y, last.x, last.y],
            });
        } else if len == 2 {
            let first = points.get(0).unwrap();
            let second = points.get(1).unwrap();
            let mut append = Renderer::_double_line(first.x, first.y, second.x, second.y, o, false);
            ops.append(append.as_mut());
        }
        ops
    }

    fn _compute_ellipse_points(
        increment: f64,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        offset: f64,
        overlap: f64,
        o: &RoughOptions,
    ) -> (Vec<Point<f64>>, Vec<Point<f64>>) {
        let mut core_points: Vec<Point<f64>> = Vec::new();
        let mut all_points: Vec<Point<f64>> = Vec::new();

        let rad_offset = Renderer::_offset_opt(0.5, o, 1.0) - PI / 2.0;

        all_points.push(Point::new(
            Renderer::_offset_opt(offset, o, 1.0) + cx + 0.9 * rx * (rad_offset - increment).cos(),
            Renderer::_offset_opt(offset, o, 1.0) + cy + 0.9 * ry * (rad_offset - increment).sin(),
        ));

        let mut angle = rad_offset;
        while angle < PI * 2.0 + rad_offset - 0.01 {
            let p = Point::new(
                Renderer::_offset_opt(offset, o, 1.0) + cx + rx * angle.cos(),
                Renderer::_offset_opt(offset, o, 1.0) + cy + ry * angle.sin(),
            );
            core_points.push(p);
            all_points.push(p);

            angle = angle + increment;
        }

        all_points.push(Point::new(
            Renderer::_offset_opt(offset, o, 1.0)
                + cx
                + rx * (rad_offset + PI * 2.0 + overlap * 0.5).cos(),
            Renderer::_offset_opt(offset, o, 1.0)
                + cy
                + ry * (rad_offset + PI * 2.0 + overlap * 0.5).sin(),
        ));

        all_points.push(Point::new(
            Renderer::_offset_opt(offset, o, 1.0) + cx + 0.98 * rx * (rad_offset + overlap).cos(),
            Renderer::_offset_opt(offset, o, 1.0) + cy + 0.98 * ry * (rad_offset + overlap).sin(),
        ));

        all_points.push(Point::new(
            Renderer::_offset_opt(offset, o, 1.0)
                + cx
                + 0.9 * rx * (rad_offset + overlap * 0.5).cos(),
            Renderer::_offset_opt(offset, o, 1.0)
                + cy
                + 0.9 * ry * (rad_offset + overlap * 0.5).sin(),
        ));

        (all_points, core_points)
    }

    fn _arc(
        increment: f64,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        strt: f64,
        stp: f64,
        offset: f64,
        o: &RoughOptions,
    ) -> Vec<Op> {
        // println!("RENDERER _ARC");

        let rad_offset = strt + Renderer::_offset_opt(0.1, o, 1.0);
        let mut points: Vec<Point<f64>> = Vec::new();
        points.push(Point::new(
            Renderer::_offset_opt(offset, o, 1.0) + cx + 0.9 * rx * (rad_offset - increment).cos(),
            Renderer::_offset_opt(offset, o, 1.0) + cy + 0.9 * ry * (rad_offset - increment).sin(),
        ));

        let mut angle = rad_offset;
        while angle <= stp {
            points.push(Point::new(
                Renderer::_offset_opt(offset, o, 1.0) + cx + rx * angle.cos(),
                Renderer::_offset_opt(offset, o, 1.0) + cy + ry * angle.sin(),
            ));
            angle = angle + increment;
        }

        points.push(Point::new(cx + rx * stp.cos(), cy + ry * stp.sin()));
        points.push(Point::new(cx + rx * stp.cos(), cy + ry * stp.sin()));

        Renderer::_curve(points, None, o)
    }

    fn _bezier_to(
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x: f64,
        y: f64,
        current: Point<f64>,
        options: &RoughOptions,
    ) -> Vec<Op> {
        let mut ops: Vec<Op> = Vec::new();
        let ros = vec![
            if options.max_randomness_offset != 0.0 {
                options.max_randomness_offset
            } else {
                1.0
            },
            if options.max_randomness_offset != 0.0 {
                options.max_randomness_offset + 0.3
            } else {
                1.3
            },
        ];

        let iterations = if options.disable_multi_stroke { 1 } else { 2 };

        for i in 0..iterations {
            if i == 0 {
                ops.push(Op {
                    op: OpType::Move,
                    data: vec![current.x, current.y],
                });
            } else {
                ops.push(Op {
                    op: OpType::Move,
                    data: vec![
                        current.x + Renderer::_offset_opt(ros[0], options, 1.0),
                        current.y + Renderer::_offset_opt(ros[0], options, 1.0),
                    ],
                });
            }

            let fp = Point::new(
                x + Renderer::_offset_opt(ros[i], options, 1.0),
                y + Renderer::_offset_opt(ros[i], options, 1.0),
            );

            ops.push(Op {
                op: OpType::BCurveTo,
                data: vec![
                    x1 + Renderer::_offset_opt(ros[i], options, 1.0),
                    y1 + Renderer::_offset_opt(ros[i], options, 1.0),
                    x2 + Renderer::_offset_opt(ros[i], options, 1.0),
                    y2 + Renderer::_offset_opt(ros[i], options, 1.0),
                    fp.x,
                    fp.y,
                ],
            });
        }
        ops
    }
}
