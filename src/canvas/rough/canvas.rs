#![allow(unused_variables)]
use std::cell::RefCell;

use super::{Drawable, OpSet, OpSetType, OpType, RoughConfig, RoughGenerator, RoughOptions};
use crate::{
    BaseLine, CanvasContext, Color, Direction, Gradient, GradientType, LineCap, LineJoin,
    LinearGradient, Pattern, PatternExtend, Point, RadialGradient, RgbaColor, TextAlign,
    TextMetrics, TextStyle, TextWeight,
};

#[derive(Default)]
struct RoughState {
    cursor: Point<f64>,
}

pub struct RoughCanvas<'a, C>
where
    C: CanvasContext,
{
    gen: RoughGenerator,
    ctx: &'a C,
    state: RefCell<RoughState>,
}

impl<'a, C> RoughCanvas<'a, C>
where
    C: CanvasContext,
{
    pub fn new(ctx: &'a C, config: RoughConfig) -> Self {
        Self {
            ctx,
            gen: RoughGenerator::new(config),
            state: Default::default(),
        }
    }

    pub fn draw(&self, drawable: Drawable<'a>) -> Drawable<'a> {
        let options = drawable.options;

        for drawing in drawable.sets.iter() {
            // println!("HERE {:?}", drawing.kind);
            match drawing.kind {
                OpSetType::Path => {
                    self.ctx.save();
                    // ctx.stroke_style = o.stroke == "none" ? "transparent" : o.stroke;
                    // println!("HERE {:?}, {}", drawing.kind, options.stroke_width);
                    self.ctx.set_line_width(options.stroke_width);

                    if let Some(line_dash) = &options.stroke_line_dash {
                        // println!("DASH");
                        self.ctx.set_line_dash(line_dash);
                        self.ctx
                            .set_line_dash_offset(options.stroke_line_dash_offset);
                    }

                    self._draw_to_context(drawing);
                    self.ctx.restore();
                }
                OpSetType::FillPath => {
                    self.ctx.save();
                    // self.ctx.fillStyle = o.fill || "";
                    // let fillRule: CanvasFillRule = (drawable.shape == "curve" || drawable.shape == "polygon") ? "evenodd" : "nonzero";
                    self._draw_to_context(drawing); // fillRule
                    self.ctx.restore();
                }
                OpSetType::FillSketch => {
                    self.fill_sketch(drawing, options);
                }
            }
        }
        drawable
    }

    fn fill_sketch(&self, drawing: &OpSet, o: &RoughOptions) {
        self.ctx.save();

        if let Some(line_dash) = &o.fill_line_dash {
            self.ctx.set_line_dash(line_dash);
            self.ctx.set_line_dash_offset(o.fill_line_dash_offset);
        }

        // ctx.strokeStyle = o.fill || '';

        if o.fill_weight < 0.0 {
            self.ctx.set_line_width(o.stroke_width / 2.0);
        } else {
            self.ctx.set_line_width(o.fill_weight);
        }

        self._draw_to_context(drawing);
        self.ctx.restore();
    }

    fn _draw_to_context(&self, drawing: &OpSet /* , rule: CanvasFillRule*/) {
        // // rule: CanvasFillRule = 'nonzero'
        self.ctx.begin_path();
        for item in drawing.ops.iter() {
            let data = &item.data;
            match item.op {
                OpType::Move => {
                    // println!("MoveTo {}:{}", data[0], data[1]);
                    self.ctx.move_to(data[0], data[1]);
                }
                OpType::BCurveTo => {
                    // println!(
                    //     "BCurveTo CP1[{}:{}] CP2[{}:{}] P[{}:{}]",
                    //     data[0], data[1], data[2], data[3], data[4], data[5]
                    // );
                    self.ctx
                        .bezier_curve_to(data[0], data[1], data[2], data[3], data[4], data[5]);
                }
                OpType::LineTo => {
                    // println!("LineTo {}:{}", data[0], data[1]);
                    self.ctx.line_to(data[0], data[1]);
                }
            }
        }

        if drawing.kind == OpSetType::FillPath {
            self.ctx.fill(); // rule
        } else {
            self.ctx.stroke();
        }
    }

    pub fn get_generator(&self) -> &RoughGenerator {
        &self.gen
    }

    pub fn get_default_options(&self) -> &RoughOptions {
        &self.gen.default_options
    }

    pub fn circle(&self, x: f64, y: f64, diameter: f64, options: &'a RoughOptions) -> Drawable<'a> {
        let drawings = self.gen.circle(x, y, diameter, options);
        self.draw(drawings)
    }

    pub fn linear_path(&self, points: &Vec<Point<f64>>, options: &'a RoughOptions) -> Drawable<'a> {
        let drawings = self.gen.linear_path(points, options);
        self.draw(drawings)
    }

    pub fn polygon(&self, points: &Vec<Point<f64>>, options: &'a RoughOptions) -> Drawable<'a> {
        let drawings = self.gen.polygon(points, options);
        self.draw(drawings)
    }

    pub fn curve(&self, points: &Vec<Point<f64>>, options: &'a RoughOptions) -> Drawable<'a> {
        let drawing = self.gen.curve(points, options);
        self.draw(drawing)
    }

    pub fn path(&self, d: &str, options: &'a RoughOptions) -> Drawable<'a> {
        let drawing = self.gen.path(d, options);
        self.draw(drawing)
    }
}

impl<'a, C> CanvasContext for RoughCanvas<'a, C>
where
    C: CanvasContext<Pattern = Pattern>,
{
    type Pattern = Pattern;

    // fn get_current_transform(&self) -> Matrix;

    // fn set_current_transform(&self, value: Matrix<f64>) {
    //     self.ctx.set_current_transform(value)
    // }

    fn get_direction(&self) -> Direction {
        self.ctx.get_direction()
    }

    fn set_direction(&self, value: Direction) -> String {
        self.ctx.set_direction(value)
    }

    fn set_fill_color(&self, value: Color) {
        self.ctx.set_fill_color(value);
    }

    fn set_fill_gradient(&self, value: &Gradient) {
        self.ctx.set_fill_gradient(value);
    }

    fn set_fill_pattern(&self, pattern: &Self::Pattern) {
        self.ctx.set_fill_pattern(pattern);
    }

    fn get_filter(&self) -> String {
        self.ctx.get_filter()
    }

    fn set_filter(&self, value: &str) {
        self.ctx.set_filter(value)
    }

    fn get_font(&self) -> String {
        self.ctx.get_font()
    }

    fn set_font(&self, family: &str, style: TextStyle, weight: TextWeight, size: f64) {
        self.ctx.set_font(family, style, weight, size);
    }

    fn get_global_alpha(&self) -> f64 {
        self.ctx.get_global_alpha()
    }

    fn set_global_alpha(&self, value: f64) {
        self.ctx.set_global_alpha(value)
    }

    fn get_global_composite_operation(&self) -> String {
        self.ctx.get_global_composite_operation()
    }

    fn set_global_composite_operation(&self, value: &str) {
        self.ctx.set_global_composite_operation(value)
    }

    // Whether images and patterns on this canvas will be smoothed when this canvas is scaled.
    // imageSmoothingEnabled
    fn is_image_smoothing_enabled(&self) -> bool {
        self.ctx.is_image_smoothing_enabled()
    }

    fn set_image_smoothing(&self, value: bool) {
        self.ctx.set_image_smoothing(value)
    }

    // fn get_image_smoothing_quality(&self) -> String {
    //     self.ctx.get_image_smoothing_quality()
    // }

    // fn set_image_smoothing_quality(&self, value: &str) {
    //     self.ctx.get_image_smoothing_quality(value)
    // }

    fn get_line_cap(&self) -> LineCap {
        self.ctx.get_line_cap()
    }

    fn set_line_cap(&self, value: LineCap) {
        self.ctx.set_line_cap(value)
    }

    fn get_line_dash_offset(&self) -> f64 {
        self.ctx.get_line_dash_offset()
    }

    fn set_line_dash_offset(&self, value: f64) {
        self.ctx.set_line_dash_offset(value)
    }

    fn get_line_join(&self) -> LineJoin {
        self.ctx.get_line_join()
    }

    fn set_line_join(&self, value: LineJoin) {
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
        self.ctx.get_shadow_blur()
    }

    fn set_shadow_blur(&self, value: f64) {
        self.ctx.set_shadow_blur(value)
    }

    fn get_shadow_color(&self) -> Color {
        self.ctx.get_shadow_color()
    }
    fn set_shadow_color(&self, value: Color) {
        self.ctx.set_shadow_color(value)
    }

    fn get_shadow_offset_x(&self) -> f64 {
        self.ctx.get_shadow_offset_x()
    }

    fn set_shadow_offset_x(&self, value: f64) {
        self.ctx.set_shadow_offset_x(value)
    }

    fn get_shadow_offset_y(&self) -> f64 {
        self.ctx.get_shadow_offset_y()
    }

    fn set_shadow_offset_y(&self, value: f64) {
        self.ctx.set_shadow_offset_y(value)
    }

    fn set_stroke_color(&self, value: Color) {
        self.ctx.set_stroke_color(value)
    }

    fn set_stroke_gradient(&self, value: &Gradient) {
        self.ctx.set_stroke_gradient(value)
    }

    fn set_stroke_pattern(&self, pattern: &Self::Pattern) {
        self.ctx.set_stroke_pattern(pattern)
    }

    fn get_text_align(&self) -> TextAlign {
        self.ctx.get_text_align()
    }

    fn set_text_align(&self, value: TextAlign) {
        self.ctx.set_text_align(value)
    }

    fn get_text_baseline(&self) -> BaseLine {
        self.ctx.get_text_baseline()
    }

    fn set_text_baseline(&self, value: BaseLine) {
        self.ctx.set_text_baseline(value)
    }

    fn arc(
        &self,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) {
        // anticlockwise: bool = false
        // closed: boolean = false
        let options = Default::default();

        let drawing = self.gen.arc(
            x,
            y,
            radius,
            radius,
            start_angle,
            end_angle,
            false,
            &options,
        );
        self.draw(drawing);
    }

    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        self.ctx.arc_to(x1, y1, x2, y2, radius)
    }

    fn begin_path(&self) {
        self.ctx.begin_path();
    }

    fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.ctx.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.clear_rect(x, y, width, height)
    }

    // [path_OR_winding: dynamic, winding: String]
    // fn clip(path_OR_winding: dynamic, winding: String);
    fn close_path(&self) {
        self.ctx.close_path();
    }

    // @Creates('ImageData|=Object')
    // [int? sh_OR_sw, dynamic imageDataColorSettings_OR_sh, Map? imageDataColorSettings]
    // fn createImageData(data_OR_imagedata_OR_sw: dynamic, sh_OR_sw: int, imageDataColorSettings_OR_sh: dynamic, imageDataColorSettings: Map) -> ImageData;

    // fn createImageDataFromImageData(imagedata: ImageData) -> ImageData;

    // [Element? element]
    // fn drawFocusIfNeeded(element_OR_path: dynamic, element: Element);

    // Draws an image from a CanvasImageSource to this canvas.
    // @JSName('drawImage')
    // fn drawImage(source: CanvasImageSource, destX: f64, destY: f64);

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // @JSName('drawImage')
    // fn drawImageScaled(source: CanvasImageSource, destX: f64, destY: f64, destWidth: f64, destHeight: f64);

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // @JSName('drawImage')
    // fn drawImageScaledFromSource(source: CanvasImageSource, sourceX: f64, sourceY: f64, sourceWidth: f64, sourceHeight: f64, destX: f64, destY: f64, destWidth: f64, destHeight: f64);

    // Draws an image from a CanvasImageSource to an area of this canvas.
    // {Rectangle<f64>? sourceRect}
    // fn drawImageToRect(source: CanvasImageSource, destRect: Rectangle<f64>, sourceRect: Rectangle<f64>);

    // pub fn ellipse(
    //     &self,
    //     x: f64,
    //     y: f64,
    //     width: f64,
    //     height: f64,
    //     options: &'a RoughOptions,
    // ) -> Drawable<'a> {
    //     let drawings = self.gen.ellipse(x, y, width, height, options);
    //     self.draw(drawings)
    // }

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
        // self.ctx.ellipse(
        //     x,
        //     y,
        //     radius_x,
        //     radius_y,
        //     rotation,
        //     start_angle,
        //     end_angle,
        //     anticlockwise,
        // )
        let options = Default::default();
        let drawings = self
            .gen
            .ellipse(x, y, radius_x * 2.0, radius_y * 2.0, &options);
        self.draw(drawings);
    }

    // [dynamic path_OR_winding, String? winding]
    // fn fill(path_OR_winding: dynamic, winding: String);

    fn fill(&self) {
        self.ctx.fill();
    }

    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        // self.ctx.fill_rect(x, y, width, height);
        let options = Default::default();
        let drawings = self.gen.rectangle(x, y, width, height, &options);
        self.draw(drawings);
    }

    // Draws text to the canvas.
    fn fill_text(&self, text: &str, x: f64, y: f64) {
        self.ctx.fill_text(text, x, y);
    }

    // fn getContextAttributes() -> Map;

    // @Creates('ImageData|=Object')
    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData;

    fn get_line_dash(&self) -> Vec<f64> {
        self.ctx.get_line_dash()
    }

    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool;

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool;
    fn line_to(&self, x: f64, y: f64) {
        let options = Default::default();
        let mut state = self.state.borrow_mut();

        let drawings = self
            .gen
            .line(state.cursor.x, state.cursor.y, x, y, &options);
        self.draw(drawings);

        state.cursor.x = x;
        state.cursor.y = y;
    }

    fn measure_text(&self, text: &str) -> TextMetrics {
        self.ctx.measure_text(text)
    }

    fn move_to(&self, x: f64, y: f64) {
        let mut state = self.state.borrow_mut();
        state.cursor.x = x;
        state.cursor.y = y;
    }

    // [int? dirtyX, int? dirtyY, int? dirtyWidth, int? dirtyHeight]
    // fn putImageData(imagedata: ImageData, dx: i64, dy: i64, dirtyX: i64, dirtyY: i64, dirtyWidth: i64, dirtyHeight: i64);
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.ctx.quadratic_curve_to(cpx, cpy, x, y)
    }

    fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let options = Default::default();
        let drawings = self.gen.rectangle(x, y, width, height, &options);
        self.draw(drawings);
    }

    fn reset_transform(&self) {
        self.ctx.reset_transform()
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
    // fn scrollPathIntoView(path: Path2D);

    fn set_line_dash(&self, dash: &Vec<f64>) {
        self.ctx.set_line_dash(dash)
    }

    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.ctx.set_transform(a, b, c, d, e, f)
    }

    fn stroke(&self) {
        self.ctx.stroke();
    }

    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        let options = Default::default();
        let drawings = self.gen.rectangle(x, y, width, height, &options);
        self.draw(drawings);
    }

    fn stroke_text(&self, text: &str, x: f64, y: f64) {
        self.ctx.stroke_text(text, x, y)
    }

    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.ctx.transform(a, b, c, d, e, f)
    }

    fn translate(&self, x: f64, y: f64) {
        self.ctx.translate(x, y);
    }
}
