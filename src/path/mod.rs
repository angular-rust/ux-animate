mod angle;
mod error;
mod length;
mod options;
mod parser;
mod segment;
mod stream;
mod writer;

pub use self::angle::*;
pub use self::error::*;
pub use self::length::*;
pub use self::options::*;
pub use self::parser::*;
pub use self::segment::*;
pub use self::stream::*;

use float_cmp::ApproxEqUlps;
use std::fmt;
use std::io::Write;

/// Representation of the SVG [path data].
///
/// [path data]: https://www.w3.org/TR/SVG11/paths.html#PathData
#[derive(Clone, PartialEq, Default)]
pub struct Path(pub Vec<PathSegment>);

impl Path {
    /// Constructs a new path.
    #[inline]
    pub fn new() -> Self {
        Path(Vec::new())
    }

    /// Constructs a new path with a specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Path(Vec::with_capacity(capacity))
    }

    /// Converts path's segments into absolute one in-place.
    ///
    /// Original segments can be mixed (relative, absolute).
    pub fn conv_to_absolute(&mut self) {
        // position of the previous segment
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        // Position of the previous MoveTo segment.
        // When we get 'm'(relative) segment, which is not first segment - then it's
        // relative to previous 'M'(absolute) segment, not to first segment.
        let mut prev_mx = 0.0;
        let mut prev_my = 0.0;

        let mut prev_cmd = PathCommand::MoveTo;
        for seg in self.iter_mut() {
            if seg.cmd() == PathCommand::ClosePath {
                prev_x = prev_mx;
                prev_y = prev_my;

                seg.set_absolute(true);
                continue;
            }

            let offset_x;
            let offset_y;
            if seg.is_relative() {
                if seg.cmd() == PathCommand::MoveTo && prev_cmd == PathCommand::ClosePath {
                    offset_x = prev_mx;
                    offset_y = prev_my;
                } else {
                    offset_x = prev_x;
                    offset_y = prev_y;
                }
            } else {
                offset_x = 0.0;
                offset_y = 0.0;
            }

            if seg.is_relative() {
                shift_segment_data(seg, offset_x, offset_y);
            }

            if seg.cmd() == PathCommand::MoveTo {
                prev_mx = seg.x().unwrap();
                prev_my = seg.y().unwrap();
            }

            seg.set_absolute(true);

            if seg.cmd() == PathCommand::HorizontalLineTo {
                prev_x = seg.x().unwrap();
            } else if seg.cmd() == PathCommand::VerticalLineTo {
                prev_y = seg.y().unwrap();
            } else {
                prev_x = seg.x().unwrap();
                prev_y = seg.y().unwrap();
            }

            prev_cmd = seg.cmd();
        }
    }

    /// Converts path's segments into relative one in-place.
    ///
    /// Original segments can be mixed (relative, absolute).
    pub fn conv_to_relative(&mut self) {
        // NOTE: this method may look like 'conv_to_absolute', but it's a bit different.

        // position of the previous segment
        let mut prev_x = 0.0;
        let mut prev_y = 0.0;

        // Position of the previous MoveTo segment.
        // When we get 'm'(relative) segment, which is not first segment - then it's
        // relative to previous 'M'(absolute) segment, not to first segment.
        let mut prev_mx = 0.0;
        let mut prev_my = 0.0;

        let mut prev_cmd = PathCommand::MoveTo;
        for seg in self.iter_mut() {
            if seg.cmd() == PathCommand::ClosePath {
                prev_x = prev_mx;
                prev_y = prev_my;

                seg.set_absolute(false);
                continue;
            }

            let offset_x;
            let offset_y;
            if seg.is_absolute() {
                if seg.cmd() == PathCommand::MoveTo && prev_cmd == PathCommand::ClosePath {
                    offset_x = prev_mx;
                    offset_y = prev_my;
                } else {
                    offset_x = prev_x;
                    offset_y = prev_y;
                }
            } else {
                offset_x = 0.0;
                offset_y = 0.0;
            }

            // unlike in 'to_absolute', we should take prev values before changing segment data
            if seg.is_absolute() {
                if seg.cmd() == PathCommand::HorizontalLineTo {
                    prev_x = seg.x().unwrap();
                } else if seg.cmd() == PathCommand::VerticalLineTo {
                    prev_y = seg.y().unwrap();
                } else {
                    prev_x = seg.x().unwrap();
                    prev_y = seg.y().unwrap();
                }
            } else {
                if seg.cmd() == PathCommand::HorizontalLineTo {
                    prev_x += seg.x().unwrap();
                } else if seg.cmd() == PathCommand::VerticalLineTo {
                    prev_y += seg.y().unwrap();
                } else {
                    prev_x += seg.x().unwrap();
                    prev_y += seg.y().unwrap();
                }
            }

            if seg.cmd() == PathCommand::MoveTo {
                if seg.is_absolute() {
                    prev_mx = seg.x().unwrap();
                    prev_my = seg.y().unwrap();
                } else {
                    prev_mx += seg.x().unwrap();
                    prev_my += seg.y().unwrap();
                }
            }

            if seg.is_absolute() {
                shift_segment_data(seg, -offset_x, -offset_y);
            }

            seg.set_absolute(false);

            prev_cmd = seg.cmd();
        }
    }

    /// Appends an absolute MoveTo segment.
    #[inline]
    pub fn push_move_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::MoveTo { abs: true, x, y });
    }

    /// Appends a relative MoveTo segment.
    #[inline]
    pub fn push_rel_move_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::MoveTo { abs: false, x, y });
    }

    /// Appends an absolute ClosePath segment.
    #[inline]
    pub fn push_close_path(&mut self) {
        self.push(PathSegment::ClosePath { abs: true });
    }

    /// Appends a relative ClosePath segment.
    #[inline]
    pub fn push_rel_close_path(&mut self) {
        self.push(PathSegment::ClosePath { abs: false });
    }

    /// Appends an absolute LineTo segment.
    #[inline]
    pub fn push_line_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::LineTo { abs: true, x, y });
    }

    /// Appends a relative LineTo segment.
    #[inline]
    pub fn push_rel_line_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::LineTo { abs: false, x, y });
    }

    /// Appends an absolute HorizontalLineTo segment.
    #[inline]
    pub fn push_hline_to(&mut self, x: f64) {
        self.push(PathSegment::HorizontalLineTo { abs: true, x });
    }

    /// Appends a relative HorizontalLineTo segment.
    #[inline]
    pub fn push_rel_hline_to(&mut self, x: f64) {
        self.push(PathSegment::HorizontalLineTo { abs: false, x });
    }

    /// Appends an absolute VerticalLineTo segment.
    #[inline]
    pub fn push_vline_to(&mut self, y: f64) {
        self.push(PathSegment::VerticalLineTo { abs: true, y });
    }

    /// Appends a relative VerticalLineTo segment.
    #[inline]
    pub fn push_rel_vline_to(&mut self, y: f64) {
        self.push(PathSegment::VerticalLineTo { abs: false, y });
    }

    /// Appends an absolute CurveTo segment.
    #[inline]
    pub fn push_curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::CurveTo {
            abs: true,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        });
    }

    /// Appends a relative CurveTo segment.
    #[inline]
    pub fn push_rel_curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::CurveTo {
            abs: false,
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        });
    }

    /// Appends an absolute SmoothCurveTo segment.
    #[inline]
    pub fn push_smooth_curve_to(&mut self, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::SmoothCurveTo {
            abs: true,
            x2,
            y2,
            x,
            y,
        });
    }

    /// Appends a relative SmoothCurveTo segment.
    #[inline]
    pub fn push_rel_smooth_curve_to(&mut self, x2: f64, y2: f64, x: f64, y: f64) {
        self.push(PathSegment::SmoothCurveTo {
            abs: false,
            x2,
            y2,
            x,
            y,
        });
    }

    /// Appends an absolute QuadTo segment.
    #[inline]
    pub fn push_quad_to(&mut self, x1: f64, y1: f64, x: f64, y: f64) {
        self.push(PathSegment::Quadratic {
            abs: true,
            x1,
            y1,
            x,
            y,
        });
    }

    /// Appends a relative QuadTo segment.
    #[inline]
    pub fn push_rel_quad_to(&mut self, x1: f64, y1: f64, x: f64, y: f64) {
        self.push(PathSegment::Quadratic {
            abs: false,
            x1,
            y1,
            x,
            y,
        });
    }

    /// Appends an absolute SmoothQuadTo segment.
    #[inline]
    pub fn push_smooth_quad_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::SmoothQuadratic { abs: true, x, y });
    }

    /// Appends a relative SmoothQuadTo segment.
    #[inline]
    pub fn push_rel_smooth_quad_to(&mut self, x: f64, y: f64) {
        self.push(PathSegment::SmoothQuadratic { abs: false, x, y });
    }

    /// Appends an absolute ArcTo segment.
    #[inline]
    pub fn push_arc_to(
        &mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    ) {
        self.push(PathSegment::EllipticalArc {
            abs: true,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        });
    }

    /// Appends a relative ArcTo segment.
    #[inline]
    pub fn push_rel_arc_to(
        &mut self,
        rx: f64,
        ry: f64,
        x_axis_rotation: f64,
        large_arc: bool,
        sweep: bool,
        x: f64,
        y: f64,
    ) {
        self.push(PathSegment::EllipticalArc {
            abs: false,
            rx,
            ry,
            x_axis_rotation,
            large_arc,
            sweep,
            x,
            y,
        });
    }
}

