use crate::prelude::*;
use super::Actor;
use glib::{
    signal::{connect_raw, SignalHandlerId},
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

// * SECTION:clutter-content
// * @Title: ClutterContent
// * @Short_Description: Delegate for painting the content of an actor
// *
// * #ClutterContent is an interface to implement types responsible for
// * painting the content of a #ClutterActor.
// *
// * Multiple actors can use the same #ClutterContent instance, in order
// * to share the resources associated with painting the same content.
// *
// * ClutterContentIface:
// * @get_preferred_size: virtual function; should be overridden by subclasses
// *   of #ClutterContent that have a natural size
// * @paint_content: virtual function; called each time the content needs to
// *   paint itself
// * @attached: virtual function; called each time a #ClutterContent is attached
// *   to a #ClutterActor.
// * @detached: virtual function; called each time a #ClutterContent is detached
// *   from a #ClutterActor.
// * @invalidate: virtual function; called each time a #ClutterContent state
// *   is changed.
// *
// * The #ClutterContentIface structure contains only
// * private data.
#[derive(Debug, Clone)]
pub struct Content {
}

impl Object for Content {}
impl Is<Content> for Content {}

impl AsRef<Content> for Content {
    fn as_ref(&self) -> &Content {
        self
    }
}

/// Trait containing all `Content` methods.
///
/// # Implementors
///
/// [`Canvas`](struct.Canvas.html), [`Content`](struct.Content.html), [`Image`](struct.Image.html)
pub trait ContentExt: 'static {
    /// Retrieves the natural size of the `self`, if any.
    ///
    /// The natural size of a `Content` is defined as the size the content
    /// would have regardless of the allocation of the actor that is painting it,
    /// for instance the size of an image data.
    /// ## `width`
    /// return location for the natural width of the content
    /// ## `height`
    /// return location for the natural height of the content
    ///
    /// # Returns
    ///
    /// `true` if the content has a preferred size, and `false`
    ///  otherwise
    fn get_preferred_size(&self) -> Option<(f32, f32)>;

    /// Invalidates a `Content`.
    ///
    /// This function should be called by `Content` implementations when
    /// they change the way a the content should be painted regardless of the
    /// actor state.
    fn invalidate(&self);

    /// This signal is emitted each time a `Content` implementation is
    /// assigned to a `Actor`.
    /// ## `actor`
    /// a `Actor`
    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// This signal is emitted each time a `Content` implementation is
    /// removed from a `Actor`.
    /// ## `actor`
    /// a `Actor`
    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Content>> ContentExt for O {
    fn get_preferred_size(&self) -> Option<(f32, f32)> {
        unimplemented!()
    }

    fn invalidate(&self) {
        unimplemented!()
    }

    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content")
    }
}
