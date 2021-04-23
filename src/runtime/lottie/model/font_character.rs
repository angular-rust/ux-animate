use super::content::ShapeGroup;

pub struct FontCharacter {
    shapes: Vec<ShapeGroup>,
    character: String,
    size: f64,
    width: f64,
    style: String,
    font_family: String,
}

impl FontCharacter {
    pub fn new(
        shapes: Vec<ShapeGroup>,
        character: String,
        size: f64,
        width: f64,
        style: String,
        font_family: String,
    ) -> Self {
        Self {
            shapes,
            character,
            size,
            width,
            style,
            font_family,
        }
    }

    pub fn hash_for(character: &String, font_family: &String, style: &String) -> u64 {
        unimplemented!()
    }

    fn get_hash_code(&self) -> u64 {
        FontCharacter::hash_for(&self.character, &self.font_family, &self.style)
    }
}

impl PartialEq for FontCharacter {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
            && self.size == other.size
            && self.width == other.width
            && self.style == other.style
            && self.font_family == other.font_family
    }
}

impl Eq for FontCharacter {}
