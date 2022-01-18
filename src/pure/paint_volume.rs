use crate::prelude::*;
use super::{Actor, ActorBox, Vertex};

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct PaintVolume{
}

impl PaintVolume {
    /// Retrieves the depth of the volume's, axis aligned, bounding box.
    ///
    /// In other words; this takes into account what actor's coordinate
    /// space `self` belongs too and conceptually fits an axis aligned box
    /// around the volume. It returns the size of that bounding box as
    /// measured along the z-axis.
    ///
    /// If, for example, `ActorExt::get_transformed_paint_volume`
    /// is used to transform a 2D child actor that is 100px wide, 100px
    /// high and 0px deep into container coordinates then the depth might
    /// not simply be 0px if the child actor has a 3D rotation applied to
    /// it.
    ///
    /// Remember: if `ActorExt::get_transformed_paint_volume` is
    /// used then the transformed volume will be defined relative to the
    /// container actor and in container coordinates a 2D child actor
    /// can have a 3D bounding volume.
    ///
    /// There are no accuracy guarantees for the reported depth,
    /// except that it must always be greater than, or equal to, the actor's
    /// depth. This is because actors may report simple, loose fitting paint
    /// volumes for efficiency.
    ///
    /// # Returns
    ///
    /// the depth, in units of `self`'s local coordinate system.
    pub fn get_depth(&self) -> f32 {
        // unsafe { ffi::paint_volume_get_depth(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves the height of the volume's, axis aligned, bounding box.
    ///
    /// In other words; this takes into account what actor's coordinate
    /// space `self` belongs too and conceptually fits an axis aligned box
    /// around the volume. It returns the size of that bounding box as
    /// measured along the y-axis.
    ///
    /// If, for example, `ActorExt::get_transformed_paint_volume`
    /// is used to transform a 2D child actor that is 100px wide, 100px
    /// high and 0px deep into container coordinates then the height might
    /// not simply be 100px if the child actor has a 3D rotation applied to
    /// it.
    ///
    /// Remember: if `ActorExt::get_transformed_paint_volume` is
    /// used then a transformed child volume will be defined relative to the
    /// ancestor container actor and so a 2D child actor
    /// can have a 3D bounding volume.
    ///
    /// There are no accuracy guarantees for the reported height,
    /// except that it must always be greater than, or equal to, the actor's
    /// height. This is because actors may report simple, loose fitting paint
    /// volumes for efficiency.
    ///
    /// # Returns
    ///
    /// the height, in units of `self`'s local coordinate system.
    pub fn get_height(&self) -> f32 {
        // unsafe { ffi::paint_volume_get_height(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Retrieves the origin of the `PaintVolume`.
    /// ## `vertex`
    /// the return location for a `Vertex`
    pub fn get_origin(&self) -> Vertex {
        // unsafe {
        //     let mut vertex = Vertex::uninitialized();
        //     ffi::paint_volume_get_origin(
        //         self.to_glib_none().0,
        //         vertex.to_glib_none_mut().0,
        //     );
        //     vertex
        // }
        unimplemented!()
    }

    /// Retrieves the width of the volume's, axis aligned, bounding box.
    ///
    /// In other words; this takes into account what actor's coordinate
    /// space `self` belongs too and conceptually fits an axis aligned box
    /// around the volume. It returns the size of that bounding box as
    /// measured along the x-axis.
    ///
    /// If, for example, `ActorExt::get_transformed_paint_volume`
    /// is used to transform a 2D child actor that is 100px wide, 100px
    /// high and 0px deep into container coordinates then the width might
    /// not simply be 100px if the child actor has a 3D rotation applied to
    /// it.
    ///
    /// Remember: if `ActorExt::get_transformed_paint_volume` is
    /// used then a transformed child volume will be defined relative to the
    /// ancestor container actor and so a 2D child actor can have a 3D
    /// bounding volume.
    ///
    /// There are no accuracy guarantees for the reported width,
    /// except that it must always be greater than, or equal to, the
    /// actor's width. This is because actors may report simple, loose
    /// fitting paint volumes for efficiency.
    ///
    /// # Returns
    ///
    /// the width, in units of `self`'s local coordinate system.
    pub fn get_width(&self) -> f32 {
        // unsafe { ffi::paint_volume_get_width(self.to_glib_none().0) }
        unimplemented!()
    }

    /// Sets the depth of the paint volume. The depth is measured along
    /// the z axis in the actor coordinates that `self` is associated with.
    /// ## `depth`
    /// the depth of the paint volume, in pixels
    pub fn set_depth(&mut self, depth: f32) {
        // unsafe {
        //     ffi::paint_volume_set_depth(self.to_glib_none_mut().0, depth);
        // }
        unimplemented!()
    }

    /// Sets the `PaintVolume` from the allocation of `actor`.
    ///
    /// This function should be used when overriding the
    /// `ActorClass.get_paint_volume`() by `Actor` sub-classes
    /// that do not paint outside their allocation.
    ///
    /// A typical example is:
    ///
    ///
    /// ```text
    /// static gboolean
    /// my_actor_get_paint_volume (Actor       *self,
    ///                            PaintVolume *volume)
    /// {
    ///   return paint_volume_set_from_allocation (volume, self);
    /// }
    /// ```
    /// ## `actor`
    /// a `Actor`
    ///
    /// # Returns
    ///
    /// `true` if the paint volume was successfully set, and `false`
    ///  otherwise
    pub fn set_from_allocation<P: Is<Actor>>(&mut self, actor: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::paint_volume_set_from_allocation(
        //         self.to_glib_none_mut().0,
        //         actor.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    /// Sets the height of the paint volume. The height is measured along
    /// the y axis in the actor coordinates that `self` is associated with.
    /// ## `height`
    /// the height of the paint volume, in pixels
    pub fn set_height(&mut self, height: f32) {
        // unsafe {
        //     ffi::paint_volume_set_height(self.to_glib_none_mut().0, height);
        // }
        unimplemented!()
    }

    /// Sets the origin of the paint volume.
    ///
    /// The origin is defined as the X, Y and Z coordinates of the top-left
    /// corner of an actor's paint volume, in actor coordinates.
    ///
    /// The default is origin is assumed at: (0, 0, 0)
    /// ## `origin`
    /// a `Vertex`
    pub fn set_origin(&mut self, origin: &Vertex) {
        // unsafe {
        //     ffi::paint_volume_set_origin(
        //         self.to_glib_none_mut().0,
        //         origin.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    /// Sets the width of the paint volume. The width is measured along
    /// the x axis in the actor coordinates that `self` is associated with.
    /// ## `width`
    /// the width of the paint volume, in pixels
    pub fn set_width(&mut self, width: f32) {
        // unsafe {
        //     ffi::paint_volume_set_width(self.to_glib_none_mut().0, width);
        // }
        unimplemented!()
    }

    /// Updates the geometry of `self` to encompass `self` and `another_pv`.
    ///
    /// There are no guarantees about how precisely the two volumes
    /// will be unioned.
    /// ## `another_pv`
    /// A second `PaintVolume` to union with `self`
    pub fn union(&mut self, another_pv: &PaintVolume) {
        // unsafe {
        //     ffi::paint_volume_union(self.to_glib_none_mut().0, another_pv.to_glib_none().0);
        // }
        unimplemented!()
    }

    /// Unions the 2D region represented by `box_` to a `PaintVolume`.
    ///
    /// This function is similar to `PaintVolume::union`, but it is
    /// specific for 2D regions.
    /// ## `box_`
    /// a `ActorBox` to union to `self`
    pub fn union_box(&mut self, box_: &ActorBox) {
        // unsafe {
        //     ffi::paint_volume_union_box(self.to_glib_none_mut().0, box_.to_glib_none().0);
        // }
        unimplemented!()
    }
}
