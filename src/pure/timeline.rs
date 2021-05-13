use super::{AnimationMode, StepMode, TimelineDirection, HandlerId};
use crate::prelude::*;
use crate::Point;
use std::boxed::Box as Box_;
use std::fmt;

// TODO: @implements Scriptable
#[derive(Default, Debug, Clone)]
pub struct Timeline {}

impl Timeline {
    /// Creates a new `Timeline` with a duration of `msecs`.
    /// ## `msecs`
    /// Duration of the timeline in milliseconds
    ///
    /// # Returns
    ///
    /// the newly created `Timeline` instance. Use
    ///  `gobject::ObjectExt::unref` when done using it
    pub fn new(msecs: u32) -> Timeline {
        Default::default()
    }
}

impl Object for Timeline {}
impl Is<Timeline> for Timeline {}

impl AsRef<Timeline> for Timeline {
    fn as_ref(&self) -> &Timeline {
        self
    }
}

/// Trait containing all `Timeline` methods.
///
/// # Implementors
///
/// [`Timeline`](struct.Timeline.html), [`Transition`](struct.Transition.html)
pub trait TimelineExt: 'static {
    /// Adds a named marker that will be hit when the timeline has reached
    /// the specified `progress`.
    ///
    /// Markers are unique string identifiers for a given position on the
    /// timeline. Once `self` reaches the given `progress` of its duration,
    /// if will emit a ::marker-reached signal for each marker attached to
    /// that particular point.
    ///
    /// A marker can be removed with `TimelineExt::remove_marker`. The
    /// timeline can be advanced to a marker using
    /// `TimelineExt::advance_to_marker`.
    ///
    /// See also: `TimelineExt::add_marker_at_time`
    /// ## `marker_name`
    /// the unique name for this marker
    /// ## `progress`
    /// the normalized value of the position of the martke
    fn add_marker(&self, marker_name: &str, progress: f64);

    /// Adds a named marker that will be hit when the timeline has been
    /// running for `msecs` milliseconds.
    ///
    /// Markers are unique string identifiers for a given position on the
    /// timeline. Once `self` reaches the given `msecs`, it will emit
    /// a ::marker-reached signal for each marker attached to that position.
    ///
    /// A marker can be removed with `TimelineExt::remove_marker`. The
    /// timeline can be advanced to a marker using
    /// `TimelineExt::advance_to_marker`.
    ///
    /// See also: `TimelineExt::add_marker`
    /// ## `marker_name`
    /// the unique name for this marker
    /// ## `msecs`
    /// position of the marker in milliseconds
    fn add_marker_at_time(&self, marker_name: &str, msecs: u32);

    /// Advance timeline to the requested point. The point is given as a
    /// time in milliseconds since the timeline started.
    ///
    /// The `self` will not emit the `Timeline::new-frame`
    /// signal for the given time. The first ::new-frame signal after the call to
    /// `TimelineExt::advance` will be emit the skipped markers.
    /// ## `msecs`
    /// Time to advance to
    fn advance(&self, msecs: u32);

    /// Advances `self` to the time of the given `marker_name`.
    ///
    /// Like `TimelineExt::advance`, this function will not
    /// emit the `Timeline::new-frame` for the time where `marker_name`
    /// is set, nor it will emit `Timeline::marker-reached` for
    /// `marker_name`.
    /// ## `marker_name`
    /// the name of the marker
    fn advance_to_marker(&self, marker_name: &str);

    /// Retrieves the value set by `TimelineExt::set_auto_reverse`.
    ///
    /// # Returns
    ///
    /// `true` if the timeline should automatically reverse, and
    ///  `false` otherwise
    fn get_auto_reverse(&self) -> bool;

    /// Retrieves the control points for the cubic bezier progress mode.
    /// ## `c_1`
    /// return location for the first control
    ///  point of the cubic bezier, or `None`
    /// ## `c_2`
    /// return location for the second control
    ///  point of the cubic bezier, or `None`
    ///
    /// # Returns
    ///
    /// `true` if the `self` is using a cubic bezier progress
    ///  more, and `false` otherwise
    fn get_cubic_bezier_progress(&self) -> Option<(Point<f32>, Point<f32>)>;

    /// Retrieves the current repeat for a timeline.
    ///
    /// Repeats start at 0.
    ///
    /// # Returns
    ///
    /// the current repeat
    fn get_current_repeat(&self) -> i32;

    /// Retrieves the delay set using `TimelineExt::set_delay`.
    ///
    /// # Returns
    ///
    /// the delay in milliseconds.
    fn get_delay(&self) -> u32;

    /// Retrieves the amount of time elapsed since the last
    /// Timeline::new-frame signal.
    ///
    /// This function is only useful inside handlers for the ::new-frame
    /// signal, and its behaviour is undefined if the timeline is not
    /// playing.
    ///
    /// # Returns
    ///
    /// the amount of time in milliseconds elapsed since the
    /// last frame
    fn get_delta(&self) -> u32;

    /// Retrieves the direction of the timeline set with
    /// `TimelineExt::set_direction`.
    ///
    /// # Returns
    ///
    /// the direction of the timeline
    fn get_direction(&self) -> TimelineDirection;

    /// Retrieves the duration of a `Timeline` in milliseconds.
    /// See `TimelineExt::set_duration`.
    ///
    /// # Returns
    ///
    /// the duration of the timeline, in milliseconds.
    fn get_duration(&self) -> u32;

    /// Retrieves the full duration of the `self`, taking into account the
    /// current value of the `Timeline:repeat-count` property.
    ///
    /// If the `Timeline:repeat-count` property is set to -1, this function
    /// will return `G_MAXINT64`.
    ///
    /// The returned value is to be considered a hint, and it's only valid
    /// as long as the `self` hasn't been changed.
    ///
    /// # Returns
    ///
    /// the full duration of the `Timeline`
    fn get_duration_hint(&self) -> i64;

    /// Request the current time position of the timeline.
    ///
    /// # Returns
    ///
    /// current elapsed time in milliseconds.
    fn get_elapsed_time(&self) -> u32;

    /// The position of the timeline in a normalized [-1, 2] interval.
    ///
    /// The return value of this function is determined by the progress
    /// mode set using `TimelineExt::set_progress_mode`, or by the
    /// progress function set using `TimelineExt::set_progress_func`.
    ///
    /// # Returns
    ///
    /// the normalized current position in the timeline.
    fn get_progress(&self) -> f64;

    /// Retrieves the progress mode set using `TimelineExt::set_progress_mode`
    /// or `TimelineExt::set_progress_func`.
    ///
    /// # Returns
    ///
    /// a `AnimationMode`
    fn get_progress_mode(&self) -> AnimationMode;

    /// Retrieves the number set using `TimelineExt::set_repeat_count`.
    ///
    /// # Returns
    ///
    /// the number of repeats
    fn get_repeat_count(&self) -> i32;

    /// Retrieves the parameters of the step progress mode used by `self`.
    /// ## `n_steps`
    /// return location for the number of steps, or `None`
    /// ## `step_mode`
    /// return location for the value change policy,
    ///  or `None`
    ///
    /// # Returns
    ///
    /// `true` if the `self` is using a step progress
    ///  mode, and `false` otherwise
    fn get_step_progress(&self) -> Option<(i32, StepMode)>;

    /// Checks whether `self` has a marker set with the given name.
    /// ## `marker_name`
    /// the name of the marker
    ///
    /// # Returns
    ///
    /// `true` if the marker was found
    fn has_marker(&self, marker_name: &str) -> bool;

    /// Queries state of a `Timeline`.
    ///
    /// # Returns
    ///
    /// `true` if timeline is currently playing
    fn is_playing(&self) -> bool;

    /// Retrieves the list of markers at time `msecs`. If `msecs` is a
    /// negative integer, all the markers attached to `self` will be
    /// returned.
    /// ## `msecs`
    /// the time to check, or -1
    /// ## `n_markers`
    /// the number of markers returned
    ///
    /// # Returns
    ///
    ///
    ///  a newly allocated, `None` terminated string array containing the names
    ///  of the markers. Use `g_strfreev` when done.
    fn list_markers(&self, msecs: i32) -> Vec<String>;

    /// Pauses the `Timeline` on current frame
    fn pause(&self);

    /// Removes `marker_name`, if found, from `self`.
    /// ## `marker_name`
    /// the name of the marker to remove
    fn remove_marker(&self, marker_name: &str);

    /// Rewinds `Timeline` to the first frame if its direction is
    /// `TimelineDirection::Forward` and the last frame if it is
    /// `TimelineDirection::Backward`.
    fn rewind(&self);

    /// Sets whether `self` should reverse the direction after the
    /// emission of the `Timeline::completed` signal.
    ///
    /// Setting the `Timeline:auto-reverse` property to `true` is the
    /// equivalent of connecting a callback to the `Timeline::completed`
    /// signal and changing the direction of the timeline from that callback;
    /// for instance, this code:
    ///
    ///
    /// ```text
    /// static void
    /// reverse_timeline (Timeline *timeline)
    /// {
    ///   TimelineDirection dir = timeline_get_direction (timeline);
    ///
    ///   if (dir == TIMELINE_FORWARD)
    ///     dir = TIMELINE_BACKWARD;
    ///   else
    ///     dir = TIMELINE_FORWARD;
    ///
    ///   timeline_set_direction (timeline, dir);
    /// }
    /// ...
    ///   timeline = timeline_new (1000);
    ///   timeline_set_repeat_count (timeline, -1);
    ///   g_signal_connect (timeline, "completed",
    ///                     G_CALLBACK (reverse_timeline),
    ///                     NULL);
    /// ```
    ///
    /// can be effectively replaced by:
    ///
    ///
    /// ```text
    ///   timeline = timeline_new (1000);
    ///   timeline_set_repeat_count (timeline, -1);
    ///   timeline_set_auto_reverse (timeline);
    /// ```
    /// ## `reverse`
    /// `true` if the `self` should reverse the direction
    fn set_auto_reverse(&self, reverse: bool);

    /// Sets the `Timeline:progress-mode` of `self`
    /// to `AnimationMode::CubicBezier`, and sets the two control
    /// points for the cubic bezier.
    ///
    /// The cubic bezier curve is between (0, 0) and (1, 1). The X coordinate
    /// of the two control points must be in the [ 0, 1 ] range, while the
    /// Y coordinate of the two control points can exceed this range.
    /// ## `c_1`
    /// the first control point for the cubic bezier
    /// ## `c_2`
    /// the second control point for the cubic bezier
    fn set_cubic_bezier_progress(&self, c_1: &Point<f32>, c_2: &Point<f32>);

    /// Sets the delay, in milliseconds, before `self` should start.
    /// ## `msecs`
    /// delay in milliseconds
    fn set_delay(&self, msecs: u32);

    /// Sets the direction of `self`, either `TimelineDirection::Forward` or
    /// `TimelineDirection::Backward`.
    /// ## `direction`
    /// the direction of the timeline
    fn set_direction(&self, direction: TimelineDirection);

    /// Sets the duration of the timeline, in milliseconds. The speed
    /// of the timeline depends on the Timeline:fps setting.
    /// ## `msecs`
    /// duration of the timeline in milliseconds
    fn set_duration(&self, msecs: u32);

    /// Sets a custom progress function for `self`. The progress function will
    /// be called by `TimelineExt::get_progress` and will be used to compute
    /// the progress value based on the elapsed time and the total duration of the
    /// timeline.
    ///
    /// If `func` is not `None`, the `Timeline:progress-mode` property will
    /// be set to `AnimationMode::CustomMode`.
    ///
    /// If `func` is `None`, any previously set progress function will be unset, and
    /// the `Timeline:progress-mode` property will be set to `AnimationMode::Linear`.
    /// ## `func`
    /// a progress function, or `None`
    /// ## `data`
    /// data to pass to `func`
    /// ## `notify`
    /// a function to be called when the progress function is removed
    ///  or the timeline is disposed
    fn set_progress_func(&self, func: Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>>);

    /// Sets the progress function using a value from the `AnimationMode`
    /// enumeration. The `mode` cannot be `AnimationMode::CustomMode` or bigger than
    /// `AnimationMode::AnimationLast`.
    /// ## `mode`
    /// the progress mode, as a `AnimationMode`
    fn set_progress_mode(&self, mode: AnimationMode);

    /// Sets the number of times the `self` should repeat.
    ///
    /// If `count` is 0, the timeline never repeats.
    ///
    /// If `count` is -1, the timeline will always repeat until
    /// it's stopped.
    /// ## `count`
    /// the number of times the timeline should repeat
    fn set_repeat_count(&self, count: i32);

    /// Sets the `Timeline:progress-mode` of the `self` to `AnimationMode::Steps`
    /// and provides the parameters of the step function.
    /// ## `n_steps`
    /// the number of steps
    /// ## `step_mode`
    /// whether the change should happen at the start
    ///  or at the end of the step
    fn set_step_progress(&self, n_steps: i32, step_mode: StepMode);

    /// Advance timeline by the requested time in milliseconds
    /// ## `msecs`
    /// Amount of time to skip
    fn skip(&self, msecs: u32);

    /// Starts the `Timeline` playing.
    fn start(&self);

    /// Stops the `Timeline` and moves to frame 0
    fn stop(&self);

    /// The `Timeline::completed` signal is emitted when the timeline's
    /// elapsed time reaches the value of the `Timeline:duration`
    /// property.
    ///
    /// This signal will be emitted even if the `Timeline` is set to be
    /// repeating.
    ///
    /// If you want to get notification on whether the `Timeline` has
    /// been stopped or has finished its run, including its eventual repeats,
    /// you should use the `Timeline::stopped` signal instead.
    fn connect_completed<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::marker-reached signal is emitted each time a timeline
    /// reaches a marker set with
    /// `TimelineExt::add_marker_at_time`. This signal is detailed
    /// with the name of the marker as well, so it is possible to connect
    /// a callback to the ::marker-reached signal for a specific marker
    /// with:
    ///
    /// `<informalexample>``<programlisting>`
    ///  timeline_add_marker_at_time (timeline, "foo", 500);
    ///  timeline_add_marker_at_time (timeline, "bar", 750);
    ///
    ///  g_signal_connect (timeline, "marker-reached",
    ///  G_CALLBACK (each_marker_reached), NULL);
    ///  g_signal_connect (timeline, "marker-reached::foo",
    ///  G_CALLBACK (foo_marker_reached), NULL);
    ///  g_signal_connect (timeline, "marker-reached::bar",
    ///  G_CALLBACK (bar_marker_reached), NULL);
    /// `</programlisting>``</informalexample>`
    ///
    /// In the example, the first callback will be invoked for both
    /// the "foo" and "bar" marker, while the second and third callbacks
    /// will be invoked for the "foo" or "bar" markers, respectively.
    /// ## `marker_name`
    /// the name of the marker reached
    /// ## `msecs`
    /// the elapsed time
    fn connect_marker_reached<F: Fn(&Self, &str, i32) + 'static>(&self, f: F) -> HandlerId;

    /// The ::new-frame signal is emitted for each timeline running
    /// timeline before a new frame is drawn to give animations a chance
    /// to update the scene.
    /// ## `msecs`
    /// the elapsed time between 0 and duration
    fn connect_new_frame<F: Fn(&Self, i32) + 'static>(&self, f: F) -> HandlerId;

    /// The ::paused signal is emitted when `TimelineExt::pause` is invoked.
    fn connect_paused<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The ::started signal is emitted when the timeline starts its run.
    /// This might be as soon as `TimelineExt::start` is invoked or
    /// after the delay set in the Timeline:delay property has
    /// expired.
    fn connect_started<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    /// The `Timeline::stopped` signal is emitted when the timeline
    /// has been stopped, either because `TimelineExt::stop` has been
    /// called, or because it has been exhausted.
    ///
    /// This is different from the `Timeline::completed` signal,
    /// which gets emitted after every repeat finishes.
    ///
    /// If the `Timeline` has is marked as infinitely repeating,
    /// this signal will never be emitted.
    /// ## `is_finished`
    /// `true` if the signal was emitted at the end of the
    ///  timeline.
    fn connect_stopped<F: Fn(&Self, bool) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_auto_reverse_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> HandlerId;

    fn connect_property_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_progress_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId;

    fn connect_property_repeat_count_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> HandlerId;
}

