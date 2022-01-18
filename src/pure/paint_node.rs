use std::{cell::RefCell, fmt};

use crate::prelude::*;

use dx::foundation::colorspace::Color;
use dx::platform::core::{Texture, Pipeline, Matrix, Framebuffer, Path, Primitive, BufferBit};

use super::{Actor, ActorBox};

// @Title: PaintNode
// @Short_Description: Paint objects
//
// #PaintNode is an element in the render graph.
//
// The render graph contains all the elements that need to be painted by
//  when submitting a frame to the graphics system.
//
// The render graph is distinct from the scene graph: the scene graph is
// composed by actors, which can be visible or invisible; the scene graph
// elements also respond to events. The render graph, instead, is only
// composed by nodes that will be painted.
//
// Each #Actor can submit multiple #PaintNode<!-- -->s to
// the render graph.

pub const PAINT_OP_INIT: PaintOperation = PaintOperation {
    opcode: PaintOpCode::Invalid,
};

pub enum PaintOpCode {
    Invalid,
    TexRect([f32; 8]),
    Path(Path),
    Primitive(Primitive),
}

pub struct PaintOperation {
    pub opcode: PaintOpCode,
    //   union {
    //     float texrect[8];

    //     CoglPath *path;

    //     CoglPrimitive *primitive;
    //   } op;
}

pub struct LayerNode {
    pub parent_instance: PaintNode,

    // pub viewport: cairo::Rectangle,
    pub projection: Matrix,

    pub fbo_width: f32,
    pub fbo_height: f32,

    pub state: Option<Pipeline>,
    pub offscreen: Option<Framebuffer>,
    pub texture: Option<Texture>,

    pub opacity: u8,
}

pub struct RootNode {
    pub parent_instance: PaintNode,

    pub framebuffer: Option<Framebuffer>,

    pub clear_flags: BufferBit,
    pub clear_color: Color,
}

pub struct TransformNode {
    pub parent_instance: PaintNode,

    pub modelview: Matrix,
}

struct DummyNode {
    pub parent_instance: PaintNode,

    pub actor: Option<Actor>,
    pub framebuffer: Option<Framebuffer>,
}

pub struct PipelineNode {
    pub parent_instance: PaintNode,

    pub pipeline: Option<Pipeline>,
}

pub struct ColorNode {
    pub parent_instance: PipelineNode,
}

struct TextureNode {
    pub parent_instance: PipelineNode,
}

pub struct TextNode {
    pub parent_instance: PaintNode,

    // pub layout: Option<pango::Layout>,
    pub color: dx::foundation::colorspace::Color,
}

#[derive(Default)]
struct PaintNodeProps {
    // GTypeInstance parent_instance;
    pub parent: Option<Box<PaintNode>>,

    pub first_child: Option<Box<PaintNode>>,
    pub prev_sibling: Option<Box<PaintNode>>,
    pub next_sibling: Option<Box<PaintNode>>,
    pub last_child: Option<Box<PaintNode>>,

    pub n_children: u32,

    pub operations: Option<Vec<PaintOperation>>,

    pub name: Option<String>,

    pub ref_count: u32,
}

#[derive(Default)]
pub struct PaintNode {
    props: RefCell<PaintNodeProps>,
}

impl Object for PaintNode {}
impl Is<PaintNode> for PaintNode {}

impl AsRef<PaintNode> for PaintNode {
    fn as_ref(&self) -> &PaintNode {
        self
    }
}

/// Trait containing all `PaintNode` methods.
///
/// # Implementors
///
/// [`ClipNode`](struct.ClipNode.html), [`PaintNode`](struct.PaintNode.html), [`PipelineNode`](struct.PipelineNode.html), [`TextNode`](struct.TextNode.html)
pub trait PaintNodeExt: 'static {
    /// Adds `child` to the list of children of `self`.
    ///
    /// This function will acquire a reference on `child`.
    /// ## `child`
    /// the child `PaintNode` to add
    fn add_child<P: Is<PaintNode>>(&self, child: &P);

    /// Adds a rectangle region to the `self`, as described by the
    /// passed `rect`.
    /// ## `rect`
    /// a `ActorBox`
    fn add_rectangle(&self, rect: &ActorBox);

    /// Adds a rectangle region to the `self`, with texture coordinates.
    /// ## `rect`
    /// a `ActorBox`
    /// ## `x1`
    /// the left X coordinate of the texture
    /// ## `y1`
    /// the top Y coordinate of the texture
    /// ## `x2`
    /// the right X coordinate of the texture
    /// ## `y2`
    /// the bottom Y coordinate of the texture
    fn add_texture_rectangle(&self, rect: &ActorBox, x1: f32, y1: f32, x2: f32, y2: f32);

    /// Sets a user-readable `name` for `self`.
    ///
    /// The `name` will be used for debugging purposes.
    ///
    /// The `self` will copy the passed string.
    /// ## `name`
    /// a string annotating the `self`
    fn set_name(&self, name: &str);
}

impl<O: Is<PaintNode>> PaintNodeExt for O {
    fn add_child<P: Is<PaintNode>>(&self, child: &P) {
        // unsafe {
        //     ffi::paint_node_add_child(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_rectangle(&self, rect: &ActorBox) {
        // unsafe {
        //     ffi::paint_node_add_rectangle(
        //         self.as_ref().to_glib_none().0,
        //         rect.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_texture_rectangle(&self, rect: &ActorBox, x1: f32, y1: f32, x2: f32, y2: f32) {
        let node = self.as_ref();
        let mut props = node.props.borrow_mut();
        if props.operations.is_none() {
            props.operations = Some(Vec::new());
        }
        let operation = PAINT_OP_INIT;

        // unsafe {
        //     ffi::paint_node_add_texture_rectangle(
        //         self.as_ref().to_glib_none().0,
        //         rect.to_glib_none().0,
        //         x1,
        //         y1,
        //         x2,
        //         y2,
        //     );
        // }
        unimplemented!()
    }

    fn set_name(&self, name: &str) {
        let node = self.as_ref();
        let mut props = node.props.borrow_mut();
        props.name = Some(name.into());
    }
}

impl fmt::Display for PaintNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PaintNode")
    }
}
