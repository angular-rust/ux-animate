use crate::StaticColor;
use glib::{translate::*, GString};
use std::{fmt, hash, mem};

glib_wrapper! {
    #[derive(Debug)] // PartialOrd, Ord
    pub struct InternalColor(Boxed<ffi::ClutterColor>);

    match fn {
        copy => |ptr| ffi::clutter_color_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_color_free(ptr),
        get_type => || ffi::clutter_color_get_type(),
    }
}

impl InternalColor {
    /// Allocates a new, transparent black `Color`.
    ///
    /// # Returns
    ///
    /// the newly allocated `Color`; use
    ///  `Color::free` to free its resources
    pub fn alloc() -> InternalColor {
        unsafe { from_glib_full(ffi::clutter_color_alloc()) }
    }

    /// Creates a new `Color` with the given values.
    ///
    /// This function is the equivalent of:
    ///
    ///
    /// ```text
    ///   color_init (color_alloc (), red, green, blue, alpha);
    /// ```
    /// ## `red`
    /// red component of the color, between 0 and 255
    /// ## `green`
    /// green component of the color, between 0 and 255
    /// ## `blue`
    /// blue component of the color, between 0 and 255
    /// ## `alpha`
    /// alpha component of the color, between 0 and 255
    ///
    /// # Returns
    ///
    /// the newly allocated color.
    ///  Use `Color::free` when done
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> InternalColor {
        unsafe { from_glib_full(ffi::clutter_color_new(red, green, blue, alpha)) }
    }

    /// Adds `self` to `b` and saves the resulting color inside `result`.
    ///
    /// The alpha channel of `result` is set as as the maximum value
    /// between the alpha channels of `self` and `b`.
    /// ## `b`
    /// a `Color`
    /// ## `result`
    /// return location for the result
    pub fn add(&self, b: &InternalColor) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_add(
                self.to_glib_none().0,
                b.to_glib_none().0,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    /// Darkens `self` by a fixed amount, and saves the changed color
    /// in `result`.
    /// ## `result`
    /// return location for the darker color
    pub fn darken(&self) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_darken(self.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    // /// Compares two `Color`<!-- -->s and checks if they are the same.
    // ///
    // /// This function can be passed to `glib::HashTable::new` as the `key_equal_func`
    // /// parameter, when using `Color`<!-- -->s as keys in a `glib::HashTable`.
    // /// ## `v2`
    // /// a `Color`
    // ///
    // /// # Returns
    // ///
    // /// `true` if the two colors are the same.
    // fn equal(&self, v2: &Color) -> bool {
    //     unsafe {
    //         from_glib(ffi::clutter_color_equal(
    //             ToGlibPtr::<*mut ffi::ClutterColor>::to_glib_none(self).0
    //                 as glib_sys::gconstpointer,
    //             ToGlibPtr::<*mut ffi::ClutterColor>::to_glib_none(v2).0 as glib_sys::gconstpointer,
    //         ))
    //     }
    // }

    // /// Converts a `Color` to a hash value.
    // ///
    // /// This function can be passed to `glib::HashTable::new` as the `hash_func`
    // /// parameter, when using `Color`<!-- -->s as keys in a `glib::HashTable`.
    // ///
    // /// # Returns
    // ///
    // /// a hash value corresponding to the color
    // fn hash(&self) -> u32 {
    //     unsafe {
    //         ffi::clutter_color_hash(
    //             ToGlibPtr::<*mut ffi::ClutterColor>::to_glib_none(self).0
    //                 as glib_sys::gconstpointer,
    //         )
    //     }
    // }

    /// Initializes `self` with the given values.
    /// ## `red`
    /// red component of the color, between 0 and 255
    /// ## `green`
    /// green component of the color, between 0 and 255
    /// ## `blue`
    /// blue component of the color, between 0 and 255
    /// ## `alpha`
    /// alpha component of the color, between 0 and 255
    ///
    /// # Returns
    ///
    /// the initialized `Color`
    pub fn init(&mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Option<InternalColor> {
        unsafe {
            from_glib_none(ffi::clutter_color_init(
                self.to_glib_none_mut().0,
                red,
                green,
                blue,
                alpha,
            ))
        }
    }

    /// Interpolates between `self` and `final_` `Color`<!-- -->s
    /// using `progress`
    /// ## `final_`
    /// the final `Color`
    /// ## `progress`
    /// the interpolation progress
    /// ## `result`
    /// return location for the interpolation
    pub fn interpolate(&self, final_: &InternalColor, progress: f64) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_interpolate(
                self.to_glib_none().0,
                final_.to_glib_none().0,
                progress,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    /// Lightens `self` by a fixed amount, and saves the changed color
    /// in `result`.
    /// ## `result`
    /// return location for the lighter color
    pub fn lighten(&self) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_lighten(self.to_glib_none().0, result.to_glib_none_mut().0);
            result
        }
    }

    /// Shades `self` by `factor` and saves the modified color into `result`.
    /// ## `factor`
    /// the shade factor to apply
    /// ## `result`
    /// return location for the shaded color
    pub fn shade(&self, factor: f64) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_shade(self.to_glib_none().0, factor, result.to_glib_none_mut().0);
            result
        }
    }

