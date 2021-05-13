use glib::translate::*;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct ActorBox(Boxed<ffi::ClutterActorBox>);

    match fn {
        copy => |ptr| ffi::clutter_actor_box_copy(mut_override(ptr)),
        free => |ptr| ffi::clutter_actor_box_free(ptr),
        get_type => || ffi::clutter_actor_box_get_type(),
    }
}

impl ActorBox {
    /// Allocates a new `ActorBox` using the passed coordinates
    /// for the top left and bottom right points.
    ///
    /// This function is the logical equivalent of:
    ///
    ///
    /// ```text
    ///   actor_box_init (actor_box_alloc (),
    ///                           x_1, y_1,
    ///                           x_2, y_2);
    /// ```
    /// ## `x_1`
    /// X coordinate of the top left point
    /// ## `y_1`
    /// Y coordinate of the top left point
    /// ## `x_2`
    /// X coordinate of the bottom right point
    /// ## `y_2`
    /// Y coordinate of the bottom right point
    ///
    /// # Returns
    ///
    /// the newly allocated `ActorBox`.
    ///  Use `ActorBox::free` to free the resources
    pub fn new(x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> ActorBox {
        unsafe { from_glib_full(ffi::clutter_actor_box_new(x_1, y_1, x_2, y_2)) }
    }

    //pub fn clamp_to_pixel(&self) {
    //    unsafe { TODO: call clutter_sys:clutter_actor_box_clamp_to_pixel() }
    //}

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
        unsafe { from_glib(ffi::clutter_actor_box_contains(self.to_glib_none().0, x, y)) }
    }

    /// Checks `self` and `box_b` for equality
    /// ## `box_b`
    /// a `ActorBox`
    ///
    /// # Returns
    ///
    /// `true` if the passed `ActorBox` are equal
    fn equal(&self, box_b: &ActorBox) -> bool {
        unsafe {
            from_glib(ffi::clutter_actor_box_equal(
                self.to_glib_none().0,
                box_b.to_glib_none().0,
            ))
        }
    }

    //pub fn from_vertices(&mut self, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4) {
    //    unsafe { TODO: call clutter_sys:clutter_actor_box_from_vertices() }
    //}

    /// Retrieves the area of `self`
    ///
    /// # Returns
    ///
    /// the area of a `ActorBox`, in pixels
    pub fn get_area(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_area(self.to_glib_none().0) }
    }

    /// Retrieves the height of the `self`
    ///
    /// # Returns
    ///
    /// the height of the box
    pub fn get_height(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_height(self.to_glib_none().0) }
    }

    /// Retrieves the origin of `self`
    /// ## `x`
    /// return location for the X coordinate, or `None`
    /// ## `y`
    /// return location for the Y coordinate, or `None`
    pub fn get_origin(&self) -> (f32, f32) {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            ffi::clutter_actor_box_get_origin(
                self.to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            );
            let x = x.assume_init();
            let y = y.assume_init();
            (x, y)
        }
    }

    /// Retrieves the size of `self`
    /// ## `width`
    /// return location for the width, or `None`
    /// ## `height`
    /// return location for the height, or `None`
    pub fn get_size(&self) -> (f32, f32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::clutter_actor_box_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }

    /// Retrieves the width of the `self`
    ///
    /// # Returns
    ///
    /// the width of the box
    pub fn get_width(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_width(self.to_glib_none().0) }
    }

    /// Retrieves the X coordinate of the origin of `self`
    ///
    /// # Returns
    ///
    /// the X coordinate of the origin
    pub fn get_x(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_x(self.to_glib_none().0) }
    }

    /// Retrieves the Y coordinate of the origin of `self`
    ///
    /// # Returns
    ///
    /// the Y coordinate of the origin
    pub fn get_y(&self) -> f32 {
        unsafe { ffi::clutter_actor_box_get_y(self.to_glib_none().0) }
    }

    /// Initializes `self` with the given coordinates.
    /// ## `x_1`
    /// X coordinate of the top left point
    /// ## `y_1`
    /// Y coordinate of the top left point
    /// ## `x_2`
    /// X coordinate of the bottom right point
    /// ## `y_2`
    /// Y coordinate of the bottom right point
    ///
    /// # Returns
    ///
    /// the initialized `ActorBox`
    pub fn init(&mut self, x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> Option<ActorBox> {
        unsafe {
            from_glib_none(ffi::clutter_actor_box_init(
                self.to_glib_none_mut().0,
                x_1,
                y_1,
                x_2,
                y_2,
            ))
        }
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
        unsafe {
            ffi::clutter_actor_box_init_rect(self.to_glib_none_mut().0, x, y, width, height);
        }
    }

    /// Interpolates between `self` and `final_` `ActorBox`<!-- -->es
    /// using `progress`
    /// ## `final_`
    /// the final `ActorBox`
    /// ## `progress`
    /// the interpolation progress
    /// ## `result`
    /// return location for the interpolation
    pub fn interpolate(&self, final_: &ActorBox, progress: f64) -> ActorBox {
        unsafe {
            let mut result = ActorBox::uninitialized();
            ffi::clutter_actor_box_interpolate(
                self.to_glib_none().0,
                final_.to_glib_none().0,
                progress,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    /// Changes the origin of `self`, maintaining the size of the `ActorBox`.
    /// ## `x`
    /// the X coordinate of the new origin
    /// ## `y`
    /// the Y coordinate of the new origin
    pub fn set_origin(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::clutter_actor_box_set_origin(self.to_glib_none_mut().0, x, y);
        }
    }

    /// Sets the size of `self`, maintaining the origin of the `ActorBox`.
    /// ## `width`
    /// the new width
    /// ## `height`
    /// the new height
    pub fn set_size(&mut self, width: f32, height: f32) {
        unsafe {
            ffi::clutter_actor_box_set_size(self.to_glib_none_mut().0, width, height);
        }
    }

    /// Unions the two boxes `self` and `b` and stores the result in `result`.
    /// ## `b`
    /// the second `ActorBox`
    /// ## `result`
    /// the `ActorBox` representing a union
    ///  of `self` and `b`
    pub fn union(&self, b: &ActorBox) -> ActorBox {
        unsafe {
            let mut result = ActorBox::uninitialized();
            ffi::clutter_actor_box_union(
                self.to_glib_none().0,
                b.to_glib_none().0,
                result.to_glib_none_mut().0,
            );
            result
        }
    }

    /// Allocates a new `ActorBox`.
    ///
    /// # Returns
    ///
    /// the newly allocated `ActorBox`.
    ///  Use `ActorBox::free` to free its resources
    pub fn alloc() -> Option<ActorBox> {
        unsafe { from_glib_full(ffi::clutter_actor_box_alloc()) }
    }
}

impl PartialEq for ActorBox {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for ActorBox {}

#[doc(hidden)]
impl Uninitialized for ActorBox {
    #[inline]
    unsafe fn uninitialized() -> Self {
        Self::alloc().unwrap()
    }
}
