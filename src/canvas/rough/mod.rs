#![allow(dead_code)]
#![allow(unused_variables)]
use crate::Point;

mod canvas;
mod fillers;
mod generator;
pub(crate) mod geometry;
mod math;
mod renderer;
mod svg;

pub use canvas::RoughCanvas;
pub use fillers::*;
pub use generator::RoughGenerator;
pub use math::*;
pub use renderer::*;
pub use svg::*;

pub const SVGNS: &str = "http://www.w3.org/2000/svg";

#[derive(Default, Clone)]
pub struct RoughConfig {
  pub options: RoughOptions
}

// export interface DrawingSurface {
//   width: f64 | SVGAnimatedLength;
//   height: f64 | SVGAnimatedLength;
// }

// extends Options
#[derive(Clone)]
pub struct RoughOptions {
  // Options
  max_randomness_offset: f64,
  roughness: f64,
  bowing: f64,
  stroke: String,
  stroke_width: f64,
  curve_fitting: f64,
  curve_tightness: f64,
  curve_step_count: f64,
  fill: String,
  fill_style: String,
  fill_weight: f64,
  hachure_angle: f64,
  hachure_gap: f64,
  simplification: f64,
  dash_offset: f64,
  dash_gap: f64,
  zigzag_offset: f64,
  seed: f64,
  combine_nested_svg_paths: bool,
  stroke_line_dash: Option<Vec<f64>>,
  stroke_line_dash_offset: f64,
  fill_line_dash: Option<Vec<f64>>,
  fill_line_dash_offset: f64,
  disable_multi_stroke: bool,
  disable_multi_stroke_fill: bool,
  // randomizer?: Random;
}

impl Default for RoughOptions {
  fn default() -> Self {
    Self {
      max_randomness_offset: 2.0,
      roughness: 1.0,
      bowing: 1.0,
      stroke: "#000".into(),
      stroke_width: 1.0,
      curve_fitting: 0.95,
      curve_tightness: 0.0,
      curve_step_count: 9.0,
      fill: "".into(),
      fill_style: "hachure".into(),
      fill_weight: -1.0,
      hachure_angle: -41.0,
      hachure_gap: -1.0,
      simplification: 0.0,
      dash_offset: -1.0,
      dash_gap: -1.0,
      zigzag_offset: -1.0,
      seed: 0.0,
      combine_nested_svg_paths: false,
      stroke_line_dash: None,
      stroke_line_dash_offset: 0.0,
      fill_line_dash: None,
      fill_line_dash_offset: 0.0,
      disable_multi_stroke: false,
      disable_multi_stroke_fill: false,
    }
  }
}

#[derive(PartialEq, Eq)]
pub enum OpType {
  Move,
  BCurveTo,
  LineTo
}

#[derive(PartialEq, Eq, Debug)]
pub enum OpSetType {
  Path,
  FillPath,
  FillSketch,
}

pub struct Op {
  op: OpType,
  data: Vec<f64>,
}

pub struct OpSet {
  pub kind: OpSetType,
  pub ops: Vec<Op>,
  pub size: Option<Point<f64>>,
  pub path: Option<String>,
}

pub enum DrawableType {
  Line,
  Rectangle,
  Ellipse,
  Circle,
  LinearPath,
  Arc,
  Curve,
  Polygon,
  Path,
}

pub struct Drawable<'a> {
  pub shape: DrawableType,
  pub options: &'a RoughOptions,
  pub sets: Vec<OpSet>,
}

pub struct PathInfo {
  pub d: String,
  pub stroke: String,
  pub stroke_width: f64,
  pub fill: Option<String>,
}
