#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg(target_arch = "wasm32")]

use primitives::{
    BaseLine, CanvasContext, Color, Direction, Gradient, LineCap, LineJoin, Point, Rect, RgbColor,
    Size, TextAlign, TextMetrics, TextStyle, TextWeight, ToHexString,
};

use wasm_bindgen::JsValue;
use wasm_bindgen_test::console_log;
use web_sys;

#[derive(Debug, Clone, Copy)]
pub struct Pattern {}

impl Pattern {
    // Create pattern
    // fn createPattern(image: Object, repetitionType: String) -> CanvasPattern;
    // /// Create pattern from image
    // fn createPatternFromImage(image: ImageElement, repetitionType: String) -> CanvasPattern;
    pub fn new() -> Self {
        unimplemented!()
    }
}

pub struct Canvas<'a> {
    ctx: &'a web_sys::CanvasRenderingContext2d,
}

impl<'a> Canvas<'a> {
    pub fn new(ctx: &'a web_sys::CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }
}

impl<'a> CanvasContext<Pattern> for Canvas<'a> {
    // fn get_transform(&self) -> Matrix {
    //    self.ctx.get_transform() -> Result<DomMatrix, JsValue>
    // }

    // fn set_current_transform(&self, value: Matrix<f64>) {
    //     let _ = self.ctx.set_transform(value.a, value.b, value.c, value.d, value.e, value.f);
    // }

    fn get_direction(&self) -> Direction {
        unimplemented!() // FIXME: not implemented in web_sys
    }

    fn set_direction(&self, value: Direction) -> String {
        unimplemented!() // FIXME: not implemented in web_sys
    }

    fn set_fill_color(&self, value: Color) {
        let color = JsValue::from(value.to_hex_string());
        self.ctx.set_fill_style(&color);
    }

    fn set_fill_gradient(&self, value: &Gradient) {
        unimplemented!()
    }

    fn set_fill_pattern(&self, pattern: &Pattern) {
        println!("SET FILL PATTERN");
    }

    fn get_filter(&self) -> String {
        self.ctx.filter()
    }

    fn set_filter(&self, value: &str) {
        self.ctx.set_filter(value);
    }

    fn get_font(&self) -> String {
        self.ctx.font()
    }

    fn set_font(&self, family: &str, style: TextStyle, weight: TextWeight, size: f64) {
        // TODO: handle style and weight
        self.ctx.set_font(format!("{}px {}", size, family).as_str());
    }

    fn get_global_alpha(&self) -> f64 {
        self.ctx.global_alpha()
    }

    fn set_global_alpha(&self, value: f64) {
        self.ctx.set_global_alpha(value);
    }

    fn get_global_composite_operation(&self) -> String {
        // TODO: handle err
        self.ctx.global_composite_operation().unwrap()
    }

    fn set_global_composite_operation(&self, value: &str) {
        // TODO: handle err
        let _ = self.ctx.set_global_composite_operation(value);
    }

    // Whether images and patterns on this canvas will be smoothed when this canvas is scaled.
    // imageSmoothingEnabled
    fn is_image_smoothing_enabled(&self) -> bool {
        self.ctx.image_smoothing_enabled()
    }

    fn set_image_smoothing(&self, value: bool) {
        self.ctx.set_image_smoothing_enabled(value);
    }

    // fn get_image_smoothing_quality(&self) -> String {
    //     unimplemented!()
    // }

    // fn set_image_smoothing_quality(&self, value: String) {
    //     unimplemented!()
    // }

    fn get_line_cap(&self) -> LineCap {
        // self.ctx.line_cap()
        unimplemented!()
    }

    fn set_line_cap(&self, value: LineCap) {
        // self.ctx.set_line_cap(value);
    }

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash_offset(&self) -> f64 {
        self.ctx.line_dash_offset()
    }

    fn set_line_dash_offset(&self, value: f64) {
        self.ctx.set_line_dash_offset(value);
    }

    fn get_line_join(&self) -> LineJoin {
        // self.ctx.line_join()
        unimplemented!()
    }

    fn set_line_join(&self, value: LineJoin) {
        // TODO: complete it
        // self.ctx.set_line_join(value)
    }

    fn get_line_width(&self) -> f64 {
        self.ctx.line_width()
    }

    fn set_line_width(&self, value: f64) {
        self.ctx.set_line_width(value);
    }

    fn get_miter_limit(&self) -> f64 {
        self.ctx.miter_limit()
    }

    fn set_miter_limit(&self, value: f64) {
        self.ctx.set_miter_limit(value);
    }

    fn get_shadow_blur(&self) -> f64 {
        self.ctx.shadow_blur()
    }

    fn set_shadow_blur(&self, value: f64) {
        self.ctx.set_shadow_blur(value);
    }

    fn get_shadow_color(&self) -> Color {
        // self.ctx.shadow_color()
        unimplemented!()
    }

    fn set_shadow_color(&self, value: Color) {
        // self.ctx.set_shadow_color(value);
        unimplemented!()
    }

    fn get_shadow_offset_x(&self) -> f64 {
        self.ctx.shadow_offset_x()
    }

    fn set_shadow_offset_x(&self, value: f64) {
        self.ctx.set_shadow_offset_x(value);
    }

    fn get_shadow_offset_y(&self) -> f64 {
        self.ctx.shadow_offset_y()
    }

    fn set_shadow_offset_y(&self, value: f64) {
        self.ctx.set_shadow_offset_y(value);
    }

