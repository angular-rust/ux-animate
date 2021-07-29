#![allow(clippy::many_single_char_names)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![cfg(not(target_arch = "wasm32"))]

use crate::{
    BaseLine, CanvasContext, Color, Direction, FontStyle, FontWeight, Gradient, GradientType,
    LineCap, LineJoin, LinearGradient, PatternExtend, RadialGradient, RgbaColor, TextAlign,
    TextMetrics,
};
// Point, Rect, Size
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub extend: PatternExtend,
    pub inner: cairo::SurfacePattern,
}

impl Pattern {
    // Create pattern
    pub fn new(extend: PatternExtend, surface: &cairo::Surface) -> Self {
        Self {
            extend,
            inner: cairo::SurfacePattern::create(surface),
        }
    }
}

#[derive(Clone)]
enum Paint {
    None,
    Solid(Color),
    Gradient(Gradient),
    Pattern(Pattern),
}

impl Default for Paint {
    fn default() -> Self {
        Paint::None
    }
}

#[derive(Default, Clone)]
struct CanvasState {
    stroke: Paint,
    fill: Paint,
}

pub struct Canvas<'a> {
    ctx: &'a cairo::Context,
    state: RefCell<CanvasState>,
}

impl<'a> Canvas<'a> {
    #[allow(dead_code)]
    pub fn new(ctx: &'a cairo::Context) -> Self {
        Self {
            ctx,
            state: Default::default(),
        }
    }

    fn handle_paint(&self, paint: &Paint) -> bool {
        match paint {
            Paint::Solid(value) => {
                let value = *value;
                let color: RgbaColor = value.into();
                self.ctx.set_source_rgba(
                    color.red as f64 / 255.,
                    color.green as f64 / 255.,
                    color.blue as f64 / 255.,
                    color.alpha as f64 / 255.,
                );

                true
            }
            Paint::Gradient(value) => {
                match value.kind {
                    GradientType::Linear(params) => {
                        let LinearGradient { x0, y0, x1, y1 } = params;
                        let gradient = cairo::LinearGradient::new(x0, y0, x1, y1);
                        let stops = value.stops.borrow();
                        for stop in stops.iter() {
                            let RgbaColor {
                                red,
                                green,
                                blue,
                                alpha,
                            } = stop.color.into();

                            gradient.add_color_stop_rgba(
                                stop.offset,
                                red as f64 / 255.,
                                green as f64 / 255.,
                                blue as f64 / 255.,
                                alpha as f64 / 255.,
                            );
                        }
                        self.ctx.set_source(&gradient);
                    }
                    GradientType::Radial(params) => {
                        let RadialGradient {
                            x0,
                            y0,
                            r0,
                            x1,
                            y1,
                            r1,
                        } = params;
                        let gradient = cairo::RadialGradient::new(x0, y0, r0, x1, y1, r1);
                        let stops = value.stops.borrow();
                        for stop in stops.iter() {
                            let RgbaColor {
                                red,
                                green,
                                blue,
                                alpha,
                            } = stop.color.into();

                            gradient.add_color_stop_rgba(
                                stop.offset,
                                red as f64 / 255.,
                                green as f64 / 255.,
                                blue as f64 / 255.,
                                alpha as f64 / 255.,
                            );
                        }
                        self.ctx.set_source(&gradient);
                    }
                }

                true
            }
            Paint::Pattern(value) => {
                let extend = match value.extend {
                    PatternExtend::None => cairo::Extend::None,
                    PatternExtend::Repeat => cairo::Extend::Repeat,
                    PatternExtend::Reflect => cairo::Extend::Reflect,
                    PatternExtend::Pad => cairo::Extend::Pad,
                };

                value.inner.set_extend(extend);
                self.ctx.set_source(&value.inner);

                true
            }
            Paint::None => false,
        }
    }
}

impl<'a> CanvasContext for Canvas<'a> {
    type Pattern = Pattern;
    // fn get_current_transform(&self) -> Matrix;

    // fn set_current_transform(&self, value: Matrix<f64>) {
    //     // self.ctx.get_matrix()
    //     unimplemented!()
    // }

    fn get_direction(&self) -> Direction {
        unimplemented!()
    }

