#![allow(unused_variables)]
#![allow(dead_code)]
use super::{
    random_seed, Drawable, DrawableType, OpSet, OpType, PathInfo, Renderer, RoughConfig,
    RoughOptions,
};
use crate::{OpSetType, Point};

// import { line, solidFillPolygon, patternFillPolygon, rectangle, ellipseWithParams, generateEllipseParams, linearPath, arc, patternFillArc, curve, svgPath } from './renderer.js';
// import { curveToBezier } from 'points-on-curve/lib/curve-to-bezier.js';
// import { pointsOnBezierCurves } from 'points-on-curve';
// import { pointsOnPath } from 'points-on-path';

pub const NOS: &str = "none";

pub struct RoughGenerator {
    pub config: RoughConfig,
    pub default_options: RoughOptions,
}

impl RoughGenerator {
    pub fn new(config: RoughConfig) -> Self {
        let ret = Self {
            config,
            default_options: Default::default(),
        };

        ret
    }

    pub fn new_seed(&self) -> u64 {
        random_seed()
    }

    fn _d(&self, shape: &str, sets: Vec<OpSet>, options: &RoughOptions) -> Drawable {
        // { shape, sets: sets || [], options: options || self.defaultOptions };
        unimplemented!()
    }

    pub fn line<'a>(
        &self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        let sets = vec![Renderer::line(x1, y1, x2, y2, options)];
        Drawable {
            shape: DrawableType::Line,
            sets,
            options,
        }
    }

    pub fn rectangle<'a>(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        let mut sets = Vec::new();
        let outline = Renderer::rectangle(x, y, width, height, options);
        if !options.fill.is_empty() {
            let points: Vec<Point<f64>> = vec![
                Point::new(x, y),
                Point::new(x + width, y),
                Point::new(x + width, y + height),
                Point::new(x, y + height),
            ];
            if options.fill_style == "solid" {
                sets.push(Renderer::solid_fill_polygon(&points, options));
            } else {
                sets.push(Renderer::pattern_fill_polygon(&points, options));
            }
        }

        if options.stroke != NOS {
            sets.push(outline);
        }

        Drawable {
            shape: DrawableType::Rectangle,
            sets,
            options,
        }
    }

    pub fn ellipse<'a>(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        let mut sets: Vec<OpSet> = Vec::new();
        let ellipse_params = Renderer::generate_ellipse_params(width, height, options);
        let ellipse_response = Renderer::ellipse_with_params(x, y, options, &ellipse_params);
        if !options.fill.is_empty() {
            if options.fill_style == "solid" {
                let mut shape = Renderer::ellipse_with_params(x, y, options, &ellipse_params).opset;
                shape.kind = OpSetType::FillPath;
                sets.push(shape);
            } else {
                sets.push(Renderer::pattern_fill_polygon(
                    &ellipse_response.estimated_points,
                    options,
                ));
            }
        }

        if options.stroke != NOS {
            sets.push(ellipse_response.opset);
        }

        Drawable {
            shape: DrawableType::Ellipse,
            sets,
            options,
        }
    }

    pub fn circle<'a>(
        &self,
        x: f64,
        y: f64,
        diameter: f64,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        let mut result = self.ellipse(x, y, diameter, diameter, options);
        result.shape = DrawableType::Circle;
        result
    }

    pub fn linear_path<'a>(
        &self,
        points: &Vec<Point<f64>>,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        let sets = vec![Renderer::linear_path(points, false, options)];
        Drawable {
            shape: DrawableType::LinearPath,
            sets,
            options,
        }
    }

    pub fn arc<'a>(
        &self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        start: f64,
        stop: f64,
        closed: bool,
        options: &'a RoughOptions,
    ) -> Drawable<'a> {
        // // closed: bool = false
        let mut sets: Vec<OpSet> = Vec::new();
        let outline = Renderer::arc(x, y, width, height, start, stop, closed, true, options);
        if closed && !options.fill.is_empty() {
            // println!("CLOSED OR FILL");
            if options.fill_style == "solid" {
                let mut shape =
                    Renderer::arc(x, y, width, height, start, stop, true, false, options);
                shape.kind = OpSetType::FillPath;
                sets.push(shape);
            } else {
                sets.push(Renderer::pattern_fill_arc(
                    x, y, width, height, start, stop, options,
                ));
            }
        }

        if options.stroke != NOS {
            // println!("SHOULD STROKE");
            sets.push(outline);
        }

        Drawable {
            shape: DrawableType::Arc,
            sets,
            options,
        }
    }

    pub fn curve<'a>(&self, points: &Vec<Point<f64>>, options: &'a RoughOptions) -> Drawable<'a> {
        let mut sets: Vec<OpSet> = Vec::new();
        let outline = Renderer::curve(points, options);
        if (!options.fill.is_empty()) && options.fill != NOS && points.len() >= 3 {
            // let bcurve = curveToBezier(points);
            // let polyPoints = pointsOnBezierCurves(bcurve, 10, (1.0 + options.roughness) / 2);
            // if options.fill_style == "solid" {
            //     sets.push(Renderer::solid_fill_polygon(polyPoints, options));
            // } else {
            //     sets.push(Renderer::pattern_fill_polygon(polyPoints, options));
            // }
            // TODO: Extdep
        }

        if options.stroke != NOS {
            sets.push(outline);
        }

        Drawable {
            shape: DrawableType::Curve,
            sets,
            options,
        }
    }

    pub fn polygon<'a>(&self, points: &Vec<Point<f64>>, options: &'a RoughOptions) -> Drawable<'a> {
        let mut sets: Vec<OpSet> = Vec::new();
        let outline = Renderer::linear_path(points, true, options);

        if !options.fill.is_empty() {
            if options.fill_style == "solid" {
                sets.push(Renderer::solid_fill_polygon(points, options));
            } else {
                sets.push(Renderer::pattern_fill_polygon(points, options));
            }
        }

        if options.stroke != NOS {
            sets.push(outline);
        }

        Drawable {
            shape: DrawableType::Polygon,
            sets,
            options,
        }
    }

    pub fn path<'a>(&self, d: &str, options: &'a RoughOptions) -> Drawable<'a> {
        let paths: Vec<OpSet> = Vec::new();
        if d.is_empty() {
            return Drawable {
                shape: DrawableType::Path,
                sets: paths,
                options,
            };
        }

        // let d = d.replace(/\n/g, ' ').replace(/(-\s)/g, '-').replace('/(\s\s)/g', ' ');
        
        let has_fill = options.fill.is_empty() && options.fill != "transparent" && options.fill != NOS;
        let has_stroke = options.stroke != NOS;
        
        let simplified = options.simplification != 0.0 && options.simplification < 1.0;
        let distance = if simplified {
          4.0 - 4.0 * options.simplification
        } else {
          (1.0 + options.roughness) / 2.0
        };

        // let sets = pointsOnPath(d, 1, distance);

        // if hasFill {
        //   if options.combine_nested_svg_paths {
        //     let combined: Vec<Point> = Vec::new();
        //     sets.forEach((set) => combined.push(...set));
        //     if options.fill_style == "solid" {
        //       paths.push(Renderer::solid_fill_polygon(combined, options));
        //     } else {
        //       paths.push(Renderer::pattern_fill_polygon(combined, options));
        //     }
        //   } else {
        //     sets.forEach((polyPoints) => {
        //       if options.fill_style == "solid" {
        //         paths.push(Renderer::solid_fill_polygon(polyPoints, options));
        //       } else {
        //         paths.push(Renderer::pattern_fill_polygon(polyPoints, options));
        //       }
        //     });
        //   }
        // }

        // if hasStroke {
        //   if simplified {
        //     sets.forEach((set) => {
        //       paths.push(linearPath(set, false, options));
        //     });
        //   } else {
        //     paths.push(svgPath(d, options));
        //   }
        // }

        Drawable {
            shape: DrawableType::Path,
            sets: paths,
            options,
        }
    }

    pub fn ops_to_path(&self, drawing: &OpSet) -> String {
        let mut path = "".to_owned();
        for item in drawing.ops.iter() {
            let data = &item.data;
            match item.op {
                OpType::Move => {
                    path.push_str(format!("M{} {} ", data[0], data[1]).as_str());
                }
                OpType::BCurveTo => {
                    path.push_str(
                        format!(
                            "C{} {}, {} {}, {} {} ",
                            data[0], data[1], data[2], data[3], data[4], data[5]
                        )
                        .as_str(),
                    );
                }
                OpType::LineTo => {
                    path.push_str(format!("L{} {} ", data[0], data[1]).as_str());
                }
            }
        }
        path.trim().into()
    }

    pub fn to_paths(&self, drawable: Drawable) -> Vec<PathInfo> {
        let sets = drawable.sets;
        let options = drawable.options;
        let mut paths: Vec<PathInfo> = Vec::new();
        for drawing in sets.iter() {
            let path: PathInfo;
            let path = match drawing.kind {
                OpSetType::Path => PathInfo {
                    d: self.ops_to_path(drawing),
                    stroke: options.stroke.clone(),
                    stroke_width: options.stroke_width,
                    fill: Some(NOS.into()),
                },
                OpSetType::FillPath => {
                    let fill = if options.fill.is_empty() {
                        NOS.into()
                    } else {
                        options.fill.clone()
                    };
                    PathInfo {
                        d: self.ops_to_path(drawing),
                        stroke: NOS.into(),
                        stroke_width: 0.0,
                        fill: Some(fill),
                    }
                }
                OpSetType::FillSketch => self.fill_sketch(drawing, options),
            };

            paths.push(path);
        }

        paths
    }

    pub fn fill_sketch(&self, drawing: &OpSet, options: &RoughOptions) -> PathInfo {
        let fweight = if options.fill_weight < 0.0 {
            options.stroke_width / 2.0
        } else {
            options.fill_weight
        };

        let stroke = if options.fill.is_empty() {
            NOS.into()
        } else {
            options.fill.clone()
        };
        PathInfo {
            d: self.ops_to_path(drawing),
            stroke,
            stroke_width: fweight,
            fill: Some(NOS.into()),
        }
    }
}
