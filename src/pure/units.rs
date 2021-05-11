use super::UnitType;
use std::{fmt, mem};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Units {
}

impl Units {
    /// Retrieves the unit type of the value stored inside `self`
    ///
    /// # Returns
    ///
    /// a unit type
    pub fn get_unit_type(&self) -> UnitType {
        // unsafe { from_glib(ffi::clutter_units_get_unit_type(self.to_glib_none().0)) }
        unimplemented!()
    }

    /// Retrieves the value stored inside `self`
    ///
    /// # Returns
    ///
    /// the value stored inside a `Units`
    pub fn get_unit_value(&self) -> f32 {
        // unsafe { ffi::clutter_units_get_unit_value(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Converts a value in `Units` to pixels
    ///
    /// # Returns
    ///
    /// the value in pixels
    pub fn in_pixels(&mut self) -> f32 {
        // unsafe { ffi::clutter_units_to_pixels(self.to_glib_none_mut().0) }
        unimplemented!()
    }

    // /// Converts `self` into a string
    // ///
    // /// See `Units::from_string` for the units syntax and for
    // /// examples of output
    // ///
    // /// Fractional values are truncated to the second decimal
    // /// position for em, mm and cm, and to the first decimal position for
    // /// typographic points. Pixels are integers.
    // ///
    // /// # Returns
    // ///
    // /// a newly allocated string containing the encoded
    // ///  `Units` value. Use `g_free` to free the string
    // fn to_string(&self) -> String {
    //     // unsafe { from_glib_full(ffi::clutter_units_to_string(self.to_glib_none().0)) }
    //     unimplemented!()
    // }

    /// Stores a value in centimeters inside `units`
    /// ## `units`
    /// a `Units`
    /// ## `cm`
    /// centimeters
    pub fn from_cm(cm: f32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_cm(units.to_glib_none_mut().0, cm);
        //     units
        // }
        unimplemented!()
    }

    /// Stores a value in em inside `units`, using the default font
    /// name as returned by `Backend::get_font_name`
    /// ## `units`
    /// a `Units`
    /// ## `em`
    /// em
    pub fn from_em(em: f32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_em(units.to_glib_none_mut().0, em);
        //     units
        // }
        unimplemented!()
    }

    /// Stores a value in em inside `units` using `font_name`
    /// ## `units`
    /// a `Units`
    /// ## `font_name`
    /// the font name and size
    /// ## `em`
    /// em
    pub fn from_em_for_font(font_name: Option<&str>, em: f32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_em_for_font(
        //         units.to_glib_none_mut().0,
        //         font_name.to_glib_none().0,
        //         em,
        //     );
        //     units
        // }
        unimplemented!()
    }

    /// Stores a value in millimiters inside `units`
    /// ## `units`
    /// a `Units`
    /// ## `mm`
    /// millimeters
    pub fn from_mm(mm: f32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_mm(units.to_glib_none_mut().0, mm);
        //     units
        // }
        unimplemented!()
    }

    /// Stores a value in pixels inside `units`
    /// ## `units`
    /// a `Units`
    /// ## `px`
    /// pixels
    pub fn from_pixels(px: i32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_pixels(units.to_glib_none_mut().0, px);
        //     units
        // }
        unimplemented!()
    }

    /// Stores a value in typographic points inside `units`
    /// ## `units`
    /// a `Units`
    /// ## `pt`
    /// typographic points
    pub fn from_pt(pt: f32) -> Units {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     ffi::clutter_units_from_pt(units.to_glib_none_mut().0, pt);
        //     units
        // }
        unimplemented!()
    }

    /// Parses a value and updates `units` with it
    ///
    /// A `Units` expressed in string should match:
    ///
    ///
    /// ```text
    ///   units: wsp* unit-value wsp* unit-name? wsp*
    ///   unit-value: number
    ///   unit-name: 'px' | 'pt' | 'mm' | 'em' | 'cm'
    ///   number: digit+
    ///           | digit* sep digit+
    ///   sep: '.' | ','
    ///   digit: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    ///   wsp: (#0x20 | #0x9 | #0xA | #0xB | #0xC | #0xD)+
    /// ```
    ///
    /// For instance, these are valid strings:
    ///
    ///
    /// ```text
    ///   10 px
    ///   5.1 em
    ///   24 pt
    ///   12.6 mm
    ///   .3 cm
    /// ```
    ///
    /// While these are not:
    ///
    ///
    /// ```text
    ///   42 cats
    ///   omg!1!ponies
    /// ```
    ///
    /// If no unit is specified, pixels are assumed.
    /// ## `units`
    /// a `Units`
    /// ## `str`
    /// the string to convert
    ///
    /// # Returns
    ///
    /// `true` if the string was successfully parsed,
    ///  and `false` otherwise
    pub fn from_string(str: &str) -> Option<Units> {
        // unsafe {
        //     let mut units = Units::uninitialized();
        //     let ret = from_glib(ffi::clutter_units_from_string(
        //         units.to_glib_none_mut().0,
        //         str.to_glib_none().0,
        //     ));
        //     if ret {
        //         Some(units)
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }
}

impl fmt::Display for Units {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Units")
    }
}
