// * @left: the margin from the left
// * @right: the margin from the right
// * @top: the margin from the top
// * @bottom: the margin from the bottom
// *
// * A representation of the components of a margin.
#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct Margin {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
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
        Default::default()
    }
}
