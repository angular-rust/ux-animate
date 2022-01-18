#![allow(unused_imports)]
use super::{Actor, Content, HandlerId};
use crate::prelude::*;
use std::fmt;

// @implements Content
#[derive(Default, Debug, Clone)]
pub struct Image {}

impl Image {
    /// Creates a new `Image` instance.
    ///
    /// # Returns
    ///
    /// the newly created `Image` instance.
    ///  Use `gobject::ObjectExt::unref` when done.
    pub fn new() -> Option<Content> {
        // unsafe { from_glib_full(ffi::image_new()) }
        unimplemented!()
    }
}

impl Object for Image {}
impl Is<Image> for Image {}

impl AsRef<Image> for Image {
    fn as_ref(&self) -> &Image {
        self
    }
}

/// Trait containing all `Image` methods.
///
/// # Implementors
///
/// [`Image`](struct.Image.html)
pub trait ImageExt: 'static {
    //fn set_area(&self, data: &[u8], pixel_format: dx::pure::PixelFormat, rect: &cairo::RectangleInt, row_stride: u32) -> Result<(), glib::Error>;

    // /// Sets the image data stored inside a `glib::Bytes` to be displayed by `self`.
    // ///
    // /// If the image data was successfully loaded, the `self` will be invalidated.
    // ///
    // /// In case of error, the `error` value will be set, and this function will
    // /// return `false`.
    // ///
    // /// The image data contained inside the `glib::Bytes` is copied in texture memory,
    // /// and no additional reference is acquired on the `data`.
    // /// ## `data`
    // /// the image data, as a `glib::Bytes`
    // /// ## `pixel_format`
    // /// the Cogl pixel format of the image data
    // /// ## `width`
    // /// the width of the image data
    // /// ## `height`
    // /// the height of the image data
    // /// ## `row_stride`
    // /// the length of each row inside `data`
    // ///
    // /// # Returns
    // ///
    // /// `true` if the image data was successfully loaded,
    // ///  and `false` otherwise.
    // fn set_bytes(
    //     &self,
    //     data: &glib::Bytes,
    //     pixel_format: dx::PixelFormat,
    //     width: u32,
    //     height: u32,
    //     row_stride: u32,
    // ) -> Result<(), glib::Error>;

    //fn set_data(&self, data: &[u8], pixel_format: dx::pure::PixelFormat, width: u32, height: u32, row_stride: u32) -> Result<(), glib::Error>;
}

impl ContentExt for Image {
    fn get_preferred_size(&self) -> Option<(f32, f32)> {
        unimplemented!()
    }

    fn invalidate(&self) {
        unimplemented!()
    }

    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl<O: Is<Image>> ImageExt for O {
    //fn set_area(&self, data: &[u8], pixel_format: dx::pure::PixelFormat, rect: &cairo::RectangleInt, row_stride: u32) -> Result<(), glib::Error> {
    //    unsafe { TODO: call sys:image_set_area() }
    //}

    // fn set_bytes(
    //     &self,
    //     data: &glib::Bytes,
    //     pixel_format: dx::PixelFormat,
    //     width: u32,
    //     height: u32,
    //     row_stride: u32,
    // ) -> Result<(), glib::Error> {
    //     // unsafe {
    //     //     let mut error = ptr::null_mut();
    //     //     let _ = ffi::image_set_bytes(
    //     //         self.as_ref().to_glib_none().0,
    //     //         data.to_glib_none().0,
    //     //         pixel_format.to_glib(),
    //     //         width,
    //     //         height,
    //     //         row_stride,
    //     //         &mut error,
    //     //     );
    //     //     if error.is_null() {
    //     //         Ok(())
    //     //     } else {
    //     //         Err(from_glib_full(error))
    //     //     }
    //     // }
    //     unimplemented!()
    // }

    //fn set_data(&self, data: &[u8], pixel_format: dx::pure::PixelFormat, width: u32, height: u32, row_stride: u32) -> Result<(), glib::Error> {
    //    unsafe { TODO: call sys:image_set_data() }
    //}
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image")
    }
}
