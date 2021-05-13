use super::{Container, Orientation};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// * @short_description: A layout manager arranging children on a single line
// *
// * The #BoxLayout is a #LayoutManager implementing the
// * following layout policy:
// *
// *  - all children are arranged on a single line
// *  - the axis used is controlled by the #BoxLayout:orientation property
// *  - the order of the packing is determined by the #BoxLayout:pack-start boolean property
// *  - each child will be allocated to its natural size or, if #Actor:x-expand or
// *  #Actor:y-expand are set, the available size
// *  - honours the #Actor's #Actor:x-align and #Actor:y-align properties
// *  to fill the available size
// *  - if the #BoxLayout:homogeneous boolean propert is set, then all widgets will
// *  get the same size, ignoring expand settings and the preferred sizes
// *
// * It is possible to control the spacing between children of a
// * #BoxLayout by using clutter_box_layout_set_spacing().
// @extends LayoutManager,
#[derive(Debug, Clone)]
pub struct BoxLayout {
    container: Option<Container>,

    spacing: u32,

    easing_mode: u64,
    easing_duration: u32,

    orientation: Orientation,

    is_pack_start: bool,
    use_animations: bool,
    is_homogeneous: bool,
}

impl BoxLayout {
    /// Creates a new `BoxLayout` layout manager
    ///
    /// # Returns
    ///
    /// the newly created `BoxLayout`
    pub fn new() -> BoxLayout {
        unimplemented!()
    }
}

impl Object for BoxLayout {}
impl Is<BoxLayout> for BoxLayout {}

impl AsRef<BoxLayout> for BoxLayout {
    fn as_ref(&self) -> &BoxLayout {
        self
    }
}

impl Default for BoxLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `BoxLayout` methods.
///
/// # Implementors
///
/// [`BoxLayout`](struct.BoxLayout.html)
pub trait BoxLayoutExt: 'static {
    /// Retrieves if the children sizes are allocated homogeneously.
    ///
    /// # Returns
    ///
    /// `true` if the `BoxLayout` is arranging its children
    ///  homogeneously, and `false` otherwise
    fn get_homogeneous(&self) -> bool;

    /// Retrieves the orientation of the `self`.
    ///
    /// # Returns
    ///
    /// the orientation of the layout
    fn get_orientation(&self) -> Orientation;

    /// Retrieves the value set using `BoxLayoutExt::set_pack_start`
    ///
    /// # Returns
    ///
    /// `true` if the `BoxLayout` should pack children
    ///  at the beginning of the layout, and `false` otherwise
    fn get_pack_start(&self) -> bool;

    /// Retrieves the spacing set using `BoxLayoutExt::set_spacing`
    ///
    /// # Returns
    ///
    /// the spacing between children of the `BoxLayout`
    fn get_spacing(&self) -> u32;

    /// Sets whether the size of `self` children should be
    /// homogeneous
    /// ## `homogeneous`
    /// `true` if the layout should be homogeneous
    fn set_homogeneous(&self, homogeneous: bool);

    /// Sets the orientation of the `BoxLayout` layout manager.
    /// ## `orientation`
    /// the orientation of the `BoxLayout`
    fn set_orientation(&self, orientation: Orientation);

    /// Sets whether children of `self` should be layed out by appending
    /// them or by prepending them
    /// ## `pack_start`
    /// `true` if the `self` should pack children at the
    ///  beginning of the layout
    fn set_pack_start(&self, pack_start: bool);

    /// Sets the spacing between children of `self`
    /// ## `spacing`
    /// the spacing between children of the layout, in pixels
    fn set_spacing(&self, spacing: u32);

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pack_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<BoxLayout>> BoxLayoutExt for O {
    fn get_homogeneous(&self) -> bool {
        let layout = self.as_ref();
        layout.is_homogeneous
    }

    fn get_orientation(&self) -> Orientation {
        let layout = self.as_ref();
        layout.orientation
    }

    fn get_pack_start(&self) -> bool {
        let layout = self.as_ref();
        layout.is_pack_start
    }

    fn get_spacing(&self) -> u32 {
        let layout = self.as_ref();
        layout.spacing
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        unimplemented!()
    }

    fn set_orientation(&self, orientation: Orientation) {
        unimplemented!()
    }

    fn set_pack_start(&self, pack_start: bool) {
        unimplemented!()
    }

    fn set_spacing(&self, spacing: u32) {
        unimplemented!()
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_pack_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoxLayout")
    }
}
