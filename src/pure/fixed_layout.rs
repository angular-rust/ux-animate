use std::fmt;

// @extends LayoutManager
#[derive(Default)]
pub struct FixedLayout {
}

impl FixedLayout {
    /// Creates a new `FixedLayout`
    ///
    /// # Returns
    ///
    /// the newly created `FixedLayout`
    pub fn new() -> FixedLayout {
        // unsafe { LayoutManager::from_glib_none(ffi::clutter_fixed_layout_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl fmt::Display for FixedLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FixedLayout")
    }
}
