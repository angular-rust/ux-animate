#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![cfg(feature = "web")]

use ux_primitives::{
    canvas::{CanvasContext, Direction},
    geom::{Point, Size, Rect},
    color::Color
};

use web_sys;

pub struct WebCanvas {
    ctx: web_sys::CanvasRenderingContext2d,
}

impl WebCanvas {
    pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            ctx
        }
    }
}

impl CanvasContext for WebCanvas {
    // Properties

    // Deprecated always returns 1.0
    fn get_backing_store_pixel_ratio(&mut self) -> f64 {
        unimplemented!() // FIXME: not implemented in web_sys
    }
    fn set_backing_store_pixel_ratio(&mut self, value: f64) {
        unimplemented!() // FIXME: not implemented in web_sys
    }

    // fn get_canvas(&self) -> CanvasElement;

    // fn get_transform(&self) -> Matrix {
    //    self.ctx.get_transform() -> Result<DomMatrix, JsValue>
    // };
    // fn set_current_transform(&mut self, value: Matrix<f64>) {
    //     let _ = self.ctx.set_transform(value.a, value.b, value.c, value.d, value.e, value.f);
    // }
    fn get_direction(&self) -> Direction {
        unimplemented!() // FIXME: not implemented in web_sys
    }
    fn set_direction(&self, value: Direction) -> String {
        unimplemented!() // FIXME: not implemented in web_sys
    }

    // @Creates('String|CanvasGradient|CanvasPattern'), @Returns('String|CanvasGradient|CanvasPattern')
    // fillStyle: Object;
    // fn get_fill_style(&self) -> Box<CanvasStyle<dyn CanvasGradientInterface, dyn CanvasPatternInterface>>;
    // fn set_fill_style(&mut self, value: CanvasStyle<impl CanvasGradientInterface, impl CanvasPatternInterface>) {
    //     unimplemented!()
    // }
    fn set_fill_style_color(&mut self, value: Color) {
        unimplemented!()
    }
    // fn set_fill_style_gradient(&mut self, value: impl CanvasGradientInterface) {
    //     unimplemented!()
    // }
    // fn set_fill_style_pattern(&mut self, value: impl CanvasPatternInterface) {
    //     unimplemented!()
    // }

    fn get_filter(&self) -> String {
        self.ctx.filter()
    }
    fn set_filter(&mut self, value: &str) {
        self.ctx.set_filter(value);
    }

    fn get_font(&self) -> String {
        self.ctx.font()
    }
    fn set_font(&mut self, value: &str) {
        self.ctx.set_font(value)
    }

    fn get_global_alpha(&self) -> f64 {
        self.ctx.global_alpha()
    }
    fn set_global_alpha(&mut self, value: f64) {
        self.ctx.set_global_alpha(value);
    }

    fn get_global_composite_operation(&self) -> String {
        // TODO: handle err
        self.ctx.global_composite_operation().unwrap()
    }
    fn set_global_composite_operation(&mut self, value: &str) {
        // TODO: handle err
        let _ = self.ctx.set_global_composite_operation(value);
    }

    // // The hash code for this object. 
    // fn get_hash_code(&self) -> u64 {
    //     unimplemented!()
    // }
    // fn set_hash_code(&mut self, value: u64) {
    //     unimplemented!()
    // }

    // Whether images and patterns on this canvas will be smoothed when this canvas is scaled. 
    // imageSmoothingEnabled
    fn is_image_smoothing_enabled(&self) -> bool {
        self.ctx.image_smoothing_enabled()
    }
    fn set_image_smoothing(&mut self, value: bool) {
        self.ctx.set_image_smoothing_enabled(value);
    }

    // fn get_image_smoothing_quality(&self) -> String {
    //     unimplemented!()
    // }
    // fn set_image_smoothing_quality(&mut self, value: String) {
    //     unimplemented!()
    // }

    fn get_line_cap(&self) -> String {
        self.ctx.line_cap()
    }
    fn set_line_cap(&mut self, value: &str) {
        self.ctx.set_line_cap(value);
    }

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash_offset(&self) -> f64 {
        self.ctx.line_dash_offset()
    }
    fn set_line_dash_offset(&mut self, value: f64) {
        self.ctx.set_line_dash_offset(value);
    }

