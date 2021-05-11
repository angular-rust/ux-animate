use super::{InternalRect, TextBuffer};
use crate::prelude::*;
use crate::Color;
use glib::signal::SignalHandlerId;
use std::fmt;

// TODO: implements atk::ImplementorIface, Scriptable, Animatable, Container
// @extends Actor,
#[derive(Debug, Clone)]
pub struct Text {}

impl Text {
    /// Creates a new `Text` actor. This actor can be used to
    /// display and edit text.
    ///
    /// # Returns
    ///
    /// the newly created `Text` actor
    pub fn new() -> Text {
        // unsafe { Actor::from_glib_none(ffi::clutter_text_new()).unsafe_cast() }
        unimplemented!()
    }

    /// Creates a new `Text` actor, using `font_name` as the font
    /// description; `text` will be used to set the contents of the actor;
    /// and `color` will be used as the color to render `text`.
    ///
    /// This function is equivalent to calling `Text::new`,
    /// `TextExt::set_font_name`, `TextExt::set_text` and
    /// `TextExt::set_color`.
    /// ## `font_name`
    /// a string with a font description
    /// ## `text`
    /// the contents of the actor
    /// ## `color`
    /// the color to be used to render `text`
    ///
    /// # Returns
    ///
    /// the newly created `Text` actor
    pub fn new_full(font_name: &str, text: &str, color: Color) -> Text {
        // let color = {
        //     let RgbaColor {
        //         red,
        //         green,
        //         blue,
        //         alpha,
        //     } = color.into();
        //     InternalColor::new(red, green, blue, alpha)
        // };

        // unsafe {
        //     Actor::from_glib_none(ffi::clutter_text_new_full(
        //         font_name.to_glib_none().0,
        //         text.to_glib_none().0,
        //         color.to_glib_none().0,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }

    pub fn with_buffer<P: Is<TextBuffer>>(buffer: &P) -> Text {
        // unsafe {
        //     Actor::from_glib_none(ffi::clutter_text_new_with_buffer(
        //         buffer.as_ref().to_glib_none().0,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }

    pub fn with_text(font_name: Option<&str>, text: &str) -> Text {
        // unsafe {
        //     Actor::from_glib_none(ffi::clutter_text_new_with_text(
        //         font_name.to_glib_none().0,
        //         text.to_glib_none().0,
        //     ))
        //     .unsafe_cast()
        // }
        unimplemented!()
    }
}

impl Object for Text {}
impl Is<Text> for Text {}
// impl Is<Actor> for Text {}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        self
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `Text` methods.
///
/// # Implementors
///
/// [`Text`](struct.Text.html)
pub trait TextExt: 'static {
    /// Emits the `Text::activate` signal, if `self` has been set
    /// as activatable using `TextExt::set_activatable`.
    ///
    /// This function can be used to emit the ::activate signal inside
    /// a `Actor::captured-event` or `Actor::key-press-event`
    /// signal handlers before the default signal handler for the
    /// `Text` is invoked.
    ///
    /// # Returns
    ///
    /// `true` if the ::activate signal has been emitted,
    ///  and `false` otherwise
    fn activate(&self) -> bool;

    /// Retrieves the position of the character at the given coordinates.
    /// ## `x`
    /// the X coordinate, relative to the actor
    /// ## `y`
    /// the Y coordinate, relative to the actor
    ///
    /// # Returns
    ///
    /// the position of the character
    fn coords_to_position(&self, x: f32, y: f32) -> i32;

    /// Deletes `n_chars` inside a `Text` actor, starting from the
    /// current cursor position.
    ///
    /// Somewhat awkwardly, the cursor position is decremented by the same
    /// number of characters you've deleted.
    /// ## `n_chars`
    /// the number of characters to delete
    fn delete_chars(&self, n_chars: u32);

    /// Deletes the currently selected text
    ///
    /// This function is only useful in subclasses of `Text`
    ///
    /// # Returns
    ///
    /// `true` if text was deleted or if the text actor
    ///  is empty, and `false` otherwise
    fn delete_selection(&self) -> bool;

    /// Deletes the text inside a `Text` actor between `start_pos`
    /// and `end_pos`.
    ///
    /// The starting and ending positions are expressed in characters,
    /// not in bytes.
    /// ## `start_pos`
    /// starting position
    /// ## `end_pos`
    /// ending position
    fn delete_text(&self, start_pos: isize, end_pos: isize);

    /// Retrieves whether a `Text` is activatable or not.
    ///
    /// # Returns
    ///
    /// `true` if the actor is activatable
    fn get_activatable(&self) -> bool;

    /// Gets the attribute list that was set on the `Text` actor
    /// `TextExt::set_attributes`, if any.
    ///
    /// # Returns
    ///
    /// the attribute list, or `None` if none was set. The
    ///  returned value is owned by the `Text` and should not be unreferenced.
    fn get_attributes(&self) -> Option<pango::AttrList>;

    /// Get the `TextBuffer` object which holds the text for
    /// this widget.
    ///
    /// # Returns
    ///
    /// A ``GtkEntryBuffer`` object.
    fn get_buffer(&self) -> Option<TextBuffer>;

    /// Retrieves the contents of the `Text` actor between
    /// `start_pos` and `end_pos`, but not including `end_pos`.
    ///
    /// The positions are specified in characters, not in bytes.
    /// ## `start_pos`
    /// start of text, in characters
    /// ## `end_pos`
    /// end of text, in characters
    ///
    /// # Returns
    ///
    /// a newly allocated string with the contents of
    ///  the text actor between the specified positions. Use `g_free`
    ///  to free the resources when done
    fn get_chars(&self, start_pos: isize, end_pos: isize) -> Option<String>;

    /// Retrieves the text color as set by `TextExt::set_color`.
    /// ## `color`
    /// return location for a `Color`
    fn get_color(&self) -> Color;

    /// Retrieves the color of the cursor of a `Text` actor.
    /// ## `color`
    /// return location for a `Color`
    fn get_cursor_color(&self) -> Color;

    /// Retrieves the cursor position.
    ///
    /// # Returns
    ///
    /// the cursor position, in characters
    fn get_cursor_position(&self) -> i32;

    /// Retrieves the rectangle that contains the cursor.
    ///
    /// The coordinates of the rectangle's origin are in actor-relative
    /// coordinates.
    /// ## `rect`
    /// return location of a `Rect`
    fn get_cursor_rect(&self) -> InternalRect;

    /// Retrieves the size of the cursor of a `Text` actor.
    ///
    /// # Returns
    ///
    /// the size of the cursor, in pixels
    fn get_cursor_size(&self) -> u32;

    /// Retrieves whether the cursor of a `Text` actor is visible.
    ///
    /// # Returns
    ///
    /// `true` if the cursor is visible
    fn get_cursor_visible(&self) -> bool;

    /// Retrieves whether a `Text` is editable or not.
    ///
    /// # Returns
    ///
    /// `true` if the actor is editable
    fn get_editable(&self) -> bool;

    /// Returns the ellipsizing position of a `Text` actor, as
    /// set by `TextExt::set_ellipsize`.
    ///
    /// # Returns
    ///
    /// `pango::EllipsizeMode`
    fn get_ellipsize(&self) -> pango::EllipsizeMode;

    /// Retrieves the `pango::FontDescription` used by `self`
    ///
    /// # Returns
    ///
    /// a `pango::FontDescription`. The returned value is owned
    ///  by the `Text` actor and it should not be modified or freed
    fn get_font_description(&self) -> Option<pango::FontDescription>;

    /// Retrieves the font name as set by `TextExt::set_font_name`.
    ///
    /// # Returns
    ///
    /// a string containing the font name. The returned
    ///  string is owned by the `Text` actor and should not be
    ///  modified or freed
    fn get_font_name(&self) -> Option<String>;

    /// Retrieves whether the `Text` actor should justify its contents
    /// on both margins.
    ///
    /// # Returns
    ///
    /// `true` if the text should be justified
    fn get_justify(&self) -> bool;

    /// Retrieves the current `pango::Layout` used by a `Text` actor.
    ///
    /// # Returns
    ///
    /// a `pango::Layout`. The returned object is owned by
    ///  the `Text` actor and should not be modified or freed
    fn get_layout(&self) -> Option<pango::Layout>;

    /// Obtains the coordinates where the `Text` will draw the `pango::Layout`
    /// representing the text.
    /// ## `x`
    /// location to store X offset of layout, or `None`
    /// ## `y`
    /// location to store Y offset of layout, or `None`
    fn get_layout_offsets(&self) -> (i32, i32);

    /// Retrieves the alignment of a `Text`, as set by
    /// `TextExt::set_line_alignment`.
    ///
    /// # Returns
    ///
    /// a `pango::Alignment`
    fn get_line_alignment(&self) -> pango::Alignment;

    /// Retrieves the value set using `TextExt::set_line_wrap`.
    ///
    /// # Returns
    ///
    /// `true` if the `Text` actor should wrap
    ///  its contents
    fn get_line_wrap(&self) -> bool;

    /// Retrieves the line wrap mode used by the `Text` actor.
    ///
    /// See text_set_line_wrap_mode ().
    ///
    /// # Returns
    ///
    /// the wrap mode used by the `Text`
    fn get_line_wrap_mode(&self) -> pango::WrapMode;

    /// Gets the maximum length of text that can be set into a text actor.
    ///
    /// See `TextExt::set_max_length`.
    ///
    /// # Returns
    ///
    /// the maximum number of characters.
    fn get_max_length(&self) -> i32;

    /// Retrieves the character to use in place of the actual text
    /// as set by `TextExt::set_password_char`.
    ///
    /// # Returns
    ///
    /// a Unicode character or 0 if the password
    ///  character is not set
    fn get_password_char(&self) -> char;

    /// Retrieves whether a `Text` is selectable or not.
    ///
    /// # Returns
    ///
    /// `true` if the actor is selectable
    fn get_selectable(&self) -> bool;

    /// Retrieves the color of selected text of a `Text` actor.
    /// ## `color`
    /// return location for a `Color`
    fn get_selected_text_color(&self) -> Color;

    /// Retrieves the currently selected text.
    ///
    /// # Returns
    ///
    /// a newly allocated string containing the currently
    ///  selected text, or `None`. Use `g_free` to free the returned
    ///  string.
    fn get_selection(&self) -> Option<String>;

    /// Retrieves the other end of the selection of a `Text` actor,
    /// in characters from the current cursor position.
    ///
    /// # Returns
    ///
    /// the position of the other end of the selection
    fn get_selection_bound(&self) -> i32;

    /// Retrieves the color of the selection of a `Text` actor.
    /// ## `color`
    /// return location for a `Color`
    fn get_selection_color(&self) -> Color;

    /// Retrieves whether the `Text` actor is in single line mode.
    ///
    /// # Returns
    ///
    /// `true` if the `Text` actor is in single line mode
    fn get_single_line_mode(&self) -> bool;

    /// Retrieves a pointer to the current contents of a `Text`
    /// actor.
    ///
    /// If you need a copy of the contents for manipulating, either
    /// use `g_strdup` on the returned string, or use:
    ///
    ///
    /// ```text
    ///    copy = text_get_chars (text, 0, -1);
    /// ```
    ///
    /// Which will return a newly allocated string.
    ///
    /// If the `Text` actor is empty, this function will return
    /// an empty string, and not `None`.
    ///
    /// # Returns
    ///
    /// the contents of the actor. The returned
    ///  string is owned by the `Text` actor and should never be modified
    ///  or freed
    fn get_text(&self) -> Option<String>;

    /// Retrieves whether the contents of the `Text` actor should be
    /// parsed for the Pango text markup.
    ///
    /// # Returns
    ///
    /// `true` if the contents will be parsed for markup
    fn get_use_markup(&self) -> bool;

    /// Inserts `text` into a `Actor` at the given position.
    ///
    /// If `position` is a negative number, the text will be appended
    /// at the end of the current contents of the `Text`.
    ///
    /// The position is expressed in characters, not in bytes.
    /// ## `text`
    /// the text to be inserted
    /// ## `position`
    /// the position of the insertion, or -1
    fn insert_text(&self, text: &str, position: isize);

    /// Inserts `wc` at the current cursor position of a
    /// `Text` actor.
    /// ## `wc`
    /// a Unicode character
    fn insert_unichar(&self, wc: char);

    /// Retrieves the coordinates of the given `position`.
    /// ## `position`
    /// position in characters
    /// ## `x`
    /// return location for the X coordinate, or `None`
    /// ## `y`
    /// return location for the Y coordinate, or `None`
    /// ## `line_height`
    /// return location for the line height, or `None`
    ///
    /// # Returns
    ///
    /// `true` if the conversion was successful
    fn position_to_coords(&self, position: i32) -> Option<(f32, f32, f32)>;

    /// Sets whether a `Text` actor should be activatable.
    ///
    /// An activatable `Text` actor will emit the `Text::activate`
    /// signal whenever the 'Enter' (or 'Return') key is pressed; if it is not
    /// activatable, a new line will be appended to the current content.
    ///
    /// An activatable `Text` must also be set as editable using
    /// `TextExt::set_editable`.
    /// ## `activatable`
    /// whether the `Text` actor should be activatable
    fn set_activatable(&self, activatable: bool);

    /// Sets the attributes list that are going to be applied to the
    /// `Text` contents.
    ///
    /// The `Text` actor will take a reference on the `pango::AttrList`
    /// passed to this function.
    /// ## `attrs`
    /// a `pango::AttrList` or `None` to unset the attributes
    fn set_attributes(&self, attrs: Option<&pango::AttrList>);

    /// Set the `TextBuffer` object which holds the text for
    /// this widget.
    /// ## `buffer`
    /// a `TextBuffer`
    fn set_buffer<P: Is<TextBuffer>>(&self, buffer: &P);

    /// Sets the color of the contents of a `Text` actor.
    ///
    /// The overall opacity of the `Text` actor will be the
    /// result of the alpha value of `color` and the composited
    /// opacity of the actor itself on the scenegraph, as returned
    /// by `ActorExt::get_paint_opacity`.
    /// ## `color`
    /// a `Color`
    fn set_color(&self, color: Color);

    /// Sets the color of the cursor of a `Text` actor.
    ///
    /// If `color` is `None`, the cursor color will be the same as the
    /// text color.
    /// ## `color`
    /// the color of the cursor, or `None` to unset it
    fn set_cursor_color(&self, color: Option<Color>);

    /// Sets the cursor of a `Text` actor at `position`.
    ///
    /// The position is expressed in characters, not in bytes.
    /// ## `position`
    /// the new cursor position, in characters
    fn set_cursor_position(&self, position: i32);

    /// Sets the size of the cursor of a `Text`. The cursor
    /// will only be visible if the `Text:cursor-visible` property
    /// is set to `true`.
    /// ## `size`
    /// the size of the cursor, in pixels, or -1 to use the
    ///  default value
    fn set_cursor_size(&self, size: i32);

    /// Sets whether the cursor of a `Text` actor should be
    /// visible or not.
    ///
    /// The color of the cursor will be the same as the text color
    /// unless `TextExt::set_cursor_color` has been called.
    ///
    /// The size of the cursor can be set using `TextExt::set_cursor_size`.
    ///
    /// The position of the cursor can be changed programmatically using
    /// `TextExt::set_cursor_position`.
    /// ## `cursor_visible`
    /// whether the cursor should be visible
    fn set_cursor_visible(&self, cursor_visible: bool);

    /// Sets whether the `Text` actor should be editable.
    ///
    /// An editable `Text` with key focus set using
    /// `ActorExt::grab_key_focus` or `StageExt::set_key_focus`
    /// will receive key events and will update its contents accordingly.
    /// ## `editable`
    /// whether the `Text` should be editable
    fn set_editable(&self, editable: bool);

    /// Sets the mode used to ellipsize (add an ellipsis: "...") to the
    /// text if there is not enough space to render the entire contents
    /// of a `Text` actor
    /// ## `mode`
    /// a `pango::EllipsizeMode`
    fn set_ellipsize(&self, mode: pango::EllipsizeMode);

    /// Sets `font_desc` as the font description for a `Text`
    ///
    /// The `pango::FontDescription` is copied by the `Text` actor
    /// so you can safely call `pango::FontDescription::free` on it after
    /// calling this function.
    /// ## `font_desc`
    /// a `pango::FontDescription`
    fn set_font_description(&self, font_desc: &mut pango::FontDescription);

    /// Sets the font used by a `Text`. The `font_name` string
    /// must either be `None`, which means that the font name from the
    /// default `Backend` will be used; or be something that can
    /// be parsed by the `pango::FontDescription::from_string` function,
    /// like:
    ///
    ///
    /// ```text
    ///   // Set the font to the system's Sans, 10 points
    ///   text_set_font_name (text, "Sans 10");
    ///
    ///   // Set the font to the system's Serif, 16 pixels
    ///   text_set_font_name (text, "Serif 16px");
    ///
    ///   // Set the font to Helvetica, 10 points
    ///   text_set_font_name (text, "Helvetica 10");
    /// ```
    /// ## `font_name`
    /// a font name, or `None` to set the default font name
    fn set_font_name(&self, font_name: Option<&str>);

    /// Sets whether the text of the `Text` actor should be justified
    /// on both margins. This setting is ignored if it is compiled
    /// against Pango &lt; 1.18.
    /// ## `justify`
    /// whether the text should be justified
    fn set_justify(&self, justify: bool);

    /// Sets the way that the lines of a wrapped label are aligned with
    /// respect to each other. This does not affect the overall alignment
    /// of the label within its allocated or specified width.
    ///
    /// To align a `Text` actor you should add it to a container
    /// that supports alignment, or use the anchor point.
    /// ## `alignment`
    /// A `pango::Alignment`
    fn set_line_alignment(&self, alignment: pango::Alignment);

    /// Sets whether the contents of a `Text` actor should wrap,
    /// if they don't fit the size assigned to the actor.
    /// ## `line_wrap`
    /// whether the contents should wrap
    fn set_line_wrap(&self, line_wrap: bool);

    /// If line wrapping is enabled (see `TextExt::set_line_wrap`) this
    /// function controls how the line wrapping is performed. The default is
    /// `pango::WrapMode::Word` which means wrap on word boundaries.
    /// ## `wrap_mode`
    /// the line wrapping mode
    fn set_line_wrap_mode(&self, wrap_mode: pango::WrapMode);

    /// Sets `markup` as the contents of a `Text`.
    ///
    /// This is a convenience function for setting a string containing
    /// Pango markup, and it is logically equivalent to:
    ///
    ///
    /// ```text
    ///   /&ast; the order is important &ast;/
    ///   text_set_text (TEXT (actor), markup);
    ///   text_set_use_markup (TEXT (actor), true);
    /// ```
    /// ## `markup`
    /// a string containing Pango markup.
    ///  Passing `None` is the same as passing "" (the empty string)
    fn set_markup(&self, markup: Option<&str>);

    /// Sets the maximum allowed length of the contents of the actor. If the
    /// current contents are longer than the given length, then they will be
    /// truncated to fit.
    /// ## `max`
    /// the maximum number of characters allowed in the text actor; 0
    ///  to disable or -1 to set the length of the current string
    fn set_max_length(&self, max: i32);

    /// Sets the character to use in place of the actual text in a
    /// password text actor.
    ///
    /// If `wc` is 0 the text will be displayed as it is entered in the
    /// `Text` actor.
    /// ## `wc`
    /// a Unicode character, or 0 to unset the password character
    fn set_password_char(&self, wc: char);

    /// Sets, or unsets, the pre-edit string. This function is useful
    /// for input methods to display a string (with eventual specific
    /// Pango attributes) before it is entered inside the `Text`
    /// buffer.
    ///
    /// The preedit string and attributes are ignored if the `Text`
    /// actor is not editable.
    ///
    /// This function should not be used by applications
    /// ## `preedit_str`
    /// the pre-edit string, or `None` to unset it
    /// ## `preedit_attrs`
    /// the pre-edit string attributes
    /// ## `cursor_pos`
    /// the cursor position for the pre-edit string
    fn set_preedit_string(
        &self,
        preedit_str: Option<&str>,
        preedit_attrs: Option<&pango::AttrList>,
        cursor_pos: u32,
    );

    /// Sets whether a `Text` actor should be selectable.
    ///
    /// A selectable `Text` will allow selecting its contents using
    /// the pointer or the keyboard.
    /// ## `selectable`
    /// whether the `Text` actor should be selectable
    fn set_selectable(&self, selectable: bool);

    /// Sets the selected text color of a `Text` actor.
    ///
    /// If `color` is `None`, the selected text color will be the same as the
    /// selection color, which then falls back to cursor, and then text color.
    /// ## `color`
    /// the selected text color, or `None` to unset it
    fn set_selected_text_color(&self, color: Option<Color>);

    /// Selects the region of text between `start_pos` and `end_pos`.
    ///
    /// This function changes the position of the cursor to match
    /// `start_pos` and the selection bound to match `end_pos`.
    /// ## `start_pos`
    /// start of the selection, in characters
    /// ## `end_pos`
    /// end of the selection, in characters
    fn set_selection(&self, start_pos: isize, end_pos: isize);

    /// Sets the other end of the selection, starting from the current
    /// cursor position.
    ///
    /// If `selection_bound` is -1, the selection unset.
    /// ## `selection_bound`
    /// the position of the end of the selection, in characters
    fn set_selection_bound(&self, selection_bound: i32);

    /// Sets the color of the selection of a `Text` actor.
    ///
    /// If `color` is `None`, the selection color will be the same as the
    /// cursor color, or if no cursor color is set either then it will be
    /// the same as the text color.
    /// ## `color`
    /// the color of the selection, or `None` to unset it
    fn set_selection_color(&self, color: Option<Color>);

    /// Sets whether a `Text` actor should be in single line mode
    /// or not. Only editable `Text`<!-- -->s can be in single line
    /// mode.
    ///
    /// A text actor in single line mode will not wrap text and will clip
    /// the visible area to the predefined size. The contents of the
    /// text actor will scroll to display the end of the text if its length
    /// is bigger than the allocated width.
    ///
    /// When setting the single line mode the `Text:activatable`
    /// property is also set as a side effect. Instead of entering a new
    /// line character, the text actor will emit the `Text::activate`
    /// signal.
    /// ## `single_line`
    /// whether to enable single line mode
    fn set_single_line_mode(&self, single_line: bool);

    /// Sets the contents of a `Text` actor.
    ///
    /// If the `Text:use-markup` property was set to `true` it
    /// will be reset to `false` as a side effect. If you want to
    /// maintain the `Text:use-markup` you should use the
    /// `TextExt::set_markup` function instead
    /// ## `text`
    /// the text to set. Passing `None` is the same
    ///  as passing "" (the empty string)
    fn set_text(&self, text: Option<&str>);

    /// Sets whether the contents of the `Text` actor contains markup
    /// in <link linkend="PangoMarkupFormat">Pango's text markup language`</link>`.
    ///
    /// Setting `Text:use-markup` on an editable `Text` will
    /// not have any effect except hiding the markup.
    ///
    /// See also `Text:use-markup`.
    /// ## `setting`
    /// `true` if the text should be parsed for markup.
    fn set_use_markup(&self, setting: bool);

    /// Will be set to `true` if `Text:cursor-color` has been set.
    fn get_property_cursor_color_set(&self) -> bool;

    /// Will be set to `true` if `Text:selected-text-color` has been set.
    fn get_property_selected_text_color_set(&self) -> bool;

    /// Will be set to `true` if `Text:selection-color` has been set.
    fn get_property_selection_color_set(&self) -> bool;

    /// The ::activate signal is emitted each time the actor is 'activated'
    /// by the user, normally by pressing the 'Enter' key. The signal is
    /// emitted only if `Text:activatable` is set to `true`.
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::cursor-changed signal is emitted whenever the cursor
    /// position or size changes.
    fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// This signal is emitted when text is deleted from the actor by
    /// the user. It is emitted before `self_` text changes.
    /// ## `start_pos`
    /// the starting position
    /// ## `end_pos`
    /// the end position
    fn connect_delete_text<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_delete_text(&self, start_pos: i32, end_pos: i32);

    //fn connect_insert_text<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    /// The ::text-changed signal is emitted after `actor`'s text changes
    fn connect_text_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cursor_color_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_cursor_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_cursor_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_font_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_justify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_line_wrap_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_text_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selected_text_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selection_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_selection_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_single_line_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Text>> TextExt for O {
    fn activate(&self) -> bool {
        // unsafe { from_glib(ffi::clutter_text_activate(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn coords_to_position(&self, x: f32, y: f32) -> i32 {
        // unsafe { ffi::clutter_text_coords_to_position(self.as_ref().to_glib_none().0, x, y) }
        unimplemented!()
    }

    fn delete_chars(&self, n_chars: u32) {
        // unsafe {
        //     ffi::clutter_text_delete_chars(self.as_ref().to_glib_none().0, n_chars);
        // }
        unimplemented!()
    }

    fn delete_selection(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_delete_selection(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn delete_text(&self, start_pos: isize, end_pos: isize) {
        // unsafe {
        //     ffi::clutter_text_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        // }
        unimplemented!()
    }

    fn get_activatable(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_activatable(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_attributes(&self) -> Option<pango::AttrList> {
        // unsafe {
        //     from_glib_none(ffi::clutter_text_get_attributes(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_buffer(&self) -> Option<TextBuffer> {
        // unsafe { from_glib_none(ffi::clutter_text_get_buffer(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_chars(&self, start_pos: isize, end_pos: isize) -> Option<String> {
        // unsafe {
        //     from_glib_full(ffi::clutter_text_get_chars(
        //         self.as_ref().to_glib_none().0,
        //         start_pos,
        //         end_pos,
        //     ))
        // }
        unimplemented!()
    }

    fn get_color(&self) -> Color {
        // unsafe {
        //     let mut color = InternalColor::uninitialized();
        //     ffi::clutter_text_get_color(self.as_ref().to_glib_none().0, color.to_glib_none_mut().0);
        //     color
        // }
        unimplemented!()
    }

    fn get_cursor_color(&self) -> Color {
        // unsafe {
        //     let mut color = InternalColor::uninitialized();
        //     ffi::clutter_text_get_cursor_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none_mut().0,
        //     );
        //     color
        // }
        unimplemented!()
    }

    fn get_cursor_position(&self) -> i32 {
        // unsafe { ffi::clutter_text_get_cursor_position(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_cursor_rect(&self) -> InternalRect {
        // unsafe {
        //     let mut rect = InternalRect::uninitialized();
        //     ffi::clutter_text_get_cursor_rect(
        //         self.as_ref().to_glib_none().0,
        //         rect.to_glib_none_mut().0,
        //     );
        //     rect
        // }
        unimplemented!()
    }

    fn get_cursor_size(&self) -> u32 {
        // unsafe { ffi::clutter_text_get_cursor_size(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_cursor_visible(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_cursor_visible(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_editable(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_editable(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_ellipsize(&self) -> pango::EllipsizeMode {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_ellipsize(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_font_description(&self) -> Option<pango::FontDescription> {
        // unsafe {
        //     from_glib_full(ffi::clutter_text_get_font_description(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_font_name(&self) -> Option<String> {
        // unsafe {
        //     from_glib_none(ffi::clutter_text_get_font_name(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_justify(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_justify(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_layout(&self) -> Option<pango::Layout> {
        // unsafe { from_glib_none(ffi::clutter_text_get_layout(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     ffi::clutter_text_get_layout_offsets(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     (x, y)
        // }
        unimplemented!()
    }

    fn get_line_alignment(&self) -> pango::Alignment {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_line_alignment(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_line_wrap(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_line_wrap(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_line_wrap_mode(&self) -> pango::WrapMode {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_line_wrap_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_max_length(&self) -> i32 {
        // unsafe { ffi::clutter_text_get_max_length(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_password_char(&self) -> char {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_password_char(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_selectable(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_selectable(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_selected_text_color(&self) -> Color {
        // unsafe {
        //     let mut color = InternalColor::uninitialized();
        //     ffi::clutter_text_get_selected_text_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none_mut().0,
        //     );
        //     color
        // }
        unimplemented!()
    }

    fn get_selection(&self) -> Option<String> {
        // unsafe {
        //     from_glib_full(ffi::clutter_text_get_selection(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_selection_bound(&self) -> i32 {
        // unsafe { ffi::clutter_text_get_selection_bound(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_selection_color(&self) -> Color {
        // unsafe {
        //     let mut color = InternalColor::uninitialized();
        //     ffi::clutter_text_get_selection_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none_mut().0,
        //     );
        //     color
        // }
        unimplemented!()
    }

    fn get_single_line_mode(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_single_line_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_text(&self) -> Option<String> {
        // unsafe { from_glib_none(ffi::clutter_text_get_text(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_use_markup(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_text_get_use_markup(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn insert_text(&self, text: &str, position: isize) {
        // unsafe {
        //     ffi::clutter_text_insert_text(
        //         self.as_ref().to_glib_none().0,
        //         text.to_glib_none().0,
        //         position,
        //     );
        // }
        unimplemented!()
    }

    fn insert_unichar(&self, wc: char) {
        // unsafe {
        //     ffi::clutter_text_insert_unichar(self.as_ref().to_glib_none().0, wc.to_glib());
        // }
        unimplemented!()
    }

    fn position_to_coords(&self, position: i32) -> Option<(f32, f32, f32)> {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     let mut line_height = mem::MaybeUninit::uninit();
        //     let ret = from_glib(ffi::clutter_text_position_to_coords(
        //         self.as_ref().to_glib_none().0,
        //         position,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //         line_height.as_mut_ptr(),
        //     ));
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     let line_height = line_height.assume_init();
        //     if ret {
        //         Some((x, y, line_height))
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }

    fn set_activatable(&self, activatable: bool) {
        // unsafe {
        //     ffi::clutter_text_set_activatable(
        //         self.as_ref().to_glib_none().0,
        //         activatable.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_attributes(&self, attrs: Option<&pango::AttrList>) {
        // unsafe {
        //     ffi::clutter_text_set_attributes(
        //         self.as_ref().to_glib_none().0,
        //         attrs.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_buffer<P: Is<TextBuffer>>(&self, buffer: &P) {
        // unsafe {
        //     ffi::clutter_text_set_buffer(
        //         self.as_ref().to_glib_none().0,
        //         buffer.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_color(&self, value: Color) {
        // let color = {
        //     let RgbaColor {
        //         red,
        //         green,
        //         blue,
        //         alpha,
        //     } = value.into();
        //     InternalColor::new(red, green, blue, alpha)
        // };

        // unsafe {
        //     ffi::clutter_text_set_color(self.as_ref().to_glib_none().0, color.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_cursor_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(InternalColor::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };
        // unsafe {
        //     ffi::clutter_text_set_cursor_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_cursor_position(&self, position: i32) {
        // unsafe {
        //     ffi::clutter_text_set_cursor_position(self.as_ref().to_glib_none().0, position);
        // }
        unimplemented!()
    }

    fn set_cursor_size(&self, size: i32) {
        // unsafe {
        //     ffi::clutter_text_set_cursor_size(self.as_ref().to_glib_none().0, size);
        // }
        unimplemented!()
    }

    fn set_cursor_visible(&self, cursor_visible: bool) {
        // unsafe {
        //     ffi::clutter_text_set_cursor_visible(
        //         self.as_ref().to_glib_none().0,
        //         cursor_visible.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_editable(&self, editable: bool) {
        // unsafe {
        //     ffi::clutter_text_set_editable(self.as_ref().to_glib_none().0, editable.to_glib());
        // }
        unimplemented!()
    }

    fn set_ellipsize(&self, mode: pango::EllipsizeMode) {
        // unsafe {
        //     ffi::clutter_text_set_ellipsize(self.as_ref().to_glib_none().0, mode.to_glib());
        // }
        unimplemented!()
    }

    fn set_font_description(&self, font_desc: &mut pango::FontDescription) {
        // unsafe {
        //     ffi::clutter_text_set_font_description(
        //         self.as_ref().to_glib_none().0,
        //         font_desc.to_glib_none_mut().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_font_name(&self, font_name: Option<&str>) {
        // unsafe {
        //     ffi::clutter_text_set_font_name(
        //         self.as_ref().to_glib_none().0,
        //         font_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_justify(&self, justify: bool) {
        // unsafe {
        //     ffi::clutter_text_set_justify(self.as_ref().to_glib_none().0, justify.to_glib());
        // }
        unimplemented!()
    }

    fn set_line_alignment(&self, alignment: pango::Alignment) {
        // unsafe {
        //     ffi::clutter_text_set_line_alignment(
        //         self.as_ref().to_glib_none().0,
        //         alignment.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_line_wrap(&self, line_wrap: bool) {
        // unsafe {
        //     ffi::clutter_text_set_line_wrap(self.as_ref().to_glib_none().0, line_wrap.to_glib());
        // }
        unimplemented!()
    }

    fn set_line_wrap_mode(&self, wrap_mode: pango::WrapMode) {
        // unsafe {
        //     ffi::clutter_text_set_line_wrap_mode(
        //         self.as_ref().to_glib_none().0,
        //         wrap_mode.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_markup(&self, markup: Option<&str>) {
        // unsafe {
        //     ffi::clutter_text_set_markup(self.as_ref().to_glib_none().0, markup.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_max_length(&self, max: i32) {
        // unsafe {
        //     ffi::clutter_text_set_max_length(self.as_ref().to_glib_none().0, max);
        // }
        unimplemented!()
    }

    fn set_password_char(&self, wc: char) {
        // unsafe {
        //     ffi::clutter_text_set_password_char(self.as_ref().to_glib_none().0, wc.to_glib());
        // }
        unimplemented!()
    }

    fn set_preedit_string(
        &self,
        preedit_str: Option<&str>,
        preedit_attrs: Option<&pango::AttrList>,
        cursor_pos: u32,
    ) {
        // unsafe {
        //     ffi::clutter_text_set_preedit_string(
        //         self.as_ref().to_glib_none().0,
        //         preedit_str.to_glib_none().0,
        //         preedit_attrs.to_glib_none().0,
        //         cursor_pos,
        //     );
        // }
        unimplemented!()
    }

    fn set_selectable(&self, selectable: bool) {
        // unsafe {
        //     ffi::clutter_text_set_selectable(self.as_ref().to_glib_none().0, selectable.to_glib());
        // }
        unimplemented!()
    }

    fn set_selected_text_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(InternalColor::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };
        // unsafe {
        //     ffi::clutter_text_set_selected_text_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_selection(&self, start_pos: isize, end_pos: isize) {
        // unsafe {
        //     ffi::clutter_text_set_selection(self.as_ref().to_glib_none().0, start_pos, end_pos);
        // }
    }

    fn set_selection_bound(&self, selection_bound: i32) {
        // unsafe {
        //     ffi::clutter_text_set_selection_bound(self.as_ref().to_glib_none().0, selection_bound);
        // }
        unimplemented!()
    }

    fn set_selection_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(InternalColor::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };
        // unsafe {
        //     ffi::clutter_text_set_selection_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_single_line_mode(&self, single_line: bool) {
        // unsafe {
        //     ffi::clutter_text_set_single_line_mode(
        //         self.as_ref().to_glib_none().0,
        //         single_line.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_text(&self, text: Option<&str>) {
        // unsafe {
        //     ffi::clutter_text_set_text(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_use_markup(&self, setting: bool) {
        // unsafe {
        //     ffi::clutter_text_set_use_markup(self.as_ref().to_glib_none().0, setting.to_glib());
        // }
        unimplemented!()
    }

    fn get_property_cursor_color_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"cursor-color-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `cursor-color-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_selected_text_color_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"selected-text-color-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `selected-text-color-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_selection_color_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"selection-color-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `selection-color-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_cursor_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_delete_text<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn emit_delete_text(&self, start_pos: i32, end_pos: i32) {
        // let _ = unsafe {
        //     glib::Object::from_glib_borrow(self.as_ptr() as *mut gobject_sys::GObject)
        //         .emit("delete-text", &[&start_pos, &end_pos])
        //         .unwrap()
        // };
        unimplemented!()
    }

    //fn connect_insert_text<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented position: *.Pointer
    //}

    fn connect_text_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_position_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_cursor_visible_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_editable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_font_description_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_font_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_justify_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_line_alignment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_line_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_line_wrap_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_max_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_password_char_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selected_text_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selected_text_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selection_bound_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selection_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_selection_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_single_line_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_use_markup_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text")
    }
}