fn shift_segment_data(d: &mut PathSegment, offset_x: f64, offset_y: f64) {
    match *d {
        PathSegment::MoveTo {
            ref mut x,
            ref mut y,
            ..
        } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::LineTo {
            ref mut x,
            ref mut y,
            ..
        } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::HorizontalLineTo { ref mut x, .. } => {
            *x += offset_x;
        }
        PathSegment::VerticalLineTo { ref mut y, .. } => {
            *y += offset_y;
        }
        PathSegment::CurveTo {
            ref mut x1,
            ref mut y1,
            ref mut x2,
            ref mut y2,
            ref mut x,
            ref mut y,
            ..
        } => {
            *x1 += offset_x;
            *y1 += offset_y;
            *x2 += offset_x;
            *y2 += offset_y;
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::SmoothCurveTo {
            ref mut x2,
            ref mut y2,
            ref mut x,
            ref mut y,
            ..
        } => {
            *x2 += offset_x;
            *y2 += offset_y;
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::Quadratic {
            ref mut x1,
            ref mut y1,
            ref mut x,
            ref mut y,
            ..
        } => {
            *x1 += offset_x;
            *y1 += offset_y;
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::SmoothQuadratic {
            ref mut x,
            ref mut y,
            ..
        } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::EllipticalArc {
            ref mut x,
            ref mut y,
            ..
        } => {
            *x += offset_x;
            *y += offset_y;
        }
        PathSegment::ClosePath { .. } => {}
    }
}

impl From<Vec<PathSegment>> for Path {
    #[inline]
    fn from(v: Vec<PathSegment>) -> Self {
        Path(v)
    }
}

impl ::std::ops::Deref for Path {
    type Target = Vec<PathSegment>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for Path {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A trait for fuzzy/approximate equality comparisons of float numbers.
pub trait FuzzyEq<Rhs: ?Sized = Self> {
    /// Returns `true` if values are approximately equal.
    fn fuzzy_eq(&self, other: &Rhs) -> bool;

    /// Returns `true` if values are not approximately equal.
    #[inline]
    fn fuzzy_ne(&self, other: &Rhs) -> bool {
        !self.fuzzy_eq(other)
    }
}

impl<T: FuzzyEq> FuzzyEq for Vec<T> {
    fn fuzzy_eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in self.iter().zip(other.iter()) {
            if a.fuzzy_ne(b) {
                return false;
            }
        }

        true
    }
}

/// A trait for writing data to the buffer.
pub trait WriteBuffer {
    /// Writes data to the `Vec<u8>` buffer using specified `WriteOptions`.
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>);

    /// Writes data to the `Vec<u8>` buffer using default `WriteOptions`.
    fn write_buf(&self, buf: &mut Vec<u8>) {
        self.write_buf_opt(&WriteOptions::default(), buf);
    }

    /// Returns an object that implements `fmt::Display` using provided write options.
    fn with_write_opt<'a>(&'a self, opt: &'a WriteOptions) -> DisplaySvg<'a, Self>
    where
        Self: Sized,
    {
        DisplaySvg { value: self, opt }
    }
}

impl<T: WriteBuffer> WriteBuffer for Vec<T> {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        for (n, l) in self.iter().enumerate() {
            l.write_buf_opt(opt, buf);
            if n < self.len() - 1 {
                opt.write_separator(buf);
            }
        }
    }
}

/// A wrapper to use `fmt::Display` with [`WriteOptions`].
///
/// Should be used via `WriteBuffer::with_write_opt`.
///
/// # Example
///
// /// ```
// /// use svgtypes::{Transform, WriteOptions, WriteBuffer, DisplaySvg};
// ///
// /// let ts = Transform::new(1.0, 0.0, 0.0, 1.0, 10.0, 20.0);
// /// assert_eq!(ts.to_string(), "matrix(1 0 0 1 10 20)");
// ///
// /// let opt = WriteOptions {
// ///     simplify_transform_matrices: true,
// ///     .. WriteOptions::default()
// /// };
// /// assert_eq!(ts.with_write_opt(&opt).to_string(), "translate(10 20)");
// /// ```
///
/// [`WriteOptions`]: struct.WriteOptions.html
pub struct DisplaySvg<'a, T: 'a + WriteBuffer> {
    value: &'a T,
    opt: &'a WriteOptions,
}

impl<'a, T: WriteBuffer> fmt::Debug for DisplaySvg<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use Display.
        write!(f, "{}", self)
    }
}

