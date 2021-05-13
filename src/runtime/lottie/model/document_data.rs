#![allow(clippy::clippy::too_many_arguments)]
use crate::Color;

#[derive(PartialEq)]
pub enum Justification {
    LeftAlign,
    RightAlign,
    Center,
}

pub struct DocumentData {
    text: String,
    font_name: String,
    size: f64,
    justification: Justification,
    tracking: i32,
    line_height: f64,
    baseline_shift: f64,
    color: Color,
    stroke_color: Color,
    stroke_width: f64,
    stroke_over_fill: bool,
}

impl DocumentData {
    pub fn new(
        text: String,
        font_name: String,
        size: f64,
        justification: Justification,
        tracking: i32,
        line_height: f64,
        baseline_shift: f64,
        color: Color,
        stroke_color: Color,
        stroke_width: f64,
        stroke_over_fill: bool,
    ) -> Self {
        Self {
            text,
            font_name,
            size,
            justification,
            tracking,
            line_height,
            baseline_shift,
            color,
            stroke_color,
            stroke_width,
            stroke_over_fill,
        }
    }
}

impl PartialEq for DocumentData {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.font_name == other.font_name
            && self.tracking == other.tracking
            && self.size == other.size
            && self.justification == other.justification
            && self.line_height == other.line_height
            && self.baseline_shift == other.baseline_shift
            && self.color == other.color
            && self.stroke_color == other.stroke_color
            && self.stroke_width == other.stroke_width
            && self.stroke_over_fill == other.stroke_over_fill
    }
}

impl Eq for DocumentData {}
