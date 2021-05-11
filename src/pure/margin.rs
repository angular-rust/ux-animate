use std::mem;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Margin {
}

impl Margin {
    /// Creates a new `Margin`.
    ///
    /// # Returns
    ///
    /// a newly allocated `Margin`. Use
    ///  `Margin::free` to free the resources associated with it when
    ///  done.
    pub fn new() -> Margin {
        // unsafe { from_glib_full(ffi::clutter_margin_new()) }
        unimplemented!()
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self::new()
    }
}