impl<'a, T: WriteBuffer> fmt::Display for DisplaySvg<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str;

        let mut out = Vec::with_capacity(32);
        self.value.write_buf_opt(self.opt, &mut out);
        write!(f, "{}", str::from_utf8(&out).unwrap())
    }
}

impl WriteBuffer for f64 {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        write_num(self, opt.remove_leading_zero, buf);
    }
}

impl FuzzyEq for f32 {
    #[inline]
    fn fuzzy_eq(&self, other: &f32) -> bool {
        self.approx_eq_ulps(other, 4)
    }
}

impl FuzzyEq for f64 {
    #[inline]
    fn fuzzy_eq(&self, other: &f64) -> bool {
        self.approx_eq_ulps(other, 4)
    }
}

impl FuzzyZero for f32 {
    #[inline]
    fn is_fuzzy_zero(&self) -> bool {
        self.fuzzy_eq(&0.0)
    }
}

impl FuzzyZero for f64 {
    #[inline]
    fn is_fuzzy_zero(&self) -> bool {
        self.fuzzy_eq(&0.0)
    }
}

fn write_num(num: &f64, rm_leading_zero: bool, buf: &mut Vec<u8>) {
    // If number is an integer, it's faster to write it as i32.
    if num.fract().is_fuzzy_zero() {
        write!(buf, "{}", *num as i32).unwrap();
        return;
    }

    // Round numbers up to 11 digits to prevent writing
    // ugly numbers like 29.999999999999996.
    // It's not 100% correct, but differences are insignificant.
    let v = (num * 100_000_000_000.0).round() / 100_000_000_000.0;

    let start_pos = buf.len();

    write!(buf, "{}", v).unwrap();

    if rm_leading_zero {
        let mut has_dot = false;
        let mut pos = 0;
        for c in buf.iter().skip(start_pos) {
            if *c == b'.' {
                has_dot = true;
                break;
            }
            pos += 1;
        }

        if has_dot && buf[start_pos + pos - 1] == b'0' {
            if pos == 2 && num.is_sign_negative() {
                // -0.1 -> -.1
                buf.remove(start_pos + 1);
            } else if pos == 1 && num.is_sign_positive() {
                // 0.1 -> .1
                buf.remove(start_pos);
            }
        }
    }
}

