/// Returns a linear interpolated value based on the start value [start], the
/// end value [end], and the interpolation factor [f].
///
/// [start] and [end] can be of any type which defines three operators +, - , *.
pub fn lerp(start: f64, end: f64, f: f64) -> f64 {
    start + (end - start) * f
}