    /// Subtracts `b` from `self` and saves the resulting color inside `result`.
    ///
    /// This function assumes that the components of `self` are greater than the
    /// components of `b`; the result is, otherwise, undefined.
    ///
    /// The alpha channel of `result` is set as the minimum value
    /// between the alpha channels of `self` and `b`.
    /// ## `b`
    /// a `Color`
    /// ## `result`
    /// return location for the result
    pub fn subtract(&self, b: &InternalColor) -> InternalColor {
        unsafe {
            let mut result = InternalColor::uninitialized();
            ffi::clutter_color_subtract(
                self.to_glib_none().0,
                b.to_glib_none().0,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    /// Converts `self` to the HLS format.
    ///
    /// The `hue` value is in the 0 .. 360 range. The `luminance` and
    /// `saturation` values are in the 0 .. 1 range.
    /// ## `hue`
    /// return location for the hue value or `None`
    /// ## `luminance`
    /// return location for the luminance value or `None`
    /// ## `saturation`
    /// return location for the saturation value or `None`
    pub fn to_hls(&self) -> (f32, f32, f32) {
        unsafe {
            let mut hue = mem::MaybeUninit::uninit();
            let mut luminance = mem::MaybeUninit::uninit();
            let mut saturation = mem::MaybeUninit::uninit();
            ffi::clutter_color_to_hls(
                self.to_glib_none().0,
                hue.as_mut_ptr(),
                luminance.as_mut_ptr(),
                saturation.as_mut_ptr(),
            );
            let hue = hue.assume_init();
            let luminance = luminance.assume_init();
            let saturation = saturation.assume_init();
            (hue, luminance, saturation)
        }
    }

    /// Converts `self` into a packed 32 bit integer, containing
    /// all the four 8 bit channels used by `Color`.
    ///
    /// # Returns
    ///
    /// a packed color
    pub fn to_pixel(&self) -> u32 {
        unsafe { ffi::clutter_color_to_pixel(self.to_glib_none().0) }
    }

    /// Returns a textual specification of `self` in the hexadecimal form
    /// `<literal>`&num;rrggbbaa`</literal>`, where `<literal>`r`</literal>`,
    /// `<literal>`g`</literal>`, `<literal>`b`</literal>` and `<literal>`a`</literal>` are
    /// hexadecimal digits representing the red, green, blue and alpha components
    /// respectively.
    ///
    /// # Returns
    ///
    /// a newly-allocated text string
    fn to_string(&self) -> GString {
        unsafe { from_glib_full(ffi::clutter_color_to_string(self.to_glib_none().0)) }
    }

    /// Converts a color expressed in HLS (hue, luminance and saturation)
    /// values into a `Color`.
    /// ## `color`
    /// return location for a `Color`
    /// ## `hue`
    /// hue value, in the 0 .. 360 range
    /// ## `luminance`
    /// luminance value, in the 0 .. 1 range
    /// ## `saturation`
    /// saturation value, in the 0 .. 1 range
    pub fn from_hls(hue: f32, luminance: f32, saturation: f32) -> InternalColor {
        unsafe {
            let mut color = InternalColor::uninitialized();
            ffi::clutter_color_from_hls(color.to_glib_none_mut().0, hue, luminance, saturation);
            color
        }
    }

    /// Converts `pixel` from the packed representation of a four 8 bit channel
    /// color to a `Color`.
    /// ## `color`
    /// return location for a `Color`
    /// ## `pixel`
    /// a 32 bit packed integer containing a color
    pub fn from_pixel(pixel: u32) -> InternalColor {
        unsafe {
            let mut color = InternalColor::uninitialized();
            ffi::clutter_color_from_pixel(color.to_glib_none_mut().0, pixel);
            color
        }
    }

    /// Parses a string definition of a color, filling the `Color.red`,
    /// `Color.green`, `Color.blue` and `Color.alpha` fields
    /// of `color`.
    ///
    /// The `color` is not allocated.
    ///
    /// The format of `str` can be either one of:
    ///
    ///  - a standard name (as taken from the X11 rgb.txt file)
    ///  - an hexadecimal value in the form: `#rgb`, `#rrggbb`, `#rgba`, or `#rrggbbaa`
    ///  - a RGB color in the form: `rgb(r, g, b)`
    ///  - a RGB color in the form: `rgba(r, g, b, a)`
    ///  - a HSL color in the form: `hsl(h, s, l)`
    ///  -a HSL color in the form: `hsla(h, s, l, a)`
    ///
    /// where 'r', 'g', 'b' and 'a' are (respectively) the red, green, blue color
    /// intensities and the opacity. The 'h', 's' and 'l' are (respectively) the
    /// hue, saturation and luminance values.
    ///
    /// In the `rgb` and `rgba` formats, the 'r', 'g', and 'b' values are either
    /// integers between 0 and 255, or percentage values in the range between 0%
    /// and 100%; the percentages require the '%' character. The 'a' value, if
    /// specified, can only be a floating point value between 0.0 and 1.0.
    ///
    /// In the `hls` and `hlsa` formats, the 'h' value (hue) is an angle between
    /// 0 and 360.0 degrees; the 'l' and 's' values (luminance and saturation) are
    /// percentage values in the range between 0% and 100%. The 'a' value, if specified,
    /// can only be a floating point value between 0.0 and 1.0.
    ///
    /// Whitespace inside the definitions is ignored; no leading whitespace
    /// is allowed.
    ///
    /// If the alpha component is not specified then it is assumed to be set to
    /// be fully opaque.
    /// ## `color`
    /// return location for a `Color`
    /// ## `str`
    /// a string specifiying a color
    ///
    /// # Returns
    ///
    /// `true` if parsing succeeded, and `false` otherwise
    pub fn from_string(str: &str) -> Option<InternalColor> {
        unsafe {
            let mut color = InternalColor::uninitialized();
            let ret = from_glib(ffi::clutter_color_from_string(
                color.to_glib_none_mut().0,
                str.to_glib_none().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    /// Retrieves a static color for the given `color` name
    ///
    /// Static colors are created by Clutter and are guaranteed to always be
    /// available and valid
    /// ## `color`
    /// the named global color
    ///
    /// # Returns
    ///
    /// a pointer to a static color; the returned pointer
    ///  is owned by internal and it should never be modified or freed
    pub fn get_static(color: StaticColor) -> Option<InternalColor> {
        unsafe { from_glib_none(ffi::clutter_color_get_static(color.to_glib())) }
    }
}

// impl PartialEq for Color {
//     #[inline]
//     fn eq(&self, other: &Self) -> bool {
//         self.equal(other)
//     }
// }

// impl Eq for Color {}

impl fmt::Display for InternalColor {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// impl hash::Hash for Color {
//     #[inline]
//     fn hash<H>(&self, state: &mut H)
//     where
//         H: hash::Hasher,
//     {
//         hash::Hash::hash(&self.hash(), state)
//     }
// }

#[doc(hidden)]
impl Uninitialized for InternalColor {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc()
    }
}
