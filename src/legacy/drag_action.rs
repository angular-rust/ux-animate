use crate::{Action, Actor, ActorMeta, DragAxis, InternalRect, ModifierType};
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
    pub struct DragAction(Object<ffi::ClutterDragAction, ffi::ClutterDragActionClass, DragActionClass>) @extends Action, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_drag_action_get_type(),
    }
}

impl DragAction {
    /// Creates a new `DragAction` instance
    ///
    /// # Returns
    ///
    /// the newly created `DragAction`
    pub fn new() -> DragAction {
        unsafe { Action::from_glib_none(ffi::clutter_drag_action_new()).unsafe_cast() }
    }
}

impl Default for DragAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `DragAction` methods.
///
/// # Implementors
///
/// [`DragAction`](struct.DragAction.html)
pub trait DragActionExt: 'static {
    /// Retrieves the "drag area" associated with `self`, that
    /// is a `Rect` that constrains the actor movements,
    /// in parents coordinates.
    /// ## `drag_area`
    /// a `Rect` to be filled
    ///
    /// # Returns
    ///
    /// `true` if the actor is actually constrained (and thus
    ///  `drag_area` is valid), `false` otherwise
    fn get_drag_area(&self) -> Option<InternalRect>;

    /// Retrieves the axis constraint set by `DragActionExt::set_drag_axis`
    ///
    /// # Returns
    ///
    /// the axis constraint
    fn get_drag_axis(&self) -> DragAxis;

    /// Retrieves the drag handle set by `DragActionExt::set_drag_handle`
    ///
    /// # Returns
    ///
    /// a `Actor`, used as the drag
    ///  handle, or `None` if none was set
    fn get_drag_handle(&self) -> Option<Actor>;

    /// Retrieves the values set by `DragActionExt::set_drag_threshold`.
    ///
    /// If the `DragAction:x-drag-threshold` property or the
    /// `DragAction:y-drag-threshold` property have been set to -1 then
    /// this function will return the default drag threshold value as stored
    /// by the `Settings:dnd-drag-threshold` property of `Settings`.
    /// ## `x_threshold`
    /// return location for the horizontal drag
    ///  threshold value, in pixels
    /// ## `y_threshold`
    /// return location for the vertical drag
    ///  threshold value, in pixels
    fn get_drag_threshold(&self) -> (u32, u32);

    /// Retrieves the coordinates, in stage space, of the latest motion
    /// event during the dragging
    /// ## `motion_x`
    /// return location for the latest motion
    ///  event's X coordinate
    /// ## `motion_y`
    /// return location for the latest motion
    ///  event's Y coordinate
    fn get_motion_coords(&self) -> (f32, f32);

    /// Retrieves the coordinates, in stage space, of the press event
    /// that started the dragging
    /// ## `press_x`
    /// return location for the press event's X coordinate
    /// ## `press_y`
    /// return location for the press event's Y coordinate
    fn get_press_coords(&self) -> (f32, f32);

    /// Sets `drag_area` to constrain the dragging of the actor associated
    /// with `self`, so that it position is always within `drag_area`, expressed
    /// in parent's coordinates.
    /// If `drag_area` is `None`, the actor is not constrained.
    /// ## `drag_area`
    /// a `Rect`
    fn set_drag_area(&self, drag_area: Option<&InternalRect>);

    /// Restricts the dragging action to a specific axis
    /// ## `axis`
    /// the axis to constraint the dragging to
    fn set_drag_axis(&self, axis: DragAxis);

    /// Sets the actor to be used as the drag handle.
    /// ## `handle`
    /// a `Actor`, or `None` to unset
    fn set_drag_handle<P: IsA<Actor>>(&self, handle: Option<&P>);

    /// Sets the horizontal and vertical drag thresholds that must be
    /// cleared by the pointer before `self` can begin the dragging.
    ///
    /// If `x_threshold` or `y_threshold` are set to -1 then the default
    /// drag threshold stored in the `Settings:dnd-drag-threshold`
    /// property of `Settings` will be used.
    /// ## `x_threshold`
    /// a distance on the horizontal axis, in pixels, or
    ///  -1 to use the default drag threshold from `Settings`
    /// ## `y_threshold`
    /// a distance on the vertical axis, in pixels, or
    ///  -1 to use the default drag threshold from `Settings`
    fn set_drag_threshold(&self, x_threshold: i32, y_threshold: i32);

    /// Whether the `DragAction:drag-area` property has been set.
    fn get_property_drag_area_set(&self) -> bool;

    /// The horizontal threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// horizontally at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the default threshold
    /// stored in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn get_property_x_drag_threshold(&self) -> i32;

    /// The horizontal threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// horizontally at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the default threshold
    /// stored in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn set_property_x_drag_threshold(&self, x_drag_threshold: i32);

    /// The vertical threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// vertically at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the value stored
    /// in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn get_property_y_drag_threshold(&self) -> i32;

    /// The vertical threshold, in pixels, that the cursor must travel
    /// in order to begin a drag action.
    ///
    /// When set to a positive value, `DragAction` will only emit
    /// `DragAction::drag-begin` if the pointer has moved
    /// vertically at least of the given amount of pixels since
    /// the button press event.
    ///
    /// When set to -1, `DragAction` will use the value stored
    /// in the `Settings:dnd-drag-threshold` property of
    /// `Settings`.
    ///
    /// When read, this property will always return a valid drag
    /// threshold, either as set or the default one.
    fn set_property_y_drag_threshold(&self, y_drag_threshold: i32);

    /// The ::drag-begin signal is emitted when the `DragAction`
    /// starts the dragging
    ///
    /// The emission of this signal can be delayed by using the
    /// `DragAction:x-drag-threshold` and
    /// `DragAction:y-drag-threshold` properties
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `event_x`
    /// the X coordinate (in stage space) of the press event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the press event
    /// ## `modifiers`
    /// the modifiers of the press event
    fn connect_drag_begin<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-end signal is emitted at the end of the dragging,
    /// when the pointer button's is released
    ///
    /// This signal is emitted if and only if the `DragAction::drag-begin`
    /// signal has been emitted first
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `event_x`
    /// the X coordinate (in stage space) of the release event
    /// ## `event_y`
    /// the Y coordinate (in stage space) of the release event
    /// ## `modifiers`
    /// the modifiers of the release event
    fn connect_drag_end<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-motion signal is emitted for each motion event after
    /// the `DragAction::drag-begin` signal has been emitted.
    ///
    /// The components of the distance between the press event and the
    /// latest motion event are computed in the actor's coordinate space,
    /// to take into account eventual transformations. If you want the
    /// stage coordinates of the latest motion event you can use
    /// `DragActionExt::get_motion_coords`.
    ///
    /// The default handler of the signal will call `ActorExt::move_by`
    /// either on `actor` or, if set, of `DragAction:drag-handle` using
    /// the `delta_x` and `delta_y` components of the dragging motion. If you
    /// want to override the default behaviour, you can connect to the
    /// `DragAction::drag-progress` signal and return `false` from the
    /// handler.
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `delta_x`
    /// the X component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    /// ## `delta_y`
    /// the Y component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    fn connect_drag_motion<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::drag-progress signal is emitted for each motion event after
    /// the `DragAction::drag-begin` signal has been emitted.
    ///
    /// The components of the distance between the press event and the
    /// latest motion event are computed in the actor's coordinate space,
    /// to take into account eventual transformations. If you want the
    /// stage coordinates of the latest motion event you can use
    /// `DragActionExt::get_motion_coords`.
    ///
    /// The default handler will emit `DragAction::drag-motion`,
    /// if `DragAction::drag-progress` emission returns `true`.
    /// ## `actor`
    /// the `Actor` attached to the action
    /// ## `delta_x`
    /// the X component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    /// ## `delta_y`
    /// the Y component of the distance between the press event
    ///  that began the dragging and the current position of the pointer,
    ///  as of the latest motion event
    ///
    /// # Returns
    ///
    /// `true` if the drag should continue, and `false`
    ///  if it should be stopped.
    fn connect_drag_progress<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_drag_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_drag_area_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_drag_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_drag_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_y_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<DragAction>> DragActionExt for O {
    fn get_drag_area(&self) -> Option<InternalRect> {
        unsafe {
            let mut drag_area = InternalRect::uninitialized();
            let ret = from_glib(ffi::clutter_drag_action_get_drag_area(
                self.as_ref().to_glib_none().0,
                drag_area.to_glib_none_mut().0,
            ));
            if ret {
                Some(drag_area)
            } else {
                None
            }
        }
    }

    fn get_drag_axis(&self) -> DragAxis {
        unsafe {
            from_glib(ffi::clutter_drag_action_get_drag_axis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_drag_handle(&self) -> Option<Actor> {
        unsafe {
            from_glib_none(ffi::clutter_drag_action_get_drag_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_drag_threshold(&self) -> (u32, u32) {
        unsafe {
            let mut x_threshold = mem::MaybeUninit::uninit();
            let mut y_threshold = mem::MaybeUninit::uninit();
            ffi::clutter_drag_action_get_drag_threshold(
                self.as_ref().to_glib_none().0,
                x_threshold.as_mut_ptr(),
                y_threshold.as_mut_ptr(),
            );
            let x_threshold = x_threshold.assume_init();
            let y_threshold = y_threshold.assume_init();
            (x_threshold, y_threshold)
        }
    }

    fn get_motion_coords(&self) -> (f32, f32) {
        unsafe {
            let mut motion_x = mem::MaybeUninit::uninit();
            let mut motion_y = mem::MaybeUninit::uninit();
            ffi::clutter_drag_action_get_motion_coords(
                self.as_ref().to_glib_none().0,
                motion_x.as_mut_ptr(),
                motion_y.as_mut_ptr(),
            );
            let motion_x = motion_x.assume_init();
            let motion_y = motion_y.assume_init();
            (motion_x, motion_y)
        }
    }

    fn get_press_coords(&self) -> (f32, f32) {
        unsafe {
            let mut press_x = mem::MaybeUninit::uninit();
            let mut press_y = mem::MaybeUninit::uninit();
            ffi::clutter_drag_action_get_press_coords(
                self.as_ref().to_glib_none().0,
                press_x.as_mut_ptr(),
                press_y.as_mut_ptr(),
            );
            let press_x = press_x.assume_init();
            let press_y = press_y.assume_init();
            (press_x, press_y)
        }
    }

    fn set_drag_area(&self, drag_area: Option<&InternalRect>) {
        unsafe {
            ffi::clutter_drag_action_set_drag_area(
                self.as_ref().to_glib_none().0,
                drag_area.to_glib_none().0,
            );
        }
    }

    fn set_drag_axis(&self, axis: DragAxis) {
        unsafe {
            ffi::clutter_drag_action_set_drag_axis(self.as_ref().to_glib_none().0, axis.to_glib());
        }
    }

    fn set_drag_handle<P: IsA<Actor>>(&self, handle: Option<&P>) {
        unsafe {
            ffi::clutter_drag_action_set_drag_handle(
                self.as_ref().to_glib_none().0,
                handle.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_drag_threshold(&self, x_threshold: i32, y_threshold: i32) {
        unsafe {
            ffi::clutter_drag_action_set_drag_threshold(
                self.as_ref().to_glib_none().0,
                x_threshold,
                y_threshold,
            );
        }
    }

    fn get_property_drag_area_set(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"drag-area-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `drag-area-set` getter")
                .unwrap()
        }
    }

    fn get_property_x_drag_threshold(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"x-drag-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `x-drag-threshold` getter")
                .unwrap()
        }
    }

    fn set_property_x_drag_threshold(&self, x_drag_threshold: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"x-drag-threshold\0".as_ptr() as *const _,
                Value::from(&x_drag_threshold).to_glib_none().0,
            );
        }
    }

    fn get_property_y_drag_threshold(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"y-drag-threshold\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `y-drag-threshold` getter")
                .unwrap()
        }
    }

    fn set_property_y_drag_threshold(&self, y_drag_threshold: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"y-drag-threshold\0".as_ptr() as *const _,
                Value::from(&y_drag_threshold).to_glib_none().0,
            );
        }
    }

    fn connect_drag_begin<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_begin_trampoline<
            P,
            F: Fn(&P, &Actor, f32, f32, ModifierType) + 'static,
        >(
            this: *mut ffi::ClutterDragAction,
            actor: *mut ffi::ClutterActor,
            event_x: libc::c_float,
            event_y: libc::c_float,
            modifiers: ffi::ClutterModifierType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DragAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                event_x,
                event_y,
                from_glib(modifiers),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_end<F: Fn(&Self, &Actor, f32, f32, ModifierType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_end_trampoline<
            P,
            F: Fn(&P, &Actor, f32, f32, ModifierType) + 'static,
        >(
            this: *mut ffi::ClutterDragAction,
            actor: *mut ffi::ClutterActor,
            event_x: libc::c_float,
            event_y: libc::c_float,
            modifiers: ffi::ClutterModifierType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DragAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                event_x,
                event_y,
                from_glib(modifiers),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_motion<F: Fn(&Self, &Actor, f32, f32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_motion_trampoline<P, F: Fn(&P, &Actor, f32, f32) + 'static>(
            this: *mut ffi::ClutterDragAction,
            actor: *mut ffi::ClutterActor,
            delta_x: libc::c_float,
            delta_y: libc::c_float,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DragAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                delta_x,
                delta_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-motion\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_motion_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_progress<F: Fn(&Self, &Actor, f32, f32) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drag_progress_trampoline<
            P,
            F: Fn(&P, &Actor, f32, f32) -> bool + 'static,
        >(
            this: *mut ffi::ClutterDragAction,
            actor: *mut ffi::ClutterActor,
            delta_x: libc::c_float,
            delta_y: libc::c_float,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(
                &DragAction::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(actor),
                delta_x,
                delta_y,
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_drag_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_drag_area_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drag-area\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drag_area_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_drag_area_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_drag_area_set_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drag-area-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drag_area_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_drag_axis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_drag_axis_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drag-axis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drag_axis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_drag_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_drag_handle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::drag-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_drag_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_x_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_x_drag_threshold_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::x-drag-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_x_drag_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_y_drag_threshold_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_y_drag_threshold_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterDragAction,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<DragAction>,
        {
            let f: &F = &*(f as *const F);
            f(&DragAction::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::y-drag-threshold\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_y_drag_threshold_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DragAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DragAction")
    }
}