    fn get_line_join(&self) -> String {
        self.ctx.line_join()
    }
    fn set_line_join(&mut self, value: &str) {
        self.ctx.set_line_join(value)
    }

    fn get_line_width(&self) -> f64 {
        self.ctx.line_width()
    }
    fn set_line_width(&mut self, value: f64) {
        self.ctx.set_line_width(value);
    }

    fn get_miter_limit(&self) -> f64 {
        self.ctx.miter_limit()
    }
    fn set_miter_limit(&mut self, value: f64) {
        self.ctx.set_miter_limit(value);
    }

    // // A representation of the runtime type of the object.
    // runtimeType: Type; // TODO

    fn get_shadow_blur(&self) -> f64 {
        self.ctx.shadow_blur()
    }
    fn set_shadow_blur(&mut self, value: f64) {
        self.ctx.set_shadow_blur(value);
    }

    fn get_shadow_color(&self) -> String {
        self.ctx.shadow_color()
    }
    fn set_shadow_color(&mut self, value: &str) {
        self.ctx.set_shadow_color(value);
    }

    fn get_shadow_offset_x(&self) -> f64 {
        self.ctx.shadow_offset_x()
    }
    fn set_shadow_offset_x(&mut self, value: f64) {
        self.ctx.set_shadow_offset_x(value);
    }

    fn get_shadow_offset_y(&self) -> f64 {
        self.ctx.shadow_offset_y()
    }
    fn set_shadow_offset_y(&mut self, value: f64) {
        self.ctx.set_shadow_offset_y(value);
    }

    // @Creates('String|CanvasGradient|CanvasPattern'), @Returns('String|CanvasGradient|CanvasPattern')
    // fn set_stroke_style(&mut self, value: CanvasStyle<impl CanvasGradientInterface, impl CanvasPatternInterface>) {
    //     unimplemented!()
    // }
    fn set_stroke_style_color(&mut self, value: Color) {
        unimplemented!() // TODO: complete it
    }
    // fn set_stroke_style_gradient(&mut self, value: impl CanvasGradientInterface) {
    //     unimplemented!()
    // }
    // fn set_stroke_style_pattern(&mut self, value: impl CanvasPatternInterface) {
    //     unimplemented!()
    // }

    fn get_text_align(&mut self) -> String {
        self.ctx.text_align()
    }
    fn set_text_align(&mut self, value: &str) {
        self.ctx.set_text_align(value);
    }

    fn get_text_baseline(&self) -> String {
        self.ctx.text_baseline()
    }
    fn set_text_baseline(&mut self, value: &str) {
        self.ctx.set_text_baseline(value);
    }

    // Methods
    // fn add_hit_region(options: Map); // TODO:

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
        // TODO: compele clockwise
        let _ = self.ctx.arc(x, y, radius, start_angle, end_angle);

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
    fn clear_hit_regions(&self) {
        self.ctx.clear_hit_regions();
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
    // fn createLinearGradient(x0: f64, y0: f64, x1: f64, y1: f64) -> CanvasGradient; // TODO:
    // fn createPattern(image: Object, repetitionType: String) -> CanvasPattern; // TODO:
    // fn createPatternFromImage(image: ImageElement, repetitionType: String) -> CanvasPattern; // TODO:
    // fn createRadialGradient(x0: f64, y0: f64, r0: f64, x1: f64, y1: f64, r1: f64) -> CanvasGradient; // TODO:

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
        let _ = self.ctx.ellipse(x, y, radius_x, radius_y, rotation, start_angle, end_angle);

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
    fn fill_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.fill_rect(x, y, width, height);
    }

    // Draws text to the canvas. 
    // [f64? max_width]
    fn fill_text(&self, text: &str, x: f64, y: f64, max_width: f64) {
        // TODO: max_width
        let _ = self.ctx.fill_text(text, x, y);
        // pub fn fill_text_with_max_width(
        //     &self,
        //     text: &str,
        //     x: f64,
        //     y: f64,
        //     max_width: f64
        // ) -> Result<(), JsValue>
    }

