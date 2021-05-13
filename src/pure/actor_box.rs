use super::Vertex;

// * ActorBox:
// * @x1: X coordinate of the top left corner
// * @y1: Y coordinate of the top left corner
// * @x2: X coordinate of the bottom right corner
// * @y2: Y coordinate of the bottom right corner
// *
// * Bounding box of an actor. The coordinates of the top left and right bottom
// * corners of an actor. The coordinates of the two points are expressed in
// * pixels with sub-pixel precision
#[derive(Debug, PartialOrd)] // Hash
pub struct ActorBox {
    x1: f32,
    y1: f32,

    x2: f32,
    y2: f32,
}

impl ActorBox {
    /// Allocates a new `ActorBox` using the passed coordinates
    /// for the top left and bottom right points.
    ///
    /// This function is the logical equivalent of:
    ///
    /// ## `x1`
    /// X coordinate of the top left point
    /// ## `y1`
    /// Y coordinate of the top left point
    /// ## `x2`
    /// X coordinate of the bottom right point
    /// ## `y2`
    /// Y coordinate of the bottom right point
    ///
    /// # Returns
    ///
    /// the newly allocated `ActorBox`.
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2
        }
    }

    pub fn clamp_to_pixel(&mut self) {
       self.x1 = self.x1.floor();
       self.y1 = self.y1.floor();
       self.x2 = self.x2.ceil();
       self.y2 = self.y2.ceil();
    }

    /// Checks whether a point with `x`, `y` coordinates is contained
    /// withing `self`
    /// ## `x`
    /// X coordinate of the point
    /// ## `y`
    /// Y coordinate of the point
    ///
    /// # Returns
    ///
    /// `true` if the point is contained by the `ActorBox`
    pub fn contains(&self, x: f32, y: f32) -> bool {
        (x > self.x1 && x < self.x2) &&
         (y > self.y1 && y < self.y2)
    }

    /// Checks `self` and `box_b` for equality
    /// ## `box_b`
    /// a `ActorBox`
    ///
    /// # Returns
    ///
    /// `true` if the passed `ActorBox` are equal
    fn equal(&self, other: &ActorBox) -> bool {
        use std::f32::EPSILON;
        let error_margin = EPSILON;
        (self.x1 - other.x1).abs() < error_margin && (self.y1 - other.y1).abs() < error_margin &&
        (self.x2 - other.x2).abs() < error_margin && (self.y2 - other.y2).abs() < error_margin
    }

    pub fn from_vertices(&mut self, verts: &[Vertex; 4]) {     
        // 4-way min/max
        let mut x1 = verts[0].x;
        let mut y1 = verts[0].y;
      
        if verts[1].x < x1 {
          x1 = verts[1].x;
        }
      
        if verts[2].x < x1 {
          x1 = verts[2].x;
        }
      
        if verts[3].x < x1 {
          x1 = verts[3].x;
        }
      
        if verts[1].y < y1 {
          y1 = verts[1].y;
        }
      
        if verts[2].y < y1 {
          y1 = verts[2].y;
        }
      
        if verts[3].y < y1 {
          y1 = verts[3].y;
        }
      
        let mut x2 = verts[0].x;
        let mut y2 = verts[0].y;
      
        if verts[1].x > x2 {
          x2 = verts[1].x;
        }
      
        if verts[2].x > x2 {
          x2 = verts[2].x;
        }
      
        if verts[3].x > x2 {
          x2 = verts[3].x;
        }
      
        if verts[1].y > y2 {
          y2 = verts[1].y;
        }
      
        if verts[2].y > y2 {
          y2 = verts[2].y;
        }
      
        if verts[3].y > y2 {
          y2 = verts[3].y;
        }
      
        self.x1 = x1;
        self.x2 = x2;
        self.y1 = y1;
        self.y2 = y2;
    }

    /// Retrieves the area of `self`
    ///
    /// # Returns
    ///
    /// the area of a `ActorBox`, in pixels
    pub fn get_area(&self) -> f32 {
        (self.x2 - self.x1) * (self.y2 - self.y1)
    }

    /// Retrieves the height of the `self`
    ///
    /// # Returns
    ///
    /// the height of the box
    pub fn get_height(&self) -> f32 {
        self.y2 - self.y1
    }

    /// Retrieves the origin of `self`
    /// ## `x`
    /// return location for the X coordinate, or `None`
    /// ## `y`
    /// return location for the Y coordinate, or `None`
    pub fn get_origin(&self) -> (f32, f32) {
        (self.x1, self.y1)
    }

    /// Retrieves the size of `self`
    /// ## `width`
    /// return location for the width, or `None`
    /// ## `height`
    /// return location for the height, or `None`
    pub fn get_size(&self) -> (f32, f32) {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;

        (width, height)
    }

    /// Retrieves the width of the `self`
    ///
    /// # Returns
    ///
    /// the width of the box
    pub fn get_width(&self) -> f32 {
        self.x2 - self.x1
    }

    /// Retrieves the X coordinate of the origin of `self`
    ///
    /// # Returns
    ///
    /// the X coordinate of the origin
    pub fn get_x(&self) -> f32 {
        self.x1
    }

    /// Retrieves the Y coordinate of the origin of `self`
    ///
    /// # Returns
    ///
    /// the Y coordinate of the origin
    pub fn get_y(&self) -> f32 {
        self.y1
    }

    /// Initializes `self` with the given origin and size.
    /// ## `x`
    /// X coordinate of the origin
    /// ## `y`
    /// Y coordinate of the origin
    /// ## `width`
    /// width of the box
    /// ## `height`
    /// height of the box
    pub fn init_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.x1 = x;
        self.y1 = y;
        self.x2 = x + width;
        self.x2 = y + height;
    }

    /// Interpolates between `self` and `final_` `ActorBox`<!-- -->es
    /// using `progress`
    /// ## `dest`
    /// the final `ActorBox`
    /// ## `progress`
    /// the interpolation progress
    /// ## `result`
    /// return location for the interpolation
    pub fn interpolate(&self, dest: &ActorBox, progress: f32) -> Self {
        let x1 = self.x1 + (dest.x1 - self.x1) * progress;
        let y1 = self.y1 + (dest.y1 - self.y1) * progress;
        let x2 = self.x2 + (dest.x2 - self.x2) * progress;
        let y2 = self.y2 + (dest.y2 - self.y2) * progress;
        
        Self {
            x1,
            y1,
            x2,
            y2,
        }
    }

    /// Changes the origin of `self`, maintaining the size of the `ActorBox`.
    /// ## `x`
    /// the X coordinate of the new origin
    /// ## `y`
    /// the Y coordinate of the new origin
    pub fn set_origin(&mut self, x: f32, y: f32) {
        let width = self.x2 - self.x1;
        let height = self.y2 - self.y1;

        self.init_rect(x, y, width, height);
    }

    /// Sets the size of `self`, maintaining the origin of the `ActorBox`.
    /// ## `width`
    /// the new width
    /// ## `height`
    /// the new height
    pub fn set_size(&mut self, width: f32, height: f32) {
        self.x2 = self.x1 + width;
        self.y2 = self.y1 + height;
    }

    /// Unions the two boxes `self` and `other` and stores the result in `result`.
    /// ## `other`
    /// the second `ActorBox`
    /// ## `result`
    /// the `ActorBox` representing a union
    ///  of `self` and `other`
    pub fn union(&self, other: &ActorBox) -> ActorBox {
        let x1 = self.x1.min(other.x1);
        let y1 = self.y1.min(other.y1);
      
        let x2 = self.x2.max(other.x2);
        let y2 = self.y2.max(other.y2);

        Self {
            x1,
            y1,
            x2,
            y2
        }
    }
}

impl PartialEq for ActorBox {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for ActorBox {}
