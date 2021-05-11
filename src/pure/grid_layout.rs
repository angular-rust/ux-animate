use super::{Actor, GridPosition, Orientation};
use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// @extends LayoutManager
#[derive(Debug, Clone)]
pub struct GridLayout {}

impl GridLayout {
    /// Creates a new `GridLayout`
    ///
    /// # Returns
    ///
    /// the new `GridLayout`
    pub fn new() -> GridLayout {
        // unsafe { LayoutManager::from_glib_none(ffi::clutter_grid_layout_new()).unsafe_cast() }
        unimplemented!()
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `GridLayout` methods.
///
/// # Implementors
///
/// [`GridLayout`](struct.GridLayout.html)
pub trait GridLayoutExt: 'static {
    /// Adds a widget to the grid.
    ///
    /// The position of `child` is determined by `left` and `top`. The
    /// number of 'cells' that `child` will occupy is determined by
    /// `width` and `height`.
    /// ## `child`
    /// the `Actor` to add
    /// ## `left`
    /// the column number to attach the left side of `child` to
    /// ## `top`
    /// the row number to attach the top side of `child` to
    /// ## `width`
    /// the number of columns that `child` will span
    /// ## `height`
    /// the number of rows that `child` will span
    fn attach<P: Is<Actor>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32);

    /// Adds a actor to the grid.
    ///
    /// The actor is placed next to `sibling`, on the side determined by
    /// `side`. When `sibling` is `None`, the actor is placed in row (for
    /// left or right placement) or column 0 (for top or bottom placement),
    /// at the end indicated by `side`.
    ///
    /// Attaching widgets labeled [1], [2], [3] with `sibling` == `None` and
    /// `side` == `GridPosition::Left` yields a layout of [3][2][1].
    /// ## `child`
    /// the actor to add
    /// ## `sibling`
    /// the child of `self` that `child` will be placed
    ///  next to, or `None` to place `child` at the beginning or end
    /// ## `side`
    /// the side of `sibling` that `child` is positioned next to
    /// ## `width`
    /// the number of columns that `child` will span
    /// ## `height`
    /// the number of rows that `child` will span
    fn attach_next_to<P: Is<Actor>, Q: Is<Actor>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: GridPosition,
        width: i32,
        height: i32,
    );

    /// Gets the child of `self` whose area covers the grid
    /// cell whose upper left corner is at `left`, `top`.
    /// ## `left`
    /// the left edge of the cell
    /// ## `top`
    /// the top edge of the cell
    ///
    /// # Returns
    ///
    /// the child at the given position, or `None`
    fn get_child_at(&self, left: i32, top: i32) -> Option<Actor>;

    /// Returns whether all columns of `self` have the same width.
    ///
    /// # Returns
    ///
    /// whether all columns of `self` have the same width.
    fn get_column_homogeneous(&self) -> bool;

    /// Retrieves the spacing set using `GridLayoutExt::set_column_spacing`
    ///
    /// # Returns
    ///
    /// the spacing between coluns of `self`
    fn get_column_spacing(&self) -> u32;

    /// Retrieves the orientation of the `self`.
    ///
    /// # Returns
    ///
    /// the orientation of the layout
    fn get_orientation(&self) -> Orientation;

    /// Returns whether all rows of `self` have the same height.
    ///
    /// # Returns
    ///
    /// whether all rows of `self` have the same height.
    fn get_row_homogeneous(&self) -> bool;

    /// Retrieves the spacing set using `GridLayoutExt::set_row_spacing`
    ///
    /// # Returns
    ///
    /// the spacing between rows of `self`
    fn get_row_spacing(&self) -> u32;

    /// Inserts a column at the specified position.
    ///
    /// Children which are attached at or to the right of this position
    /// are moved one column to the right. Children which span across this
    /// position are grown to span the new column.
    /// ## `position`
    /// the position to insert the column at
    fn insert_column(&self, position: i32);

    /// Inserts a row or column at the specified position.
    ///
    /// The new row or column is placed next to `sibling`, on the side
    /// determined by `side`. If `side` is `GridPosition::Left` or
    /// `GridPosition::Bottom`, a row is inserted. If `side` is
    /// `GridPosition::Left` of `GridPosition::Right`,
    /// a column is inserted.
    /// ## `sibling`
    /// the child of `self` that the new row or column will be
    ///  placed next to
    /// ## `side`
    /// the side of `sibling` that `child` is positioned next to
    fn insert_next_to<P: Is<Actor>>(&self, sibling: &P, side: GridPosition);

    /// Inserts a row at the specified position.
    ///
    /// Children which are attached at or below this position
    /// are moved one row down. Children which span across this
    /// position are grown to span the new row.
    /// ## `position`
    /// the position to insert the row at
    fn insert_row(&self, position: i32);

    /// Sets whether all columns of `self` will have the same width.
    /// ## `homogeneous`
    /// `true` to make columns homogeneous
    fn set_column_homogeneous(&self, homogeneous: bool);

    /// Sets the spacing between columns of `self`
    /// ## `spacing`
    /// the spacing between columns of the layout, in pixels
    fn set_column_spacing(&self, spacing: u32);

    /// Sets the orientation of the `self`.
    ///
    /// `GridLayout` uses the orientation as a hint when adding
    /// children to the `Actor` using it as a layout manager via
    /// `ActorExt::add_child`; changing this value will not have
    /// any effect on children that are already part of the layout.
    /// ## `orientation`
    /// the orientation of the `GridLayout`
    fn set_orientation(&self, orientation: Orientation);

    /// Sets whether all rows of `self` will have the same height.
    /// ## `homogeneous`
    /// `true` to make rows homogeneous
    fn set_row_homogeneous(&self, homogeneous: bool);

    /// Sets the spacing between rows of `self`
    /// ## `spacing`
    /// the spacing between rows of the layout, in pixels
    fn set_row_spacing(&self, spacing: u32);

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl Object for GridLayout {}

