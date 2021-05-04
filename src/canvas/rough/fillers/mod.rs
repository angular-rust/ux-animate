use crate::{
    canvas::rough::{Op, OpSet, RoughOptions},
    Point,
};

mod dashed;
mod dot;
mod hachure;
mod hatch;
mod scanline_hachure;
mod zigzag;
mod zigzag_line;

// import { ResolvedOptions, OpSet, Op } from '../core';
// import { Point } from '../geometry';

pub(crate) use dashed::DashedFiller;
pub(crate) use dot::DotFiller;
pub(crate) use hachure::HachureFiller;
pub(crate) use hatch::HatchFiller;
pub(crate) use scanline_hachure::*;
pub(crate) use zigzag::ZigZagFiller;
pub(crate) use zigzag_line::ZigZagLineFiller;


pub struct Line<T> {
    pub start: Point<T>,
    pub end: Point<T>,
}

pub trait PatternFiller {
  fn fill_polygon(&self, points: Vec<Point<f64>>, options: &RoughOptions) -> OpSet;
}

pub trait RenderHelper {
    fn rand_offset(&self, x: f64, options: &RoughOptions) -> f64;
    fn rand_offset_with_range(&self, min: f64, max: f64, options: &RoughOptions) -> f64;
    fn ellipse(&self, x: f64, y: f64, width: f64, height: f64, options: &RoughOptions) -> OpSet;
    fn double_line_ops(&self, x1: f64, y1: f64, x2: f64, y2: f64, options: &RoughOptions) -> Vec<Op>;
}

// pub const fillers: { [name: string]: PatternFiller } = {};

pub fn get_filler(options: &RoughOptions, helper: impl RenderHelper) -> Box<dyn PatternFiller> {
    //   let fillerName = o.fillStyle || 'hachure';
    //   if (!fillers[fillerName]) {
    //     switch (fillerName) {
    //       case 'zigzag':
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new ZigZagFiller(helper);
    //         }
    //         break;
    //       case 'cross-hatch':
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new HatchFiller(helper);
    //         }
    //         break;
    //       case 'dots':
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new DotFiller(helper);
    //         }
    //         break;
    //       case 'dashed':
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new DashedFiller(helper);
    //         }
    //         break;
    //       case 'zigzag-line':
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new ZigZagLineFiller(helper);
    //         }
    //         break;
    //       case 'hachure':
    //       default:
    //         fillerName = 'hachure';
    //         if (!fillers[fillerName]) {
    //           fillers[fillerName] = new HachureFiller(helper);
    //         }
    //         break;
    //     }
    //   }
    //   return fillers[fillerName];
    unimplemented!()
}
