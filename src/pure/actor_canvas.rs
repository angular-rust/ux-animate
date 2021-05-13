use super::{Content, HandlerId};
use crate::prelude::*;
use std::fmt;
// use cairo::Context;

// * Canvas:
// *
// * The #Canvas structure contains
// * private data and should only be accessed using the provided
// * API.
// * @Title: Canvas
// * @Short_Description: Content for 2D painting
// * @See_Also: #Content
// *
// * The #Canvas class is a #Content implementation that allows
// * drawing using the Cairo API on a 2D surface.
// *
// * In order to draw on a #Canvas, you should connect a handler to the
// * #Canvas::draw signal; the signal will receive a #cairo_t context
// * that can be used to draw. #Canvas will emit the #Canvas::draw
// * signal when invalidated using clutter_content_invalidate().
// *
// * See [canvas.c](https://git.gnome.org/browse/clutter/tree/examples/canvas.c?h=clutter-1.18)
// * for an example of how to use #Canvas.
// @implements Content
pub struct ActorCanvas {
    cr: cairo::Context,

    width: u32,
    height: u32,

    texture: dx::pure::Texture,
    dirty: bool,

    buffer: dx::pure::Bitmap,

    scale_factor: u32,
    scale_factor_set: bool,
}

impl ActorCanvas {
    /// Creates a new instance of `Canvas`.
    ///
    /// You should call `CanvasExt::set_size` to set the size of the canvas.
    ///
    /// You should call `Content::invalidate` every time you wish to
    /// draw the contents of the canvas.
    ///
    /// # Returns
    ///
    /// The newly allocated instance of
    ///  `Canvas`. Use `gobject::ObjectExt::unref` when done.
    pub fn new() -> Option<Content> {
        // Self {

        // }
        unimplemented!()
    }
}

impl Object for ActorCanvas {}
impl Is<ActorCanvas> for ActorCanvas {}

impl AsRef<ActorCanvas> for ActorCanvas {
    fn as_ref(&self) -> &ActorCanvas {
        self
    }
}

/// Trait containing all `Canvas` methods.
///
/// # Implementors
///
/// [`Canvas`](struct.Canvas.html)
pub trait ActorCanvasExt: 'static {
    /// Retrieves the scaling factor of `self`, as set using
    /// `CanvasExt::set_scale_factor`.
    ///
    /// # Returns
    ///
    /// the scaling factor, or -1 if the `self`
    ///  uses the default from `Settings`
    fn get_scale_factor(&self) -> i32;

    /// Sets the scaling factor for the Cairo surface used by `self`.
    ///
    /// This function should rarely be used.
    ///
    /// The default scaling factor of a `Canvas` content uses the
    /// `Settings:window-scaling-factor` property, which is set by
    /// the windowing system. By using this function it is possible to
    /// override that setting.
    ///
    /// Changing the scale factor will invalidate the `self`.
    /// ## `scale`
    /// the scale factor, or -1 for the default
    fn set_scale_factor(&self, scale: i32);

    /// Sets the size of the `self`, and invalidates the content.
    ///
    /// This function will cause the `self` to be invalidated only
    /// if the size of the canvas surface has changed.
    ///
    /// If you want to invalidate the contents of the `self` when setting
    /// the size, you can use the return value of the function to conditionally
    /// call `Content::invalidate`:
    ///
    ///
    /// ```text
    ///   if (!canvas_set_size (canvas, width, height))
    ///     content_invalidate (CONTENT (canvas));
    /// ```
    /// ## `width`
    /// the width of the canvas, in pixels
    /// ## `height`
    /// the height of the canvas, in pixels
    ///
    /// # Returns
    ///
    /// this function returns `true` if the size change
    ///  caused a content invalidation, and `false` otherwise
    fn set_size(&self, width: u32, height: u32) -> bool;

    /// The height of the canvas.
    fn get_property_height(&self) -> u32;

    /// The height of the canvas.
    fn set_property_height(&self, height: u32);

    /// Whether the `Canvas:scale-factor` property is set.
    ///
    /// If the `Canvas:scale-factor-set` property is `false`
    /// then `Canvas` will use the `Settings:window-scaling-factor`
    /// property.
    fn get_property_scale_factor_set(&self) -> bool;

    /// The width of the canvas.
    fn get_property_width(&self) -> u32;

    /// The width of the canvas.
    fn set_property_width(&self, width: u32);

    /// The `Canvas::draw` signal is emitted each time a canvas is
    /// invalidated.
    ///
    /// It is safe to connect multiple handlers to this signal: each
    /// handler invocation will be automatically protected by `cairo_save`
    /// and `cairo_restore` pairs.
    /// ## `cr`
    /// the Cairo context used to draw
    /// ## `width`
    /// the width of the `canvas`
    /// ## `height`
    /// the height of the `canvas`
    ///
    /// # Returns
    ///
    /// `true` if the signal emission should stop, and
    ///  `false` otherwise
    fn connect_draw<F: Fn(&Self, &cairo::Context, u32, u32) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_scale_factor_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<ActorCanvas>> ActorCanvasExt for O {
    fn get_scale_factor(&self) -> i32 {
        // let canvas = self.as_ref();
        // if !canvas.scale_factor_set {
        //     return -1;
        // }

        // canvas.scale_factor
        unimplemented!()
    }

    fn set_scale_factor(&self, scale: i32) {
        // let mut canvas = self.as_ref();
        // if scale < 0 {
        //     if !canvas.scale_factor_set {
        //         return
        //     }
        //     canvas.scale_factor_set = false;
        //     canvas.scale_factor = scale;
        // } else {
        //     if canvas.scale_factor_set && canvas.scale_factor == scale {
        //         return
        //     }
        //     canvas.scale_factor_set = true;
        //     canvas.scale_factor = scale;
        // }
        // // invalidate // TODO;
        unimplemented!()
    }

    fn set_size(&self, width: u32, height: u32) -> bool {
        // let canvas = self.as_ref();
        // let mut width_changed = false;
        // let mut height_changed = false;

        // if canvas.width != width {
        //     canvas.width = width;
        //     width_changed = true;
        // }

        // if canvas.height != height {
        //     canvas.height = height;
        //     height_changed = true;
        // }

        // if width_changed || height_changed {
        //     // invalidate // TODO:
        //     return true;
        // }

        // return false;
        unimplemented!()
    }

    fn get_property_height(&self) -> u32 {
        let canvas = self.as_ref();
        canvas.height
    }

    fn set_property_height(&self, height: u32) {
        unimplemented!()
    }

    fn get_property_scale_factor_set(&self) -> bool {
        let canvas = self.as_ref();
        canvas.scale_factor_set
    }

    fn get_property_width(&self) -> u32 {
        let canvas = self.as_ref();
        canvas.width
    }

    fn set_property_width(&self, width: u32) {
        unimplemented!()
    }

    fn connect_draw<F: Fn(&Self, &cairo::Context, u32, u32) -> bool + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_scale_factor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_scale_factor_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for ActorCanvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Canvas")
    }
}