impl<O: Is<GridLayout>> GridLayoutExt for O {
    fn attach<P: Is<Actor>>(&self, child: &P, left: i32, top: i32, width: i32, height: i32) {
        // unsafe {
        //     ffi::clutter_grid_layout_attach(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         left,
        //         top,
        //         width,
        //         height,
        //     );
        // }
        unimplemented!()
    }

    fn attach_next_to<P: Is<Actor>, Q: Is<Actor>>(
        &self,
        child: &P,
        sibling: Option<&Q>,
        side: GridPosition,
        width: i32,
        height: i32,
    ) {
        // unsafe {
        //     ffi::clutter_grid_layout_attach_next_to(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         sibling.map(|p| p.as_ref()).to_glib_none().0,
        //         side.to_glib(),
        //         width,
        //         height,
        //     );
        // }
        unimplemented!()
    }

    fn get_child_at(&self, left: i32, top: i32) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_grid_layout_get_child_at(
        //         self.as_ref().to_glib_none().0,
        //         left,
        //         top,
        //     ))
        // }
        unimplemented!()
    }

    fn get_column_homogeneous(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_grid_layout_get_column_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_column_spacing(&self) -> u32 {
        // unsafe { ffi::clutter_grid_layout_get_column_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_orientation(&self) -> Orientation {
        // unsafe {
        //     from_glib(ffi::clutter_grid_layout_get_orientation(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_row_homogeneous(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_grid_layout_get_row_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_row_spacing(&self) -> u32 {
        // unsafe { ffi::clutter_grid_layout_get_row_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn insert_column(&self, position: i32) {
        // unsafe {
        //     ffi::clutter_grid_layout_insert_column(self.as_ref().to_glib_none().0, position);
        // }
        unimplemented!()
    }

    fn insert_next_to<P: Is<Actor>>(&self, sibling: &P, side: GridPosition) {
        // unsafe {
        //     ffi::clutter_grid_layout_insert_next_to(
        //         self.as_ref().to_glib_none().0,
        //         sibling.as_ref().to_glib_none().0,
        //         side.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn insert_row(&self, position: i32) {
        // unsafe {
        //     ffi::clutter_grid_layout_insert_row(self.as_ref().to_glib_none().0, position);
        // }
        unimplemented!()
    }

    fn set_column_homogeneous(&self, homogeneous: bool) {
        // unsafe {
        //     ffi::clutter_grid_layout_set_column_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //         homogeneous.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_column_spacing(&self, spacing: u32) {
        // unsafe {
        //     ffi::clutter_grid_layout_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
        unimplemented!()
    }

    fn set_orientation(&self, orientation: Orientation) {
        // unsafe {
        //     ffi::clutter_grid_layout_set_orientation(
        //         self.as_ref().to_glib_none().0,
        //         orientation.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_row_homogeneous(&self, homogeneous: bool) {
        // unsafe {
        //     ffi::clutter_grid_layout_set_row_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //         homogeneous.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_row_spacing(&self, spacing: u32) {
        // unsafe {
        //     ffi::clutter_grid_layout_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
        unimplemented!()
    }

    fn connect_property_column_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_row_homogeneous_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for GridLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GridLayout")
    }
}
