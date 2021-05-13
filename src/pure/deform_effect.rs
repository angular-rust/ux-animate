use crate::prelude::*;
use glib::signal::SignalHandlerId;
use std::fmt;

// * SECTION:clutter-deform-effect
// * @Title: DeformEffect
// * @Short_Description: A base class for effects deforming the geometry
// *   of an actor
// *
// * #DeformEffect is an abstract class providing all the plumbing
// * for creating effects that result in the deformation of an actor's
// * geometry.
// *
// * #DeformEffect uses offscreen buffers to render the contents of
// * a #Actor and then the Cogl vertex buffers API to submit the
// * geometry to the GPU.
// *
// * #DeformEffect is available since  1.4
// *
// * ## Implementing DeformEffect
// *
// * Sub-classes of #DeformEffect should override the
// * #DeformEffectClass.deform_vertex() virtual function; this function
// * is called on every vertex that needs to be deformed by the effect.
// * Each passed vertex is an in-out parameter that initially contains the
// * position of the vertex and should be modified according to a specific
// * deformation algorithm.
// @extends OffscreenEffect, Effect, ActorMeta,
pub struct DeformEffect {
    back_pipeline: Option<dx::pure::Pipeline>,

    x_tiles: i32,
    y_tiles: i32,

    buffer: Option<dx::pure::AttributeBuffer>,
    primitive: Option<dx::pure::Primitive>,
    lines_primitive: Option<dx::pure::Primitive>,
    n_vertices: i32,
    allocation_id: u64,

    is_dirty: bool,
}

impl Object for DeformEffect {}

/// Trait containing all `DeformEffect` methods.
///
/// # Implementors
///
/// [`DeformEffect`](struct.DeformEffect.html), [`PageTurnEffect`](struct.PageTurnEffect.html)
pub trait DeformEffectExt: 'static {
    //fn get_back_material(&self) -> /*Unimplemented*/Option<dx::pure::Handle>;

    /// Retrieves the number of horizontal and vertical tiles used to sub-divide
    /// the actor's geometry during the effect
    /// ## `x_tiles`
    /// return location for the number of horizontal tiles,
    ///  or `None`
    /// ## `y_tiles`
    /// return location for the number of vertical tiles,
    ///  or `None`
    fn get_n_tiles(&self) -> (u32, u32);

    /// Invalidates the `self`<!-- -->'s vertices and, if it is associated
    /// to an actor, it will queue a redraw
    fn invalidate(&self);

    //fn set_back_material(&self, material: /*Unimplemented*/Option<dx::pure::Handle>);

    /// Sets the number of horizontal and vertical tiles to be used
    /// when applying the effect
    ///
    /// More tiles allow a finer grained deformation at the expenses
    /// of computation
    /// ## `x_tiles`
    /// number of horizontal tiles
    /// ## `y_tiles`
    /// number of vertical tiles
    fn set_n_tiles(&self, x_tiles: u32, y_tiles: u32);

    /// The number of horizontal tiles. The bigger the number, the
    /// smaller the tiles
    fn get_property_x_tiles(&self) -> u32;

    /// The number of horizontal tiles. The bigger the number, the
    /// smaller the tiles
    fn set_property_x_tiles(&self, x_tiles: u32);

    /// The number of vertical tiles. The bigger the number, the
    /// smaller the tiles
    fn get_property_y_tiles(&self) -> u32;

    /// The number of vertical tiles. The bigger the number, the
    /// smaller the tiles
    fn set_property_y_tiles(&self, y_tiles: u32);

    fn connect_property_x_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<DeformEffect>> DeformEffectExt for O {
    //fn get_back_material(&self) -> /*Unimplemented*/Option<dx::pure::Handle> {
    //    unimplemented!()
    //}

    fn get_n_tiles(&self) -> (u32, u32) {
        unimplemented!()
    }

    fn invalidate(&self) {
        unimplemented!()
    }

    //fn set_back_material(&self, material: /*Unimplemented*/Option<dx::pure::Handle>) {
    //    unimplemented!()
    //}

    fn set_n_tiles(&self, x_tiles: u32, y_tiles: u32) {
        unimplemented!()
    }

    fn get_property_x_tiles(&self) -> u32 {
        unimplemented!()
    }

    fn set_property_x_tiles(&self, x_tiles: u32) {
        unimplemented!()
    }

    fn get_property_y_tiles(&self) -> u32 {
        unimplemented!()
    }

    fn set_property_y_tiles(&self, y_tiles: u32) {
        unimplemented!()
    }

    fn connect_property_x_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_y_tiles_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for DeformEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeformEffect")
    }
}