    // fn getContextAttributes() -> Map; // TODO:

    // @Creates('ImageData|=Object')
    // fn getImageData(sx: i64, sy: i64, sw: i64, sh: i64) -> ImageData; // TODO:

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn get_line_dash(&self) -> Vec<f64> {
        // let _ = self.get_line_dash(); // TODO: complete it
        unimplemented!()
    }

    fn is_context_lost(&self) -> bool {
        unimplemented!() // TODO: there are no web-sys impl
    }
    // [dynamic winding_OR_y, String? winding]
    // fn isPointInPath(path_OR_x: dynamic, x_OR_y: f64, winding_OR_y: dynamic, winding: String) -> bool; // TODO:

    // [f64? y]
    // fn isPointInStroke(path_OR_x: dynamic, x_OR_y: f64, y: f64) -> bool; // TODO:
    fn line_to(&self, x: f64, y: f64) {
        self.ctx.line_to(x, y);
    }
    // fn measureText(text: String) -> TextMetrics; // TODO:
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
    fn remove_hit_region(&self, id: &str) {
        self.ctx.remove_hit_region(id);
    }
    fn reset_transform(&self) {
        // TODO: complete it
        let _ = self.ctx.reset_transform();
    }
    fn restore(&self) {
        self.ctx.restore();
    }
    fn rotate(&self, angle: f64) {
        // TODO: complete it
        let _ = self.ctx.rotate(angle);
    }
    fn save(&self) {
        self.ctx.save();
    }
    fn scale(&self, x: f64, y: f64) {
        unimplemented!()
    }

    // [Path2D? path]
    // fn scrollPathIntoView(path: Path2D); // TODO:

    // Sets the color used inside shapes. h is in degrees, 0-360. s, l are in percent, 0-100. a is 0-1.
    // [f64 a = 1]
    fn set_fill_color_hsl(&self, h: i64, s: f64, l: f64, a: f64) {
        unimplemented!() // TODO: web-sys has no impl
    }

    // Sets the color used inside shapes. r, g, b are 0-255, a is 0-1.
    // [f64 a = 1]
    fn set_fill_color_rgb(&self, r: i64, g: i64, b: i64, a: f64) {
        unimplemented!() // TODO: web-sys has no impl
    }

    // @SupportedBrowser(SupportedBrowser.CHROME), @SupportedBrowser(SupportedBrowser.IE, '11'), @SupportedBrowser(SupportedBrowser.SAFARI), @Unstable()
    fn set_line_dash(&self, dash: Vec<f64>) {
        unimplemented!() // TODO: should complete from JsValue for web-sys
    }

    // Sets the color used for stroking shapes. h is in degrees, 0-360. s, l are in percent, 0-100. a is 0-1.
    // [f64 a = 1]
    fn set_stroke_color_hsl(&self, h: i64, s: f64, l: f64, a: f64) {
        unimplemented!() // TODO: web-sys has no impl
    }

    // Sets the color used for stroking shapes. r, g, b are 0-255, a is 0-1.
    // [f64 a = 1]
    fn set_stroke_color_rgb(&self, r: i64, g: i64, b: i64, a: f64) {
        unimplemented!() // TODO: web-sys has no impl
    }

    fn set_transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        // TODO: complete it
        let _ = self.ctx.set_transform(a, b, c, d, e, f);
    }

    fn stroke(&self) {
        unimplemented!()
    }
    
    fn stroke_rect(&self, x: f64, y: f64, width: f64, height: f64) {
        self.ctx.stroke_rect(x, y, width, height);
    }
    // [f64? max_width]
    fn stroke_text(&self, text: &str, x: f64, y: f64, max_width: f64) {
        // TODO: complete it
        let _ = self.ctx.stroke_text(text, x, y);
        // fn stroke_text_with_max_width(
        //     &self,
        //     text: &str,
        //     x: f64,
        //     y: f64,
        //     max_width: f64
        // ) -> Result<(), JsValue>
    }

    fn transform(&self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        // TODO: complete it
        let _ = self.ctx.transform(a, b, c, d, e, f);
    }
    fn translate(&self, x: f64, y: f64) {
        let _ = self.ctx.translate(x, y);
    }
}