/// A trait for fuzzy/approximate comparisons of float numbers.
pub trait FuzzyZero: FuzzyEq {
    /// Returns `true` if the number is approximately zero.
    fn is_fuzzy_zero(&self) -> bool;
}

#[cfg(test)]
mod to_absolute {
    use super::*;
    use std::str::FromStr;

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => {
            #[test]
            fn $name() {
                let mut path = Path::from_str($in_text).unwrap();
                path.conv_to_absolute();
                assert_eq!(path.to_string(), $out_text);
            }
        };
    }

    test!(line_to, "m 10 20 l 20 20", "M 10 20 L 30 40");

    test!(close_path, "m 10 20 l 20 20 z", "M 10 20 L 30 40 Z");

    // test to check that parses implicit MoveTo as LineTo
    test!(implicit_line_to, "m 10 20 20 20", "M 10 20 L 30 40");

    test!(
        hline_vline,
        "m 10 20 v 10 h 10 l 10 10",
        "M 10 20 V 30 H 20 L 30 40"
    );

    test!(
        curve,
        "m 10 20 c 10 10 10 10 10 10",
        "M 10 20 C 20 30 20 30 20 30"
    );

    test!(
        move_to_1,
        "m 10 20 l 10 10 m 10 10 l 10 10",
        "M 10 20 L 20 30 M 30 40 L 40 50"
    );

    test!(
        move_to_2,
        "m 10 20 l 10 10 z m 10 10 l 10 10",
        "M 10 20 L 20 30 Z M 20 30 L 30 40"
    );

    test!(
        move_to_3,
        "m 10 20 l 10 10 Z m 10 10 l 10 10",
        "M 10 20 L 20 30 Z M 20 30 L 30 40"
    );

    // MoveTo after ClosePath can be skipped
    test!(
        move_to_4,
        "m 10 20 l 10 10 Z l 10 10",
        "M 10 20 L 20 30 Z L 20 30"
    );

    test!(
        smooth_curve,
        "m 10 20 s 10 10 10 10",
        "M 10 20 S 20 30 20 30"
    );

    test!(quad, "m 10 20 q 10 10 10 10", "M 10 20 Q 20 30 20 30");

    test!(
        arc_mixed,
        "M 30 150 a 40 40 0 0 1 65 50 Z m 30 30 A 20 20 0 0 0 125 230 Z \
           m 40 24 a 20 20 0 0 1 65 50 z",
        "M 30 150 A 40 40 0 0 1 95 200 Z M 60 180 A 20 20 0 0 0 125 230 Z \
           M 100 204 A 20 20 0 0 1 165 254 Z"
    );
}