    fn set_stroke_color(&self, value: Color) {
        let color = JsValue::from(value.to_hex_string());
        self.ctx.set_stroke_style(&color);
    }

    fn set_stroke_gradient(&self, value: &Gradient) {
        unimplemented!()
    }

    fn set_stroke_pattern(&self, pattern: &Pattern) {
        println!("SET STROKE PATTERN");
    }

    fn get_text_align(&self) -> TextAlign {
        // self.ctx.text_align()
        unimplemented!()
    }

    fn set_text_align(&self, value: TextAlign) {
        // TODO: complete it
        // self.ctx.set_text_align(value);
    }

    fn get_text_baseline(&self) -> BaseLine {
        // self.ctx.text_baseline()
        unimplemented!()
    }

    fn set_text_baseline(&self, value: BaseLine) {
        // TODO: complete it
        // self.ctx.set_text_baseline(value);
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
            // let _ = self.ctx.arc(x, y, radius, start_angle, end_angle);
            let _ = self.ctx.arc_with_anticlockwise(
                x,
                y,
                radius,
                start_angle,
                end_angle,
                anticlockwise,
            );
        } else {
            let _ = self.ctx.arc(x, y, radius, start_angle, end_angle);
            // let _ = self.ctx.arc_with_anticlockwise(x, y, radius, start_angle, end_angle, anticlockwise);
        }

        // pub fn arc_with_anticlockwise(
        //     &self,
        //     x: f64,
        //     y: f64,
        //     radius: f64,
        //     start_angle: f64,
        //     end_angle: f64,
        //     anticlockwise: bool
        // ) -> Result<(), JsValue>
    }

    fn arc_to(&self, x1: f64, y1: f64, x2: f64, y2: f64, radius: f64) {
        let _ = self.ctx.arc_to(x1, y1, x2, y2, radius);
    }

    fn begin_path(&self) {
        self.ctx.begin_path()
    }

    fn bezier_curve_to(&self, cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64) {
        self.ctx.bezier_curve_to(cp1x, cp1y, cp2x, cp2y, x, y);
    }

    fn clear_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.clear_rect(x, y, width, height);
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
        // TODO: anticlockwise
        let _ = self
            .ctx
            .ellipse(x, y, radius_x, radius_y, rotation, start_angle, end_angle);

        // fn ellipse_with_anticlockwise(
        //     &self,
        //     x: f64,
        //     y: f64,
        //     radius_x: f64,
        //     radius_y: f64,
        //     rotation: f64,
        //     start_angle: f64,
        //     end_angle: f64,
        //     anticlockwise: bool
        // ) -> Result<(), JsValue>
    }

    // [dynamic path_OR_winding, String? winding]
    // fn fill(path_OR_winding: dynamic, winding: String); // TODO:

    fn fill(&self) {
        self.ctx.fill();
    }

    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.fill_rect(x, y, width, height);
    }

    // Draws text to the canvas.
    fn fill_text(&self, text: &str, x: f64, y: f64) {
        let _ = self.ctx.fill_text(text, x, y);
    }

    // fn getContextAttributes() -> Map; // TODO:

    // @Creates('ImageData|=Object')
    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData; // TODO:

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash(&self) -> Vec<f64> {
        // let _ = self.get_line_dash(); // TODO: complete it
        unimplemented!()
    }

    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool; // TODO:

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool; // TODO:
    fn line_to(&self, x: f64, y: f64) {
        self.ctx.line_to(x, y);
    }

    fn measure_text(&self, text: &str) -> TextMetrics {
        match self.ctx.measure_text(text) {
            Ok(metric) => TextMetrics {
                width: metric.width(),
                height: -1.,
            },
            Err(err) => {
                info!("{:?}", err);
                TextMetrics {
                    width: -1.,
                    height: -1.,
                }
            }
        }
    }

    fn move_to(&self, x: f64, y: f64) {
        self.ctx.move_to(x, y);
    }

    // [int? dirtyX, int? dirtyY, int? dirtyWidth, int? dirtyHeight]
    // fn putImageData(imagedata: ImageData, dx: i64, dy: i64, dirtyX: i64, dirtyY: i64, dirtyWidth: i64, dirtyHeight: i64); // TODO:
    fn quadratic_curve_to(&self, cpx: f64, cpy: f64, x: f64, y: f64) {
        self.ctx.quadratic_curve_to(cpx, cpy, x, y);
    }

    fn rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.rect(x, y, width, height);
    }

    fn reset_transform(&self) {
        let _ = self.ctx.reset_transform();
    }

    fn restore(&self) {
        self.ctx.restore();
    }

    fn rotate(&self, angle: f64) {
        let _ = self.ctx.rotate(angle);
    }

    fn save(&self) {
        self.ctx.save();
    }

    fn scale(&self, x: f64, y: f64) {
        let _ = self.ctx.scale(x, y);
    }

    // [Path2D? path]
    // fn scrollPathIntoView(path: Path2D); // TODO:

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn set_line_dash(&self, dash: &Vec<f64>) {
        // TODO: should complete from JsValue for web-sys
    }

    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let _ = self.ctx.set_transform(a, b, c, d, e, f);
    }

    fn stroke(&self) {
        self.ctx.stroke();
    }

    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.stroke_rect(x, y, width, height);
    }

    fn stroke_text(&self, text: &str, x: f64, y: f64) {
        let _ = self.ctx.stroke_text(text, x, y);
    }

    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let _ = self.ctx.transform(a, b, c, d, e, f);
    }

    fn translate(&self, x: f64, y: f64) {
        let _ = self.ctx.translate(x, y);
    }
}