impl<O: Is<Timeline>> TimelineExt for O {
    fn add_marker(&self, marker_name: &str, progress: f64) {
        // unsafe {
        //     ffi::clutter_timeline_add_marker(
        //         self.as_ref().to_glib_none().0,
        //         marker_name.to_glib_none().0,
        //         progress,
        //     );
        // }
        unimplemented!()
    }

    fn add_marker_at_time(&self, marker_name: &str, msecs: u32) {
        // unsafe {
        //     ffi::clutter_timeline_add_marker_at_time(
        //         self.as_ref().to_glib_none().0,
        //         marker_name.to_glib_none().0,
        //         msecs,
        //     );
        // }
        unimplemented!()
    }

    fn advance(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_timeline_advance(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    fn advance_to_marker(&self, marker_name: &str) {
        // unsafe {
        //     ffi::clutter_timeline_advance_to_marker(
        //         self.as_ref().to_glib_none().0,
        //         marker_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_auto_reverse(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_timeline_get_auto_reverse(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_cubic_bezier_progress(&self) -> Option<(Point<f32>, Point<f32>)> {
        // unsafe {
        //     let mut c_1 = Point::uninitialized();
        //     let mut c_2 = Point::uninitialized();
        //     let ret = from_glib(ffi::clutter_timeline_get_cubic_bezier_progress(
        //         self.as_ref().to_glib_none().0,
        //         c_1.to_glib_none_mut().0,
        //         c_2.to_glib_none_mut().0,
        //     ));
        //     if ret {
        //         Some((c_1, c_2))
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }

    fn get_current_repeat(&self) -> i32 {
        // unsafe { ffi::clutter_timeline_get_current_repeat(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_delay(&self) -> u32 {
        // unsafe { ffi::clutter_timeline_get_delay(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_delta(&self) -> u32 {
        // unsafe { ffi::clutter_timeline_get_delta(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_direction(&self) -> TimelineDirection {
        // unsafe {
        //     from_glib(ffi::clutter_timeline_get_direction(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_duration(&self) -> u32 {
        // unsafe { ffi::clutter_timeline_get_duration(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_duration_hint(&self) -> i64 {
        // unsafe { ffi::clutter_timeline_get_duration_hint(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_elapsed_time(&self) -> u32 {
        // unsafe { ffi::clutter_timeline_get_elapsed_time(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_progress(&self) -> f64 {
        // unsafe { ffi::clutter_timeline_get_progress(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_progress_mode(&self) -> AnimationMode {
        // unsafe {
        //     from_glib(ffi::clutter_timeline_get_progress_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_repeat_count(&self) -> i32 {
        // unsafe { ffi::clutter_timeline_get_repeat_count(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_step_progress(&self) -> Option<(i32, StepMode)> {
        // unsafe {
        //     let mut n_steps = mem::MaybeUninit::uninit();
        //     let mut step_mode = mem::MaybeUninit::uninit();
        //     let ret = from_glib(ffi::clutter_timeline_get_step_progress(
        //         self.as_ref().to_glib_none().0,
        //         n_steps.as_mut_ptr(),
        //         step_mode.as_mut_ptr(),
        //     ));
        //     let n_steps = n_steps.assume_init();
        //     let step_mode = step_mode.assume_init();
        //     if ret {
        //         Some((n_steps, from_glib(step_mode)))
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }

    fn has_marker(&self, marker_name: &str) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_timeline_has_marker(
        //         self.as_ref().to_glib_none().0,
        //         marker_name.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn is_playing(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_timeline_is_playing(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn list_markers(&self, msecs: i32) -> Vec<String> {
        // unsafe {
        //     let mut n_markers = mem::MaybeUninit::uninit();
        //     let ret = FromGlibContainer::from_glib_full_num(
        //         ffi::clutter_timeline_list_markers(
        //             self.as_ref().to_glib_none().0,
        //             msecs,
        //             n_markers.as_mut_ptr(),
        //         ),
        //         n_markers.assume_init() as usize,
        //     );
        //     ret
        // }
        unimplemented!()
    }

    fn pause(&self) {
        // unsafe {
        //     ffi::clutter_timeline_pause(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_marker(&self, marker_name: &str) {
        // unsafe {
        //     ffi::clutter_timeline_remove_marker(
        //         self.as_ref().to_glib_none().0,
        //         marker_name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn rewind(&self) {
        // unsafe {
        //     ffi::clutter_timeline_rewind(self.as_ref().to_glib_none().0);
        // }
    }

    fn set_auto_reverse(&self, reverse: bool) {
        // unsafe {
        //     ffi::clutter_timeline_set_auto_reverse(
        //         self.as_ref().to_glib_none().0,
        //         reverse.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_cubic_bezier_progress(&self, c_1: &Point<f32>, c_2: &Point<f32>) {
        // unsafe {
        //     ffi::clutter_timeline_set_cubic_bezier_progress(
        //         self.as_ref().to_glib_none().0,
        //         c_1.to_glib_none().0,
        //         c_2.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_delay(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_timeline_set_delay(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    fn set_direction(&self, direction: TimelineDirection) {
        // unsafe {
        //     ffi::clutter_timeline_set_direction(
        //         self.as_ref().to_glib_none().0,
        //         direction.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_duration(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_timeline_set_duration(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    fn set_progress_func(&self, func: Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>>) {
        // let func_data: Box_<Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>>> =
        //     Box_::new(func);
        // unsafe extern "C" fn func_func(
        //     timeline: *mut ffi::Timeline,
        //     elapsed: libc::c_double,
        //     total: libc::c_double,
        //     user_data: glib_sys::gpointer,
        // ) -> libc::c_double {
        //     let timeline = from_glib_borrow(timeline);
        //     let callback: &Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>> =
        //         &*(user_data as *mut _);
        //     let res = if let Some(ref callback) = *callback {
        //         callback(&timeline, elapsed, total)
        //     } else {
        //         panic!("cannot get closure...")
        //     };
        //     res
        // }
        // let func = if func_data.is_some() {
        //     Some(func_func as _)
        // } else {
        //     None
        // };
        // unsafe extern "C" fn notify_func(data: glib_sys::gpointer) {
        //     let _callback: Box_<Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>>> =
        //         Box_::from_raw(data as *mut _);
        // }
        // let destroy_call3 = Some(notify_func as _);
        // let super_callback0: Box_<Option<Box_<dyn Fn(&Timeline, f64, f64) -> f64 + 'static>>> =
        //     func_data;
        // unsafe {
        //     ffi::clutter_timeline_set_progress_func(
        //         self.as_ref().to_glib_none().0,
        //         func,
        //         Box_::into_raw(super_callback0) as *mut _,
        //         destroy_call3,
        //     );
        // }
        unimplemented!()
    }

    fn set_progress_mode(&self, mode: AnimationMode) {
        // unsafe {
        //     ffi::clutter_timeline_set_progress_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        // }
        unimplemented!()
    }

    fn set_repeat_count(&self, count: i32) {
        // unsafe {
        //     ffi::clutter_timeline_set_repeat_count(self.as_ref().to_glib_none().0, count);
        // }
        unimplemented!()
    }

    fn set_step_progress(&self, n_steps: i32, step_mode: StepMode) {
        // unsafe {
        //     ffi::clutter_timeline_set_step_progress(
        //         self.as_ref().to_glib_none().0,
        //         n_steps,
        //         step_mode.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn skip(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_timeline_skip(self.as_ref().to_glib_none().0, msecs);
        // }
    }

    fn start(&self) {
        // unsafe {
        //     ffi::clutter_timeline_start(self.as_ref().to_glib_none().0);
        // }
    }

    fn stop(&self) {
        // unsafe {
        //     ffi::clutter_timeline_stop(self.as_ref().to_glib_none().0);
        // }
    }

    fn connect_completed<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_marker_reached<F: Fn(&Self, &str, i32) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_new_frame<F: Fn(&Self, i32) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_paused<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_started<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_stopped<F: Fn(&Self, bool) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_auto_reverse_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_delay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_duration_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_progress_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_repeat_count_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timeline")
    }
}
