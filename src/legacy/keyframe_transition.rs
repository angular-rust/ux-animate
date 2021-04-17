use glib::{
    object::{Cast, IsA},
    translate::*,
};
use std::{fmt, mem};
// Scriptable
use crate::{AnimationMode, PropertyTransition, Timeline, Transition};

// TODO: , @implements Scriptable
glib_wrapper! {
    pub struct KeyframeTransition(Object<ffi::ClutterKeyframeTransition, ffi::ClutterKeyframeTransitionClass, KeyframeTransitionClass>) @extends PropertyTransition, Transition, Timeline;

    match fn {
        get_type => || ffi::clutter_keyframe_transition_get_type(),
    }
}

impl KeyframeTransition {
    /// Creates a new `KeyframeTransition` for `property_name`.
    /// ## `property_name`
    /// the property to animate
    ///
    /// # Returns
    ///
    /// the newly allocated
    ///  `KeyframeTransition` instance. Use `gobject::ObjectExt::unref` when
    ///  done to free its resources.
    pub fn new(property_name: &str) -> KeyframeTransition {
        unsafe {
            Transition::from_glib_full(ffi::clutter_keyframe_transition_new(
                property_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

/// Trait containing all `KeyframeTransition` methods.
///
/// # Implementors
///
/// [`KeyframeTransition`](struct.KeyframeTransition.html)
pub trait KeyframeTransitionExt: 'static {
    /// Removes all key frames from `self`.
    fn clear(&self);

    /// Retrieves the details of the key frame at `index_` inside `self`.
    ///
    /// The `self` must already have key frames set, and `index_` must be
    /// smaller than the number of key frames.
    /// ## `index_`
    /// the index of the key frame
    /// ## `key`
    /// return location for the key, or `None`
    /// ## `mode`
    /// return location for the easing mode, or `None`
    /// ## `value`
    /// a `gobject::Value` initialized with the type of
    ///  the values
    fn get_key_frame(&self, index_: u32) -> (f64, AnimationMode, glib::Value);

    /// Retrieves the number of key frames inside `self`.
    ///
    /// # Returns
    ///
    /// the number of key frames
    fn get_n_key_frames(&self) -> u32;

    //fn set(&self, gtype: glib::types::Type, n_key_frames: u32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets the details of the key frame at `index_` inside `self`.
    ///
    /// The `self` must already have a key frame at `index_`, and `index_`
    /// must be smaller than the number of key frames inside `self`.
    /// ## `index_`
    /// the index of the key frame
    /// ## `key`
    /// the key of the key frame
    /// ## `mode`
    /// the easing mode of the key frame
    /// ## `value`
    /// a `gobject::Value` containing the value of the key frame
    fn set_key_frame(&self, index_: u32, key: f64, mode: AnimationMode, value: &glib::Value);

    /// Sets the keys for each key frame inside `self`.
    ///
    /// If `self` does not hold any key frame, `n_key_frames` key frames
    /// will be created; if `self` already has key frames, `key_frames` must
    /// have at least as many elements as the number of key frames.
    /// ## `n_key_frames`
    /// the number of values
    /// ## `key_frames`
    /// an array of keys between 0.0
    ///  and 1.0, one for each key frame
    fn set_key_frames(&self, key_frames: &[f64]);

    //fn set_modes(&self, modes: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 28 });

    /// Sets the values for each key frame inside `self`.
    ///
    /// If `self` does not hold any key frame, `n_values` key frames will
    /// be created; if `self` already has key frames, `values` must have
    /// at least as many elements as the number of key frames.
    /// ## `n_values`
    /// the number of values
    /// ## `values`
    /// an array of values, one for each
    ///  key frame
    fn set_values(&self, values: &[&glib::Value]);
}

impl<O: IsA<KeyframeTransition>> KeyframeTransitionExt for O {
    fn clear(&self) {
        unsafe {
            ffi::clutter_keyframe_transition_clear(self.as_ref().to_glib_none().0);
        }
    }

    fn get_key_frame(&self, index_: u32) -> (f64, AnimationMode, glib::Value) {
        unsafe {
            let mut key = mem::MaybeUninit::uninit();
            let mut mode = mem::MaybeUninit::uninit();
            let mut value = glib::Value::uninitialized();
            ffi::clutter_keyframe_transition_get_key_frame(
                self.as_ref().to_glib_none().0,
                index_,
                key.as_mut_ptr(),
                mode.as_mut_ptr(),
                value.to_glib_none_mut().0,
            );
            let key = key.assume_init();
            let mode = mode.assume_init();
            (key, from_glib(mode), value)
        }
    }

    fn get_n_key_frames(&self) -> u32 {
        unsafe { ffi::clutter_keyframe_transition_get_n_key_frames(self.as_ref().to_glib_none().0) }
    }

    //fn set(&self, gtype: glib::types::Type, n_key_frames: u32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_keyframe_transition_set() }
    //}

    fn set_key_frame(&self, index_: u32, key: f64, mode: AnimationMode, value: &glib::Value) {
        unsafe {
            ffi::clutter_keyframe_transition_set_key_frame(
                self.as_ref().to_glib_none().0,
                index_,
                key,
                mode.to_glib(),
                value.to_glib_none().0,
            );
        }
    }

    fn set_key_frames(&self, key_frames: &[f64]) {
        let n_key_frames = key_frames.len() as u32;
        unsafe {
            ffi::clutter_keyframe_transition_set_key_frames(
                self.as_ref().to_glib_none().0,
                n_key_frames,
                key_frames.to_glib_none().0,
            );
        }
    }

    //fn set_modes(&self, modes: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 28 }) {
    //    unsafe { TODO: call clutter_sys:clutter_keyframe_transition_set_modes() }
    //}

    fn set_values(&self, values: &[&glib::Value]) {
        let n_values = values.len() as u32;
        unsafe {
            ffi::clutter_keyframe_transition_set_values(
                self.as_ref().to_glib_none().0,
                n_values,
                values.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for KeyframeTransition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyframeTransition")
    }
}
