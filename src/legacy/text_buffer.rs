use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    GString,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct TextBuffer(Object<ffi::ClutterTextBuffer, ffi::ClutterTextBufferClass, TextBufferClass>);

    match fn {
        get_type => || ffi::clutter_text_buffer_get_type(),
    }
}

impl TextBuffer {
    /// Create a new TextBuffer object.
    ///
    /// # Returns
    ///
    /// A new TextBuffer object.
    pub fn new() -> TextBuffer {
        unsafe { from_glib_full(ffi::clutter_text_buffer_new()) }
    }

    // pub fn with_text(text: Option<&str>) -> TextBuffer {
    //     let text_len = text.len() as isize;
    //     unsafe {
    //         from_glib_full(ffi::clutter_text_buffer_new_with_text(
    //             text.to_glib_none().0,
    //             text_len,
    //         ))
    //     }
    // }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `TextBuffer` methods.
///
/// # Implementors
///
/// [`TextBuffer`](struct.TextBuffer.html)
pub trait TextBufferExt: 'static {
    /// Deletes a sequence of characters from the buffer. `n_chars` characters are
    /// deleted starting at `position`. If `n_chars` is negative, then all characters
    /// until the end of the text are deleted.
    ///
    /// If `position` or `n_chars` are out of bounds, then they are coerced to sane
    /// values.
    ///
    /// Note that the positions are specified in characters, not bytes.
    /// ## `position`
    /// position at which to delete text
    /// ## `n_chars`
    /// number of characters to delete
    ///
    /// # Returns
    ///
    /// The number of characters deleted.
    fn delete_text(&self, position: u32, n_chars: i32) -> u32;

    /// Emits the `TextBuffer::deleted-text` signal on `self`.
    ///
    /// Used when subclassing `TextBuffer`
    /// ## `position`
    /// position at which text was deleted
    /// ## `n_chars`
    /// number of characters deleted
    fn emit_deleted_text(&self, position: u32, n_chars: u32);

