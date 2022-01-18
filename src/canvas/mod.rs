#![allow(clippy::too_many_arguments)]
use primitives::prelude::CanvasContext;
use std::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
mod webimpl;
#[cfg(target_arch = "wasm32")]
pub use webimpl::*;

// #[cfg(not(target_arch = "wasm32"))]
// mod cairoimpl;
// #[cfg(not(target_arch = "wasm32"))]
// pub use cairoimpl::*;

// mod rough;
// pub use rough::*;

pub trait AdvancedShapesExt {
    //*
    //  * Create a rectangle
    //  * Optional with rounded corners if radius is set
    //  * @param x position on the x axis in pixel
    //  * @param y position on the y axis in pixel
    //  * @param w height of the rectangle in pixel
    //  * @param h width of the rectangle in pixel
    //  * @param r (optional) radius for rounded corners
    //  * @return rectangle item instance
    //  */
    fn round_rect(&self, x: f64, y: f64, width: f64, height: f64, radius: f64);
    //*
    //  * Create a symmetric polygon
    //  * @param x Position on x-axis in pixel
    //  * @param y Position on y-axis in pixel
    //  * @param n Number of edges
    //  * @param r Radius of the polygon
    //  * @return polygon item instance
    //  */
    fn polygon(&self, x: f64, y: f64, radius: f64, n: usize);
    //*
    //  * Create a path
    //  * @param x start of the path at position x in pixel
    //  * @param y start of the path at position y in pixel
    //  * @return path item instance
    //  */
    // fn path(&self, x: f64, y: f64, width: f64, height: f64, radius: f64);

    // centerX, centerY: the center point of the star
    // points: the number of points on the exterior of the star
    // inner: the radius of the inner points of the star
    // outer: the radius of the outer points of the star
    // fill, stroke: the fill and stroke colors to apply
    // line: the linewidth of the stroke
    fn star(&self, x: f64, y: f64, points: usize, outer: f64, inner: f64);
}

impl<O> AdvancedShapesExt for O
where
    O: CanvasContext,
{
    fn round_rect(&self, x: f64, y: f64, width: f64, height: f64, radius: f64) {
        if radius > 0.0 {
            // self.begin_path();
            // self.move_to(x+r, y+0);
            // self.arc_to(x+w, y+0,       x+this.rect.w,  y+this.rect.r, this.rect.r);
            // self.arc_to(x+w, y+height,  x+this.rect.r,  y+this.rect.h, this.rect.r);
            // self.arc_to(x+0, y+height,  x+0,            y+this.rect.r, this.rect.r);
            // self.arc_to(x+0, y+0,       x+this.rect.r,  y+0, this.rect.r);
            // self.close_path();

            // self.save();
            self.begin_path();
            self.move_to(x + radius, y);
            self.line_to(x + width - radius, y);
            self.quadratic_curve_to(x + width, y, x + width, y + radius);
            self.line_to(x + width, y + height - radius);
            self.quadratic_curve_to(x + width, y + height, x + width - radius, y + height);
            self.line_to(x + radius, y + height);
            self.quadratic_curve_to(x, y + height, x, y + height - radius);
            self.line_to(x, y + radius);
            self.quadratic_curve_to(x, y, x + radius, y);
            self.close_path();
            // ctx.restore();
        } else {
            self.begin_path(); // FIXME: ??
            self.rect(x, y, width, height);
        }
    }

    fn polygon(&self, x: f64, y: f64, radius: f64, n: usize) {
        self.move_to(x, y);
        self.begin_path();
        let angle = PI * 2.0 / n as f64;
        for idx in 0..n {
            let px = x + radius * (idx as f64 * angle).cos();
            let py = y + radius * (idx as f64 * angle).sin();
            self.line_to(px, py);
        }
        self.close_path();
    }

    fn star(&self, x: f64, y: f64, points: usize, outer: f64, inner: f64) {
        // define the star
        self.begin_path();
        self.move_to(x, y + outer);
        for idx in 0..2 * points + 1 {
            let radius = if idx % 2 == 0 { outer } else { inner };
            let angle = PI * idx as f64 / points as f64;
            self.line_to(x + radius * (angle).sin(), y + radius * (angle).cos());
        }
        self.close_path();
    }
}
