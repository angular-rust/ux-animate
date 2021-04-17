use crate::Actor;
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct Content(Interface<ffi::ClutterContent>);

    match fn {
        get_type => || ffi::clutter_content_get_type(),
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

impl<O: IsA<Content>> ContentExt for O {
    fn get_preferred_size(&self) -> Option<(f32, f32)> {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::clutter_content_get_preferred_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            ));
            let width = width.assume_init();
            let height = height.assume_init();
            if ret {
                Some((width, height))
            } else {
                None
            }
        }
    }

    fn invalidate(&self) {
        unsafe {
            ffi::clutter_content_invalidate(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_attached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn attached_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContent,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Content>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Content::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"attached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    attached_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_detached<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn detached_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterContent,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Content>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Content::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"detached\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    detached_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content")
    }
}
