use crate::{Action, Actor, ActorMeta, LongPressState, ModifierType};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    StaticType, Value,
};
use std::boxed::Box as Box_;
use std::{fmt, mem, mem::transmute};

glib_wrapper! {
    pub struct ClickAction(Object<ffi::ClutterClickAction, ffi::ClutterClickActionClass, ClickActionClass>) @extends Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_click_action_get_type(),
    }
}

impl ClickAction {
    /// Creates a new `ClickAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `ClickAction`
    pub fn new() -> ClickAction {
        unsafe { Action::from_glib_none(ffi::clutter_click_action_new()).unsafe_cast() }
    }
}

impl Default for ClickAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `ClickAction` methods.
///
/// # Implementors
///
/// [`ClickAction`](struct.ClickAction.html)
pub trait ClickActionExt: 'static {
    /// Retrieves the button that was pressed.
    ///
    /// - 1 - left mouse button in a right-handed configuration, or the right mouse button in a left-handed configuration
    /// - 2 - scroll wheel button
    /// - 3 - right mouse button in a right-handed configuration, or the left mouse button in a left-handed configuration
    ///
    /// # Returns
    ///
    /// the button value
    fn get_button(&self) -> u32;

    /// Retrieves the screen coordinates of the button press.
    /// ## `press_x`
    /// return location for the X coordinate, or `None`
    /// ## `press_y`
    /// return location for the Y coordinate, or `None`
    fn get_coords(&self) -> (f32, f32);

    /// Retrieves the modifier state of the click action.
    ///
    /// # Returns
    ///
    /// the modifier state parameter, or 0
    fn get_state(&self) -> ModifierType;

    /// Emulates a release of the pointer button, which ungrabs the pointer
    /// and unsets the `ClickAction:pressed` state.
    ///
    /// This function will also cancel the long press gesture if one was
    /// initiated.
    ///
    /// This function is useful to break a grab, for instance after a certain
    /// amount of time has passed.
    fn release(&self);

    /// Whether the clickable actor has the pointer grabbed
    fn get_property_held(&self) -> bool;

    /// The minimum duration of a press for it to be recognized as a long
    /// press gesture, in milliseconds.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:long-press-duration` property.
    fn get_property_long_press_duration(&self) -> i32;

    /// The minimum duration of a press for it to be recognized as a long
    /// press gesture, in milliseconds.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:long-press-duration` property.
    fn set_property_long_press_duration(&self, long_press_duration: i32);

    /// The maximum allowed distance that can be covered (on both axes) before
    /// a long press gesture is cancelled, in pixels.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:dnd-drag-threshold` property.
    fn get_property_long_press_threshold(&self) -> i32;

    /// The maximum allowed distance that can be covered (on both axes) before
    /// a long press gesture is cancelled, in pixels.
    ///
    /// A value of -1 will make the `ClickAction` use the value of
    /// the `Settings:dnd-drag-threshold` property.
    fn set_property_long_press_threshold(&self, long_press_threshold: i32);

    /// Whether the clickable actor should be in "pressed" state
    fn get_property_pressed(&self) -> bool;

    /// The ::clicked signal is emitted when the `Actor` to which
    /// a `ClickAction` has been applied should respond to a
    /// pointer button press and release events
    /// ## `actor`
    /// the `Actor` attached to the `action`
    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::long-press signal is emitted during the long press gesture
    /// handling.
    ///
    /// This signal can be emitted multiple times with different states.
    ///
    /// The `LongPressState::Query` state will be emitted on button presses,
    /// and its return value will determine whether the long press handling
    /// should be initiated. If the signal handlers will return `true`, the
    /// `LongPressState::Query` state will be followed either by a signal
    /// emission with the `LongPressState::Activate` state if the long press
    /// constraints were respected, or by a signal emission with the
    /// `LongPressState::Cancel` state if the long press was cancelled.
    ///
    /// It is possible to forcibly cancel a long press detection using
    /// `ClickActionExt::release`.
    /// ## `actor`
    /// the `Actor` attached to the `action`
    /// ## `state`
    /// the long press state
    ///
    /// # Returns
    ///
    /// Only the `LongPressState::Query` state uses the
    ///  returned value of the handler; other states will ignore it
    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ClickAction>> ClickActionExt for O {
    fn get_button(&self) -> u32 {
        unsafe { ffi::clutter_click_action_get_button(self.as_ref().to_glib_none().0) }
    }

    fn get_coords(&self) -> (f32, f32) {
        unsafe {
            let mut press_x = mem::MaybeUninit::uninit();
            let mut press_y = mem::MaybeUninit::uninit();
            ffi::clutter_click_action_get_coords(
                self.as_ref().to_glib_none().0,
                press_x.as_mut_ptr(),
                press_y.as_mut_ptr(),
            );
            let press_x = press_x.assume_init();
            let press_y = press_y.assume_init();
            (press_x, press_y)
        }
    }

    fn get_state(&self) -> ModifierType {
        unsafe {
            from_glib(ffi::clutter_click_action_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn release(&self) {
        unsafe {
            ffi::clutter_click_action_release(self.as_ref().to_glib_none().0);
        }
    }

    fn get_property_held(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"held\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `held` getter")
                .unwrap()
        }
    }

    fn get_property_long_press_duration(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `long-press-duration` getter")
                .unwrap()
        }
    }

    fn set_property_long_press_duration(&self, long_press_duration: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-duration\0".as_ptr() as *const _,
                Value::from(&long_press_duration).to_glib_none().0,
            );
        }
    }

    fn get_property_long_press_threshold(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `long-press-threshold` getter")
                .unwrap()
        }
    }

    fn set_property_long_press_threshold(&self, long_press_threshold: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"long-press-threshold\0".as_ptr() as *const _,
                Value::from(&long_press_threshold).to_glib_none().0,
            );
        }
    }

    fn get_property_pressed(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"pressed\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `pressed` getter")
                .unwrap()
        }
    }

    fn connect_clicked<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn clicked_trampoline<P, F: Fn(&P, &Actor) + 'static>(
            this: *mut ffi::ClutterClickAction,
            actor: *mut ffi::ClutterActor,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ClickAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"clicked\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    clicked_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_long_press<F: Fn(&Self, &Actor, LongPressState) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn long_press_trampoline<
            P,
            F: Fn(&P, &Actor, LongPressState) -> bool + 'static,
        >(
            this: *mut ffi::ClutterClickAction,
            actor: *mut ffi::ClutterActor,
            state: ffi::ClutterLongPressState,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ClickAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                from_glib(state),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"long-press\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    long_press_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_held_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_held_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::held\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_held_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_long_press_duration_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_press_duration_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::long-press-duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_press_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_long_press_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_long_press_threshold_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::long-press-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_long_press_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pressed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pressed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterClickAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ClickAction>,
        {
            let f: &F = &*(f as *const F);
            f(&ClickAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pressed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pressed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ClickAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClickAction")
    }
}
