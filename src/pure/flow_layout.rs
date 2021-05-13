use super::{FlowOrientation, HandlerId};
use crate::prelude::*;
use std::fmt;

// @extends LayoutManager
#[derive(Debug, Clone)]
pub struct FlowLayout {}

impl FlowLayout {
    /// Creates a new `FlowLayout` with the given `orientation`
    /// ## `orientation`
    /// the orientation of the flow layout
    ///
    /// # Returns
    ///
    /// the newly created `FlowLayout`
    pub fn new(orientation: FlowOrientation) -> FlowLayout {
        // unsafe {
        //     LayoutManager::from_glib_none(ffi::clutter_flow_layout_new(orientation.to_glib()))
        //         .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Object for FlowLayout {}
impl Is<FlowLayout> for FlowLayout {}

impl AsRef<FlowLayout> for FlowLayout {
    fn as_ref(&self) -> &FlowLayout {
        self
    }
}

/// Trait containing all `FlowLayout` methods.
///
/// # Implementors
///
/// [`FlowLayout`](struct.FlowLayout.html)
pub trait FlowLayoutExt: 'static {
    /// Retrieves the spacing between columns
    ///
    /// # Returns
    ///
    /// the spacing between columns of the `FlowLayout`,
    ///  in pixels
    fn get_column_spacing(&self) -> f32;

    /// Retrieves the minimum and maximum column widths
    /// ## `min_width`
    /// return location for the minimum column width, or `None`
    /// ## `max_width`
    /// return location for the maximum column width, or `None`
    fn get_column_width(&self) -> (f32, f32);

    /// Retrieves whether the `self` is homogeneous
    ///
    /// # Returns
    ///
    /// `true` if the `FlowLayout` is homogeneous
    fn get_homogeneous(&self) -> bool;

    /// Retrieves the orientation of the `self`
    ///
    /// # Returns
    ///
    /// the orientation of the `FlowLayout`
    fn get_orientation(&self) -> FlowOrientation;

    /// Retrieves the minimum and maximum row heights
    /// ## `min_height`
    /// return location for the minimum row height, or `None`
    /// ## `max_height`
    /// return location for the maximum row height, or `None`
    fn get_row_height(&self) -> (f32, f32);

    /// Retrieves the spacing between rows
    ///
    /// # Returns
    ///
    /// the spacing between rows of the `FlowLayout`,
    ///  in pixels
    fn get_row_spacing(&self) -> f32;

    /// Retrieves the value of `FlowLayout:snap-to-grid` property
    ///
    /// # Returns
    ///
    /// `true` if the `self` is placing its children on a grid
    fn get_snap_to_grid(&self) -> bool;

    /// Sets the space between columns, in pixels
    /// ## `spacing`
    /// the space between columns
    fn set_column_spacing(&self, spacing: f32);

    /// Sets the minimum and maximum widths that a column can have
    /// ## `min_width`
    /// minimum width of a column
    /// ## `max_width`
    /// maximum width of a column
    fn set_column_width(&self, min_width: f32, max_width: f32);

    /// Sets whether the `self` should allocate the same space for
    /// each child
    /// ## `homogeneous`
    /// whether the layout should be homogeneous or not
    fn set_homogeneous(&self, homogeneous: bool);

    /// Sets the orientation of the flow layout
    ///
    /// The orientation controls the direction used to allocate
    /// the children: either horizontally or vertically. The
    /// orientation also controls the direction of the overflowing
    /// ## `orientation`
    /// the orientation of the layout
    fn set_orientation(&self, orientation: FlowOrientation);

    /// Sets the minimum and maximum heights that a row can have
    /// ## `min_height`
    /// the minimum height of a row
    /// ## `max_height`
    /// the maximum height of a row
    fn set_row_height(&self, min_height: f32, max_height: f32);

    /// Sets the spacing between rows, in pixels
    /// ## `spacing`
    /// the space between rows
    fn set_row_spacing(&self, spacing: f32);

    /// Whether the `self` should place its children on a grid.
    /// ## `snap_to_grid`
    /// `true` if `self` should place its children on a grid
    fn set_snap_to_grid(&self, snap_to_grid: bool);

    /// Maximum width for each column in the layout, in pixels. If
    /// set to -1 the width will be the maximum child width
    fn get_property_max_column_width(&self) -> f32;

    /// Maximum width for each column in the layout, in pixels. If
    /// set to -1 the width will be the maximum child width
    fn set_property_max_column_width(&self, max_column_width: f32);

    /// Maximum height for each row in the layout, in pixels. If
    /// set to -1 the width will be the maximum child height
    fn get_property_max_row_height(&self) -> f32;

    /// Maximum height for each row in the layout, in pixels. If
    /// set to -1 the width will be the maximum child height
    fn set_property_max_row_height(&self, max_row_height: f32);

    /// Minimum width for each column in the layout, in pixels
    fn get_property_min_column_width(&self) -> f32;

    /// Minimum width for each column in the layout, in pixels
    fn set_property_min_column_width(&self, min_column_width: f32);

    /// Minimum height for each row in the layout, in pixels
    fn get_property_min_row_height(&self) -> f32;

    /// Minimum height for each row in the layout, in pixels
    fn set_property_min_row_height(&self, min_row_height: f32);

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_max_column_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_max_row_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_min_column_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_min_row_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_snap_to_grid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<FlowLayout>> FlowLayoutExt for O {
    fn get_column_spacing(&self) -> f32 {
        // unsafe { ffi::clutter_flow_layout_get_column_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_column_width(&self) -> (f32, f32) {
        // unsafe {
        //     let mut min_width = mem::MaybeUninit::uninit();
        //     let mut max_width = mem::MaybeUninit::uninit();
        //     ffi::clutter_flow_layout_get_column_width(
        //         self.as_ref().to_glib_none().0,
        //         min_width.as_mut_ptr(),
        //         max_width.as_mut_ptr(),
        //     );
        //     let min_width = min_width.assume_init();
        //     let max_width = max_width.assume_init();
        //     (min_width, max_width)
        // }
        unimplemented!()
    }

    fn get_homogeneous(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_flow_layout_get_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_orientation(&self) -> FlowOrientation {
        // unsafe {
        //     from_glib(ffi::clutter_flow_layout_get_orientation(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_row_height(&self) -> (f32, f32) {
        // unsafe {
        //     let mut min_height = mem::MaybeUninit::uninit();
        //     let mut max_height = mem::MaybeUninit::uninit();
        //     ffi::clutter_flow_layout_get_row_height(
        //         self.as_ref().to_glib_none().0,
        //         min_height.as_mut_ptr(),
        //         max_height.as_mut_ptr(),
        //     );
        //     let min_height = min_height.assume_init();
        //     let max_height = max_height.assume_init();
        //     (min_height, max_height)
        // }
        unimplemented!()
    }

    fn get_row_spacing(&self) -> f32 {
        // unsafe { ffi::clutter_flow_layout_get_row_spacing(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_snap_to_grid(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_flow_layout_get_snap_to_grid(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn set_column_spacing(&self, spacing: f32) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_column_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
        unimplemented!()
    }

    fn set_column_width(&self, min_width: f32, max_width: f32) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_column_width(
        //         self.as_ref().to_glib_none().0,
        //         min_width,
        //         max_width,
        //     );
        // }
        unimplemented!()
    }

    fn set_homogeneous(&self, homogeneous: bool) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_homogeneous(
        //         self.as_ref().to_glib_none().0,
        //         homogeneous.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_orientation(&self, orientation: FlowOrientation) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_orientation(
        //         self.as_ref().to_glib_none().0,
        //         orientation.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_row_height(&self, min_height: f32, max_height: f32) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_row_height(
        //         self.as_ref().to_glib_none().0,
        //         min_height,
        //         max_height,
        //     );
        // }
        unimplemented!()
    }

    fn set_row_spacing(&self, spacing: f32) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_row_spacing(self.as_ref().to_glib_none().0, spacing);
        // }
    }

    fn set_snap_to_grid(&self, snap_to_grid: bool) {
        // unsafe {
        //     ffi::clutter_flow_layout_set_snap_to_grid(
        //         self.as_ref().to_glib_none().0,
        //         snap_to_grid.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn get_property_max_column_width(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"max-column-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `max-column-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_max_column_width(&self, max_column_width: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"max-column-width\0".as_ptr() as *const _,
        //         Value::from(&max_column_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_max_row_height(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"max-row-height\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `max-row-height` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_max_row_height(&self, max_row_height: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"max-row-height\0".as_ptr() as *const _,
        //         Value::from(&max_row_height).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_min_column_width(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-column-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-column-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_column_width(&self, min_column_width: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-column-width\0".as_ptr() as *const _,
        //         Value::from(&min_column_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_min_row_height(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-row-height\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-row-height` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_row_height(&self, min_row_height: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-row-height\0".as_ptr() as *const _,
        //         Value::from(&min_row_height).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_property_column_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_homogeneous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_max_column_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_max_row_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_min_column_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_min_row_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_orientation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_row_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_snap_to_grid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for FlowLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FlowLayout")
    }
}