    fn set_direction(&self, value: Direction) -> String {
        info!("NOT IMPLEMENTED");
        unimplemented!()
    }

    fn set_fill_color(&self, value: Color) {
        let mut state = self.state.borrow_mut();
        state.fill = Paint::Solid(value);
    }

    fn set_fill_gradient(&self, value: &Gradient) {
        let mut state = self.state.borrow_mut();
        state.fill = Paint::Gradient(value.clone());
    }

    fn set_fill_pattern(&self, value: &Self::Pattern) {
        let mut state = self.state.borrow_mut();
        state.fill = Paint::Pattern(value.clone());
    }

    fn get_filter(&self) -> String {
        unimplemented!()
    }

    fn set_filter(&self, value: &str) {
        unimplemented!()
    }

    fn get_font(&self) -> String {
        // self.ctx.get_font_face(font_face)
        unimplemented!()
    }

    fn set_font(&self, family: &str, style: FontStyle, weight: FontWeight, size: f64) {
        let slant = match style {
            FontStyle::Italic => cairo::FontSlant::Italic,
            FontStyle::Normal => cairo::FontSlant::Normal,
            FontStyle::Oblique => cairo::FontSlant::Oblique,
        };

        let weight = match weight {
            FontWeight::Bold => cairo::FontWeight::Bold,
            _ => cairo::FontWeight::Normal,
        };

        self.ctx.select_font_face(family, slant, weight);
        self.ctx.set_font_size(size);
    }

    fn get_global_alpha(&self) -> f64 {
        unimplemented!()
    }

    fn set_global_alpha(&self, value: f64) {
        unimplemented!()
    }

    fn get_global_composite_operation(&self) -> String {
        unimplemented!()
    }

    fn set_global_composite_operation(&self, value: &str) {
        unimplemented!()
    }

    // Whether images and patterns on this canvas will be smoothed when this canvas is scaled.
    // imageSmoothingEnabled
    fn is_image_smoothing_enabled(&self) -> bool {
        unimplemented!()
    }

    fn set_image_smoothing(&self, value: bool) {
        unimplemented!()
    }

    // fn get_image_smoothing_quality(&self) -> String {
    //     unimplemented!()
    // }

    // fn set_image_smoothing_quality(&self, value: &str) {
    //     unimplemented!()
    // }

    fn get_line_cap(&self) -> LineCap {
        match self.ctx.get_line_cap() {
            cairo::LineCap::Round => LineCap::Round,
            cairo::LineCap::Square => LineCap::Square,
            _ => LineCap::Butt, // Return default value
        }
    }

    fn set_line_cap(&self, value: LineCap) {
        let value = match value {
            LineCap::Round => cairo::LineCap::Round,
            LineCap::Square => cairo::LineCap::Square,
            LineCap::Butt => cairo::LineCap::Butt,
        };
        self.ctx.set_line_cap(value)
    }

    fn get_line_dash_offset(&self) -> f64 {
        let (_, result) = self.ctx.get_dash();
        result
    }

    fn set_line_dash_offset(&self, value: f64) {
        let (dash, _) = self.ctx.get_dash();
        self.ctx.set_dash(&dash, value);
    }

    fn get_line_join(&self) -> LineJoin {
        match self.ctx.get_line_join() {
            cairo::LineJoin::Bevel => LineJoin::Bevel,
            cairo::LineJoin::Round => LineJoin::Round,
            _ => LineJoin::Miter,
        }
    }

    fn set_line_join(&self, value: LineJoin) {
        let value = match value {
            LineJoin::Bevel => cairo::LineJoin::Bevel,
            LineJoin::Round => cairo::LineJoin::Round,
            LineJoin::Miter => cairo::LineJoin::Miter,
        };
        self.ctx.set_line_join(value)
    }

    fn get_line_width(&self) -> f64 {
        self.ctx.get_line_width()
    }

    fn set_line_width(&self, value: f64) {
        self.ctx.set_line_width(value);
    }

    fn get_miter_limit(&self) -> f64 {
        self.ctx.get_miter_limit()
    }

    fn set_miter_limit(&self, value: f64) {
        self.ctx.set_miter_limit(value);
    }

    fn get_shadow_blur(&self) -> f64 {
        unimplemented!()
    }

    fn set_shadow_blur(&self, value: f64) {
        unimplemented!()
    }

