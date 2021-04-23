pub struct Font {
    family: String,
    name: String,
    style: String,
    ascent: f64,
}

impl Font {
    pub fn new(family: String, name: String, style: String, ascent: f64) -> Self {
        Self {
            family,
            name,
            style,
            ascent
        }
    }
}