#[cfg(test)]
mod to_relative {
    use super::*;
    use std::str::FromStr;

    macro_rules! test {
        ($name:ident, $in_text:expr, $out_text:expr) => {
            #[test]
            fn $name() {
                let mut path = Path::from_str($in_text).unwrap();
                path.conv_to_relative();
                assert_eq!(path.to_string(), $out_text);
            }
        };
    }

    test!(line_to, "M 10 20 L 30 40", "m 10 20 l 20 20");

    test!(close_path, "M 10 20 L 30 40 Z", "m 10 20 l 20 20 z");

    test!(implicit_line_to, "M 10 20 30 40", "m 10 20 l 20 20");

    test!(
        hline_vline,
        "M 10 20 V 30 H 20 L 30 40",
        "m 10 20 v 10 h 10 l 10 10"
    );

    test!(
        curve,
        "M 10 20 C 20 30 20 30 20 30",
        "m 10 20 c 10 10 10 10 10 10"
    );

    test!(
        move_to_1,
        "M 10 20 L 20 30 M 30 40 L 40 50",
        "m 10 20 l 10 10 m 10 10 l 10 10"
    );

    test!(
        move_to_2,
        "M 10 20 L 20 30 Z M 20 30 L 30 40",
        "m 10 20 l 10 10 z m 10 10 l 10 10"
    );

    test!(
        move_to_3,
        "M 10 20 L 20 30 z M 20 30 L 30 40",
        "m 10 20 l 10 10 z m 10 10 l 10 10"
    );

    // MoveTo after ClosePath can be skipped
    test!(
        move_to_4,
        "M 10 20 L 20 30 Z L 20 30",
        "m 10 20 l 10 10 z l 10 10"
    );

    test!(
        smooth_curve,
        "M 10 20 S 20 30 20 30",
        "m 10 20 s 10 10 10 10"
    );

    test!(quad, "M 10 20 Q 20 30 20 30", "m 10 20 q 10 10 10 10");

    test!(
        arc_mixed,
        "M 30 150 a 40 40 0 0 1 65 50 Z m 30 30 A 20 20 0 0 0 125 230 Z \
           m 40 24 a 20 20 0 0 1 65 50 z",
        "m 30 150 a 40 40 0 0 1 65 50 z m 30 30 a 20 20 0 0 0 65 50 z \
           m 40 24 a 20 20 0 0 1 65 50 z"
    );
}