    fn get_shadow_color(&self) -> Color {
        unimplemented!()
    }
    fn set_shadow_color(&self, value: Color) {
        unimplemented!()
    }

    fn get_shadow_offset_x(&self) -> f64 {
        unimplemented!()
    }
    fn set_shadow_offset_x(&self, value: f64) {
        unimplemented!()
    }

    fn get_shadow_offset_y(&self) -> f64 {
        unimplemented!()
    }

    fn set_shadow_offset_y(&self, value: f64) {
        unimplemented!()
    }

    fn set_stroke_color(&self, value: Color) {
        let mut state = self.state.borrow_mut();
        state.stroke = Paint::Solid(value);
    }

    fn set_stroke_gradient(&self, value: &Gradient) {
        let mut state = self.state.borrow_mut();
        state.stroke = Paint::Gradient(value.clone());
    }

    fn set_stroke_pattern(&self, value: &Self::Pattern) {
        let mut state = self.state.borrow_mut();
        state.stroke = Paint::Pattern(value.clone());
    }

    fn get_text_align(&self) -> TextAlign {
        unimplemented!()
    }

    fn set_text_align(&self, value: TextAlign) {
        //TODO: complete it
    }

    fn get_text_baseline(&self) -> BaseLine {
        unimplemented!()
    }

    fn set_text_baseline(&self, value: BaseLine) {
        //TODO: complete it
    }