    /// Emits the `TextBuffer::inserted-text` signal on `self`.
    ///
    /// Used when subclassing `TextBuffer`
    /// ## `position`
    /// position at which text was inserted
    /// ## `chars`
    /// text that was inserted
    /// ## `n_chars`
    /// number of characters inserted
    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32);

    /// Retrieves the length in bytes of the buffer.
    /// See `TextBufferExt::get_length`.
    ///
    /// # Returns
    ///
    /// The byte length of the buffer.
    fn get_bytes(&self) -> usize;

    /// Retrieves the length in characters of the buffer.
    ///
    /// # Returns
    ///
    /// The number of characters in the buffer.
    fn get_length(&self) -> u32;

    /// Retrieves the maximum allowed length of the text in
    /// `self`. See `TextBufferExt::set_max_length`.
    ///
    /// # Returns
    ///
    /// the maximum allowed number of characters
    ///  in `TextBuffer`, or 0 if there is no maximum.
    fn get_max_length(&self) -> i32;

    /// Retrieves the contents of the buffer.
    ///
    /// The memory pointer returned by this call will not change
    /// unless this object emits a signal, or is finalized.
    ///
    /// # Returns
    ///
    /// a pointer to the contents of the widget as a
    ///  string. This string points to internally allocated
    ///  storage in the buffer and must not be freed, modified or
    ///  stored.
    fn get_text(&self) -> Option<GString>;

    /// Inserts `n_chars` characters of `chars` into the contents of the
    /// buffer, at position `position`.
    ///
    /// If `n_chars` is negative, then characters from chars will be inserted
    /// until a null-terminator is found. If `position` or `n_chars` are out of
    /// bounds, or the maximum buffer text length is exceeded, then they are
    /// coerced to sane values.
    ///
    /// Note that the position and length are in characters, not in bytes.
    /// ## `position`
    /// the position at which to insert text.
    /// ## `chars`
    /// the text to insert into the buffer.
    /// ## `n_chars`
    /// the length of the text in characters, or -1
    ///
    /// # Returns
    ///
    /// The number of characters actually inserted.
    fn insert_text(&self, position: u32, chars: &str, n_chars: i32) -> u32;

    /// Sets the maximum allowed length of the contents of the buffer. If
    /// the current contents are longer than the given length, then they
    /// will be truncated to fit.
    /// ## `max_length`
    /// the maximum length of the entry buffer, or 0 for no maximum.
    ///  (other than the maximum length of entries.) The value passed in will
    ///  be clamped to the range [ 0, `TEXT_BUFFER_MAX_SIZE` ].
    fn set_max_length(&self, max_length: i32);

    /// Sets the text in the buffer.
    ///
    /// This is roughly equivalent to calling `TextBufferExt::delete_text`
    /// and `TextBufferExt::insert_text`.
    ///
    /// Note that `n_chars` is in characters, not in bytes.
    /// ## `chars`
    /// the new text
    /// ## `n_chars`
    /// the number of characters in `text`, or -1
    fn set_text(&self, chars: &str, n_chars: i32);

    /// This signal is emitted after text is deleted from the buffer.
    /// ## `position`
    /// the position the text was deleted at.
    /// ## `n_chars`
    /// The number of characters that were deleted.
    fn connect_deleted_text<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    /// This signal is emitted after text is inserted into the buffer.
    /// ## `position`
    /// the position the text was inserted at.
    /// ## `chars`
    /// The text that was inserted.
    /// ## `n_chars`
    /// The number of characters that were inserted.
    fn connect_inserted_text<F: Fn(&Self, u32, &str, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TextBuffer>> TextBufferExt for O {
    fn delete_text(&self, position: u32, n_chars: i32) -> u32 {
        unsafe {
            ffi::clutter_text_buffer_delete_text(self.as_ref().to_glib_none().0, position, n_chars)
        }
    }

    fn emit_deleted_text(&self, position: u32, n_chars: u32) {
        unsafe {
            ffi::clutter_text_buffer_emit_deleted_text(
                self.as_ref().to_glib_none().0,
                position,
                n_chars,
            );
        }
    }

    fn emit_inserted_text(&self, position: u32, chars: &str, n_chars: u32) {
        unsafe {
            ffi::clutter_text_buffer_emit_inserted_text(
                self.as_ref().to_glib_none().0,
                position,
                chars.to_glib_none().0,
                n_chars,
            );
        }
    }

    fn get_bytes(&self) -> usize {
        unsafe { ffi::clutter_text_buffer_get_bytes(self.as_ref().to_glib_none().0) }
    }

    fn get_length(&self) -> u32 {
        unsafe { ffi::clutter_text_buffer_get_length(self.as_ref().to_glib_none().0) }
    }

    fn get_max_length(&self) -> i32 {
        unsafe { ffi::clutter_text_buffer_get_max_length(self.as_ref().to_glib_none().0) }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::clutter_text_buffer_get_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn insert_text(&self, position: u32, chars: &str, n_chars: i32) -> u32 {
        unsafe {
            ffi::clutter_text_buffer_insert_text(
                self.as_ref().to_glib_none().0,
                position,
                chars.to_glib_none().0,
                n_chars,
            )
        }
    }

    fn set_max_length(&self, max_length: i32) {
        unsafe {
            ffi::clutter_text_buffer_set_max_length(self.as_ref().to_glib_none().0, max_length);
        }
    }

    fn set_text(&self, chars: &str, n_chars: i32) {
        unsafe {
            ffi::clutter_text_buffer_set_text(
                self.as_ref().to_glib_none().0,
                chars.to_glib_none().0,
                n_chars,
            );
        }
    }

    fn connect_deleted_text<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn deleted_text_trampoline<P, F: Fn(&P, u32, u32) + 'static>(
            this: *mut ffi::ClutterTextBuffer,
            position: libc::c_uint,
            n_chars: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                n_chars,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"deleted-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    deleted_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_inserted_text<F: Fn(&Self, u32, &str, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn inserted_text_trampoline<P, F: Fn(&P, u32, &str, u32) + 'static>(
            this: *mut ffi::ClutterTextBuffer,
            position: libc::c_uint,
            chars: *mut libc::c_char,
            n_chars: libc::c_uint,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &TextBuffer::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                &GString::from_glib_borrow(chars),
                n_chars,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"inserted-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    inserted_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTextBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&TextBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_length_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTextBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&TextBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterTextBuffer,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<TextBuffer>,
        {
            let f: &F = &*(f as *const F);
            f(&TextBuffer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TextBuffer")
    }
}