    // anticlockwise: bool = false
    fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        if anticlockwise {
            self.ctx.arc_negative(x, y, radius, start_angle, end_angle);
        } else {
            self.ctx.arc(x, y, radius, start_angle, end_angle);
        }
    }

    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        unimplemented!()
    }

    fn begin_path(&self) {
        self.ctx.new_path();
    }

    fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.ctx.curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    // FIXME: seems code correct but it should clear with transpatency but it black (((
    fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.new_path();
        self.ctx.save();
        self.ctx.set_source_rgba(0., 0., 0., 0.);
        self.ctx.set_operator(cairo::Operator::Clear);
        self.ctx.rectangle(x, y, width, height);
        self.ctx.fill();
        self.ctx.restore();
    }

    // [path_OR_winding: dynamic, winding: String]
    // fn clip(path_OR_winding: dynamic, winding: String); // TODO:
    fn close_path(&self) {
        self.ctx.close_path();
    }

    // @Creates('ImageData|=Object')
    // [int? sh_OR_sw, dynamic imageDataColorSettings_OR_sh, Map? imageDataColorSettings]
    // fn createImageData(data_OR_imagedata_OR_sw: dynamic, sh_OR_sw: int, imageDataColorSettings_OR_sh: dynamic, imageDataColorSettings: Map) -> ImageData; // TODO:

    // fn createImageDataFromImageData(imagedata: ImageData) -> ImageData; // TODO:

    // [Element? element]
    // fn drawFocusIfNeeded(element_OR_path: dynamic, element: Element); // TODO:

    // Draws an image from a CanvasImageSource to this canvas.
    // @JSName('drawImage')
    // fn drawImage(source: CanvasImageSource, destX: f64, destY: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // @JSName('drawImage')
    // fn drawImageScaled(source: CanvasImageSource, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // @JSName('drawImage')
    // fn drawImageScaledFromSource(source: CanvasImageSource, sourceX: f64, sourceY: f64, sourceWidth: f64, sourceHeight: f64, destX: f64, destY: f64, destWidth: f64, destHeight: f64); // TODO:

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // {Rectangle<f64>? sourceRect}
    // fn drawImageToRect(source: CanvasImageSource, destRect: Rectangle<f64>, sourceRect: Rectangle<f64>); // TODO:

    fn ellipse(
        &self,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        self.ctx.save();

        self.ctx.translate(x, y);
        self.ctx.scale(1., radius_y / radius_x);
        self.ctx.rotate(rotation);
        self.ctx.translate(-x, -y);

        self.ctx.arc(x, y, radius_x, start_angle, end_angle);

        self.ctx.restore();
        // stroke later to prevent stroke ugly transformation
    }

    // [dynamic path_OR_winding, String? winding]
    // fn fill(path_OR_winding: dynamic, winding: String); // TODO:

    fn fill(&self) {
        let state = self.state.borrow();
        if self.handle_paint(&state.fill) {
            self.ctx.fill_preserve();
        }
    }

    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let state = self.state.borrow();
        if self.handle_paint(&state.fill) {
            self.ctx.new_path();
            self.ctx.rectangle(x, y, width, height);
            self.ctx.fill();
        }
    }

    // Draws text to the canvas.
    fn fill_text(&self, text: &str, x: f64, y: f64) {
        let state = self.state.borrow();
        if self.handle_paint(&state.fill) {
            self.ctx.new_path();
            self.ctx.save();
            self.ctx.move_to(x, y);
            self.ctx.text_path(text);
            self.ctx.fill();
            self.ctx.restore();
        }
    }

    // fn getContextAttributes() -> Map; // TODO:

    // @Creates('ImageData|=Object')
    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData; // TODO:

    fn get_line_dash(&self) -> Vec<f64> {
        let (result, _) = self.ctx.get_dash();
        result
    }

    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool; // TODO:

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool; // TODO:
    fn line_to(&self, x: f64, y: f64) {
        self.ctx.line_to(x, y);
    }

    fn measure_text(&self, text: &str) -> TextMetrics {
        let ext = self.ctx.text_extents(text);
        TextMetrics {
            width: ext.width,
            height: ext.height,
        }
    }

    fn move_to(&self, x: f64, y: f64) {
        self.ctx.move_to(x, y);
    }

    // [int? dirtyX, int? dirtyY, int? dirtyWidth, int? dirtyHeight]
    // fn putImageData(imagedata: ImageData, dx: i64, dy: i64, dirtyX: i64, dirtyY: i64, dirtyWidth: i64, dirtyHeight: i64); // TODO:
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        // A quadratic Bezier can be always represented by a cubic one by
        // applying the degree elevation algorithm. The resulted cubic representation
        // will share its anchor points with the original quadratic,
        // while the control points will be at 2/3 of the quadratic handle segments:

        // C1 = (2·C + P1)/3
        // C2 = (2·C + P2)/3

        let (x0, y0) = self.ctx.get_current_point();
        self.ctx.curve_to(
            2.0 / 3.0 * cpx + 1.0 / 3.0 * x0,
            2.0 / 3.0 * cpy + 1.0 / 3.0 * y0,
            2.0 / 3.0 * cpx + 1.0 / 3.0 * x,
            2.0 / 3.0 * cpy + 1.0 / 3.0 * y,
            x,
            y,
        );
    }

    fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.rectangle(x, y, width, height);
    }

    fn reset_transform(&self) {
        unimplemented!()
    }

    fn restore(&self) {
        self.ctx.restore();
    }

    fn rotate(&self, angle: f64) {
        self.ctx.rotate(angle);
    }

    fn save(&self) {
        self.ctx.save();
    }

    fn scale(&self, x: f64, y: f64) {
        self.ctx.scale(x, y);
    }

    // [Path2D? path]
    // fn scrollPathIntoView(path: Path2D); // TODO:

    fn set_line_dash(&self, dash: &[f64]) {
        let (_, offset) = self.ctx.get_dash();
        self.ctx.set_dash(dash, offset);
    }

    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let m = cairo::Matrix::new(a, b, c, d, e, f);
        self.ctx.transform(m);
    }

    fn stroke(&self) {
        let state = self.state.borrow();
        if self.handle_paint(&state.stroke) {
            self.ctx.stroke_preserve();
        }
    }

    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let state = self.state.borrow();
        if self.handle_paint(&state.stroke) {
            self.ctx.new_path();
            self.ctx.rectangle(x, y, width, height);
            self.ctx.stroke();
        }
    }

    fn stroke_text(&self, text: &str, x: f64, y: f64) {
        let state = self.state.borrow();
        if self.handle_paint(&state.stroke) {
            self.ctx.new_path();
            self.ctx.save();
            self.ctx.move_to(x, y);
            self.ctx.text_path(text);
            self.ctx.stroke();
            self.ctx.restore();
        }
    }

    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let m = cairo::Matrix::new(a, b, c, d, e, f);
        self.ctx.transform(m);
    }

    fn translate(&self, x: f64, y: f64) {
        self.ctx.translate(x, y);
    }
}
