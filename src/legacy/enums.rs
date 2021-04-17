use glib::{
    error::ErrorDomain,
    translate::*,
    value::{FromValue, FromValueOptional, SetValue, Value},
    Quark, StaticType, Type,
};
use std::fmt;

/// Controls how a `Actor` should align itself inside the extra space
/// assigned to it during the allocation.
///
/// Alignment only matters if the allocated space given to an actor is
/// bigger than its natural size; for example, when the `Actor:x-expand`
/// or the `Actor:y-expand` properties of `Actor` are set to `true`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ActorAlign {
    /// Stretch to cover the whole allocated space
    Fill,
    /// Snap to left or top side, leaving space
    ///  to the right or bottom. For horizontal layouts, in right-to-left
    ///  locales this should be reversed.
    Start,
    /// Center the actor inside the allocation
    Center,
    /// Snap to right or bottom side, leaving space
    ///  to the left or top. For horizontal layouts, in right-to-left locales
    ///  this should be reversed.
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ActorAlign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ActorAlign::{}",
            match *self {
                ActorAlign::Fill => "Fill",
                ActorAlign::Start => "Start",
                ActorAlign::Center => "Center",
                ActorAlign::End => "End",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ActorAlign {
    type GlibType = ffi::ClutterActorAlign;

    fn to_glib(&self) -> ffi::ClutterActorAlign {
        match *self {
            ActorAlign::Fill => ffi::CLUTTER_ACTOR_ALIGN_FILL,
            ActorAlign::Start => ffi::CLUTTER_ACTOR_ALIGN_START,
            ActorAlign::Center => ffi::CLUTTER_ACTOR_ALIGN_CENTER,
            ActorAlign::End => ffi::CLUTTER_ACTOR_ALIGN_END,
            ActorAlign::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterActorAlign> for ActorAlign {
    fn from_glib(value: ffi::ClutterActorAlign) -> Self {
        match value {
            0 => ActorAlign::Fill,
            1 => ActorAlign::Start,
            2 => ActorAlign::Center,
            3 => ActorAlign::End,
            value => ActorAlign::__Unknown(value),
        }
    }
}

impl StaticType for ActorAlign {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_actor_align_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ActorAlign {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ActorAlign {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ActorAlign {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Specifies the axis on which `AlignConstraint` should maintain
/// the alignment.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum AlignAxis {
    /// Maintain the alignment on the X axis
    XAxis,
    /// Maintain the alignment on the Y axis
    YAxis,
    /// Maintain the alignment on both the X and Y axis
    Both,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AlignAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AlignAxis::{}",
            match *self {
                AlignAxis::XAxis => "XAxis",
                AlignAxis::YAxis => "YAxis",
                AlignAxis::Both => "Both",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for AlignAxis {
    type GlibType = ffi::ClutterAlignAxis;

    fn to_glib(&self) -> ffi::ClutterAlignAxis {
        match *self {
            AlignAxis::XAxis => ffi::CLUTTER_ALIGN_X_AXIS,
            AlignAxis::YAxis => ffi::CLUTTER_ALIGN_Y_AXIS,
            AlignAxis::Both => ffi::CLUTTER_ALIGN_BOTH,
            AlignAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterAlignAxis> for AlignAxis {
    fn from_glib(value: ffi::ClutterAlignAxis) -> Self {
        match value {
            0 => AlignAxis::XAxis,
            1 => AlignAxis::YAxis,
            2 => AlignAxis::Both,
            value => AlignAxis::__Unknown(value),
        }
    }
}

impl StaticType for AlignAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_align_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AlignAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AlignAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AlignAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The animation modes used by `Alpha` and `Animation`. This
/// enumeration can be expanded in later versions of Clutter.
///
/// <figure id="easing-modes">
///  `<title>`Easing modes provided by Clutter`</title>`
///  <graphic fileref="easing-modes.png" format="PNG"/>
/// `</figure>`
///
/// Every global alpha function registered using `Alpha::register_func`
/// or `Alpha::register_closure` will have a logical id greater than
/// `AnimationMode::AnimationLast`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum AnimationMode {
    /// custom progress function
    CustomMode,
    /// linear tweening
    Linear,
    /// quadratic tweening
    EaseInQuad,
    /// quadratic tweening, inverse of
    ///  `AnimationMode::EaseInQuad`
    EaseOutQuad,
    /// quadratic tweening, combininig
    ///  `AnimationMode::EaseInQuad` and `AnimationMode::EaseOutQuad`
    EaseInOutQuad,
    /// cubic tweening
    EaseInCubic,
    /// cubic tweening, invers of
    ///  `AnimationMode::EaseInCubic`
    EaseOutCubic,
    /// cubic tweening, combining
    ///  `AnimationMode::EaseInCubic` and `AnimationMode::EaseOutCubic`
    EaseInOutCubic,
    /// quartic tweening
    EaseInQuart,
    /// quartic tweening, inverse of
    ///  `AnimationMode::EaseInQuart`
    EaseOutQuart,
    /// quartic tweening, combining
    ///  `AnimationMode::EaseInQuart` and `AnimationMode::EaseOutQuart`
    EaseInOutQuart,
    /// quintic tweening
    EaseInQuint,
    /// quintic tweening, inverse of
    ///  `AnimationMode::EaseInQuint`
    EaseOutQuint,
    /// fifth power tweening, combining
    ///  `AnimationMode::EaseInQuint` and `AnimationMode::EaseOutQuint`
    EaseInOutQuint,
    /// sinusoidal tweening
    EaseInSine,
    /// sinusoidal tweening, inverse of
    ///  `AnimationMode::EaseInSine`
    EaseOutSine,
    /// sine wave tweening, combining
    ///  `AnimationMode::EaseInSine` and `AnimationMode::EaseOutSine`
    EaseInOutSine,
    /// exponential tweening
    EaseInExpo,
    /// exponential tweening, inverse of
    ///  `AnimationMode::EaseInExpo`
    EaseOutExpo,
    /// exponential tweening, combining
    ///  `AnimationMode::EaseInExpo` and `AnimationMode::EaseOutExpo`
    EaseInOutExpo,
    /// circular tweening
    EaseInCirc,
    /// circular tweening, inverse of
    ///  `AnimationMode::EaseInCirc`
    EaseOutCirc,
    /// circular tweening, combining
    ///  `AnimationMode::EaseInCirc` and `AnimationMode::EaseOutCirc`
    EaseInOutCirc,
    /// elastic tweening, with offshoot on start
    EaseInElastic,
    /// elastic tweening, with offshoot on end
    EaseOutElastic,
    /// elastic tweening with offshoot on both ends
    EaseInOutElastic,
    /// overshooting cubic tweening, with
    ///  backtracking on start
    EaseInBack,
    /// overshooting cubic tweening, with
    ///  backtracking on end
    EaseOutBack,
    /// overshooting cubic tweening, with
    ///  backtracking on both ends
    EaseInOutBack,
    /// exponentially decaying parabolic (bounce)
    ///  tweening, with bounce on start
    EaseInBounce,
    /// exponentially decaying parabolic (bounce)
    ///  tweening, with bounce on end
    EaseOutBounce,
    /// exponentially decaying parabolic (bounce)
    ///  tweening, with bounce on both ends
    EaseInOutBounce,
    /// parametrized step function; see `TimelineExt::set_step_progress`
    ///  for further details. (Since 1.12)
    Steps,
    /// equivalent to `AnimationMode::Steps` with a number of steps
    ///  equal to 1, and a step mode of `StepMode::Start`. (Since 1.12)
    StepStart,
    /// equivalent to `AnimationMode::Steps` with a number of steps
    ///  equal to 1, and a step mode of `StepMode::End`. (Since 1.12)
    StepEnd,
    /// cubic bezier between (0, 0) and (1, 1) with two
    ///  control points; see `TimelineExt::set_cubic_bezier_progress`. (Since 1.12)
    CubicBezier,
    /// equivalent to `AnimationMode::CubicBezier` with control points
    ///  in (0.25, 0.1) and (0.25, 1.0). (Since 1.12)
    Ease,
    /// equivalent to `AnimationMode::CubicBezier` with control points
    ///  in (0.42, 0) and (1.0, 1.0). (Since 1.12)
    EaseIn,
    /// equivalent to `AnimationMode::CubicBezier` with control points
    ///  in (0, 0) and (0.58, 1.0). (Since 1.12)
    EaseOut,
    /// equivalent to `AnimationMode::CubicBezier` with control points
    ///  in (0.42, 0) and (0.58, 1.0). (Since 1.12)
    EaseInOut,
    /// last animation mode, used as a guard for
    ///  registered global alpha functions
    AnimationLast,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AnimationMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AnimationMode::{}",
            match *self {
                AnimationMode::CustomMode => "CustomMode",
                AnimationMode::Linear => "Linear",
                AnimationMode::EaseInQuad => "EaseInQuad",
                AnimationMode::EaseOutQuad => "EaseOutQuad",
                AnimationMode::EaseInOutQuad => "EaseInOutQuad",
                AnimationMode::EaseInCubic => "EaseInCubic",
                AnimationMode::EaseOutCubic => "EaseOutCubic",
                AnimationMode::EaseInOutCubic => "EaseInOutCubic",
                AnimationMode::EaseInQuart => "EaseInQuart",
                AnimationMode::EaseOutQuart => "EaseOutQuart",
                AnimationMode::EaseInOutQuart => "EaseInOutQuart",
                AnimationMode::EaseInQuint => "EaseInQuint",
                AnimationMode::EaseOutQuint => "EaseOutQuint",
                AnimationMode::EaseInOutQuint => "EaseInOutQuint",
                AnimationMode::EaseInSine => "EaseInSine",
                AnimationMode::EaseOutSine => "EaseOutSine",
                AnimationMode::EaseInOutSine => "EaseInOutSine",
                AnimationMode::EaseInExpo => "EaseInExpo",
                AnimationMode::EaseOutExpo => "EaseOutExpo",
                AnimationMode::EaseInOutExpo => "EaseInOutExpo",
                AnimationMode::EaseInCirc => "EaseInCirc",
                AnimationMode::EaseOutCirc => "EaseOutCirc",
                AnimationMode::EaseInOutCirc => "EaseInOutCirc",
                AnimationMode::EaseInElastic => "EaseInElastic",
                AnimationMode::EaseOutElastic => "EaseOutElastic",
                AnimationMode::EaseInOutElastic => "EaseInOutElastic",
                AnimationMode::EaseInBack => "EaseInBack",
                AnimationMode::EaseOutBack => "EaseOutBack",
                AnimationMode::EaseInOutBack => "EaseInOutBack",
                AnimationMode::EaseInBounce => "EaseInBounce",
                AnimationMode::EaseOutBounce => "EaseOutBounce",
                AnimationMode::EaseInOutBounce => "EaseInOutBounce",
                AnimationMode::Steps => "Steps",
                AnimationMode::StepStart => "StepStart",
                AnimationMode::StepEnd => "StepEnd",
                AnimationMode::CubicBezier => "CubicBezier",
                AnimationMode::Ease => "Ease",
                AnimationMode::EaseIn => "EaseIn",
                AnimationMode::EaseOut => "EaseOut",
                AnimationMode::EaseInOut => "EaseInOut",
                AnimationMode::AnimationLast => "AnimationLast",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for AnimationMode {
    type GlibType = ffi::ClutterAnimationMode;

    fn to_glib(&self) -> ffi::ClutterAnimationMode {
        match *self {
            AnimationMode::CustomMode => ffi::CLUTTER_CUSTOM_MODE,
            AnimationMode::Linear => ffi::CLUTTER_LINEAR,
            AnimationMode::EaseInQuad => ffi::CLUTTER_EASE_IN_QUAD,
            AnimationMode::EaseOutQuad => ffi::CLUTTER_EASE_OUT_QUAD,
            AnimationMode::EaseInOutQuad => ffi::CLUTTER_EASE_IN_OUT_QUAD,
            AnimationMode::EaseInCubic => ffi::CLUTTER_EASE_IN_CUBIC,
            AnimationMode::EaseOutCubic => ffi::CLUTTER_EASE_OUT_CUBIC,
            AnimationMode::EaseInOutCubic => ffi::CLUTTER_EASE_IN_OUT_CUBIC,
            AnimationMode::EaseInQuart => ffi::CLUTTER_EASE_IN_QUART,
            AnimationMode::EaseOutQuart => ffi::CLUTTER_EASE_OUT_QUART,
            AnimationMode::EaseInOutQuart => ffi::CLUTTER_EASE_IN_OUT_QUART,
            AnimationMode::EaseInQuint => ffi::CLUTTER_EASE_IN_QUINT,
            AnimationMode::EaseOutQuint => ffi::CLUTTER_EASE_OUT_QUINT,
            AnimationMode::EaseInOutQuint => ffi::CLUTTER_EASE_IN_OUT_QUINT,
            AnimationMode::EaseInSine => ffi::CLUTTER_EASE_IN_SINE,
            AnimationMode::EaseOutSine => ffi::CLUTTER_EASE_OUT_SINE,
            AnimationMode::EaseInOutSine => ffi::CLUTTER_EASE_IN_OUT_SINE,
            AnimationMode::EaseInExpo => ffi::CLUTTER_EASE_IN_EXPO,
            AnimationMode::EaseOutExpo => ffi::CLUTTER_EASE_OUT_EXPO,
            AnimationMode::EaseInOutExpo => ffi::CLUTTER_EASE_IN_OUT_EXPO,
            AnimationMode::EaseInCirc => ffi::CLUTTER_EASE_IN_CIRC,
            AnimationMode::EaseOutCirc => ffi::CLUTTER_EASE_OUT_CIRC,
            AnimationMode::EaseInOutCirc => ffi::CLUTTER_EASE_IN_OUT_CIRC,
            AnimationMode::EaseInElastic => ffi::CLUTTER_EASE_IN_ELASTIC,
            AnimationMode::EaseOutElastic => ffi::CLUTTER_EASE_OUT_ELASTIC,
            AnimationMode::EaseInOutElastic => ffi::CLUTTER_EASE_IN_OUT_ELASTIC,
            AnimationMode::EaseInBack => ffi::CLUTTER_EASE_IN_BACK,
            AnimationMode::EaseOutBack => ffi::CLUTTER_EASE_OUT_BACK,
            AnimationMode::EaseInOutBack => ffi::CLUTTER_EASE_IN_OUT_BACK,
            AnimationMode::EaseInBounce => ffi::CLUTTER_EASE_IN_BOUNCE,
            AnimationMode::EaseOutBounce => ffi::CLUTTER_EASE_OUT_BOUNCE,
            AnimationMode::EaseInOutBounce => ffi::CLUTTER_EASE_IN_OUT_BOUNCE,
            AnimationMode::Steps => ffi::CLUTTER_STEPS,
            AnimationMode::StepStart => ffi::CLUTTER_STEP_START,
            AnimationMode::StepEnd => ffi::CLUTTER_STEP_END,
            AnimationMode::CubicBezier => ffi::CLUTTER_CUBIC_BEZIER,
            AnimationMode::Ease => ffi::CLUTTER_EASE,
            AnimationMode::EaseIn => ffi::CLUTTER_EASE_IN,
            AnimationMode::EaseOut => ffi::CLUTTER_EASE_OUT,
            AnimationMode::EaseInOut => ffi::CLUTTER_EASE_IN_OUT,
            AnimationMode::AnimationLast => ffi::CLUTTER_ANIMATION_LAST,
            AnimationMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterAnimationMode> for AnimationMode {
    fn from_glib(value: ffi::ClutterAnimationMode) -> Self {
        match value {
            0 => AnimationMode::CustomMode,
            1 => AnimationMode::Linear,
            2 => AnimationMode::EaseInQuad,
            3 => AnimationMode::EaseOutQuad,
            4 => AnimationMode::EaseInOutQuad,
            5 => AnimationMode::EaseInCubic,
            6 => AnimationMode::EaseOutCubic,
            7 => AnimationMode::EaseInOutCubic,
            8 => AnimationMode::EaseInQuart,
            9 => AnimationMode::EaseOutQuart,
            10 => AnimationMode::EaseInOutQuart,
            11 => AnimationMode::EaseInQuint,
            12 => AnimationMode::EaseOutQuint,
            13 => AnimationMode::EaseInOutQuint,
            14 => AnimationMode::EaseInSine,
            15 => AnimationMode::EaseOutSine,
            16 => AnimationMode::EaseInOutSine,
            17 => AnimationMode::EaseInExpo,
            18 => AnimationMode::EaseOutExpo,
            19 => AnimationMode::EaseInOutExpo,
            20 => AnimationMode::EaseInCirc,
            21 => AnimationMode::EaseOutCirc,
            22 => AnimationMode::EaseInOutCirc,
            23 => AnimationMode::EaseInElastic,
            24 => AnimationMode::EaseOutElastic,
            25 => AnimationMode::EaseInOutElastic,
            26 => AnimationMode::EaseInBack,
            27 => AnimationMode::EaseOutBack,
            28 => AnimationMode::EaseInOutBack,
            29 => AnimationMode::EaseInBounce,
            30 => AnimationMode::EaseOutBounce,
            31 => AnimationMode::EaseInOutBounce,
            32 => AnimationMode::Steps,
            33 => AnimationMode::StepStart,
            34 => AnimationMode::StepEnd,
            35 => AnimationMode::CubicBezier,
            36 => AnimationMode::Ease,
            37 => AnimationMode::EaseIn,
            38 => AnimationMode::EaseOut,
            39 => AnimationMode::EaseInOut,
            40 => AnimationMode::AnimationLast,
            value => AnimationMode::__Unknown(value),
        }
    }
}

impl StaticType for AnimationMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_animation_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AnimationMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AnimationMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for AnimationMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BinAlignment {
    Fixed,
    Fill,
    Start,
    End,
    Center,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BinAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BinAlignment::{}",
            match *self {
                BinAlignment::Fixed => "Fixed",
                BinAlignment::Fill => "Fill",
                BinAlignment::Start => "Start",
                BinAlignment::End => "End",
                BinAlignment::Center => "Center",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BinAlignment {
    type GlibType = ffi::ClutterBinAlignment;

    fn to_glib(&self) -> ffi::ClutterBinAlignment {
        match *self {
            BinAlignment::Fixed => ffi::CLUTTER_BIN_ALIGNMENT_FIXED,
            BinAlignment::Fill => ffi::CLUTTER_BIN_ALIGNMENT_FILL,
            BinAlignment::Start => ffi::CLUTTER_BIN_ALIGNMENT_START,
            BinAlignment::End => ffi::CLUTTER_BIN_ALIGNMENT_END,
            BinAlignment::Center => ffi::CLUTTER_BIN_ALIGNMENT_CENTER,
            BinAlignment::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterBinAlignment> for BinAlignment {
    fn from_glib(value: ffi::ClutterBinAlignment) -> Self {
        match value {
            0 => BinAlignment::Fixed,
            1 => BinAlignment::Fill,
            2 => BinAlignment::Start,
            3 => BinAlignment::End,
            4 => BinAlignment::Center,
            value => BinAlignment::__Unknown(value),
        }
    }
}

impl StaticType for BinAlignment {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_bin_alignment_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BinAlignment {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BinAlignment {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BinAlignment {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Specifies which property should be used in a binding
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BindCoordinate {
    /// Bind the X coordinate
    X,
    /// Bind the Y coordinate
    Y,
    /// Bind the width
    Width,
    /// Bind the height
    Height,
    /// Equivalent to to `BindCoordinate::X` and
    ///  `BindCoordinate::Y` (added in Clutter 1.6)
    Position,
    /// Equivalent to `BindCoordinate::Width` and
    ///  `BindCoordinate::Height` (added in Clutter 1.6)
    Size,
    /// Equivalent to `BindCoordinate::Position` and
    ///  `BindCoordinate::Size` (added in Clutter 1.10)
    All,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BindCoordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BindCoordinate::{}",
            match *self {
                BindCoordinate::X => "X",
                BindCoordinate::Y => "Y",
                BindCoordinate::Width => "Width",
                BindCoordinate::Height => "Height",
                BindCoordinate::Position => "Position",
                BindCoordinate::Size => "Size",
                BindCoordinate::All => "All",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BindCoordinate {
    type GlibType = ffi::ClutterBindCoordinate;

    fn to_glib(&self) -> ffi::ClutterBindCoordinate {
        match *self {
            BindCoordinate::X => ffi::CLUTTER_BIND_X,
            BindCoordinate::Y => ffi::CLUTTER_BIND_Y,
            BindCoordinate::Width => ffi::CLUTTER_BIND_WIDTH,
            BindCoordinate::Height => ffi::CLUTTER_BIND_HEIGHT,
            BindCoordinate::Position => ffi::CLUTTER_BIND_POSITION,
            BindCoordinate::Size => ffi::CLUTTER_BIND_SIZE,
            BindCoordinate::All => ffi::CLUTTER_BIND_ALL,
            BindCoordinate::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterBindCoordinate> for BindCoordinate {
    fn from_glib(value: ffi::ClutterBindCoordinate) -> Self {
        match value {
            0 => BindCoordinate::X,
            1 => BindCoordinate::Y,
            2 => BindCoordinate::Width,
            3 => BindCoordinate::Height,
            4 => BindCoordinate::Position,
            5 => BindCoordinate::Size,
            6 => BindCoordinate::All,
            value => BindCoordinate::__Unknown(value),
        }
    }
}

impl StaticType for BindCoordinate {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_bind_coordinate_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BindCoordinate {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BindCoordinate {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BindCoordinate {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The alignment policies available on each axis of the `BoxLayout`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum BoxAlignment {
    /// Align the child to the top or to
    ///  to the left, depending on the used axis
    Start,
    /// Align the child to the bottom or to
    ///  the right, depending on the used axis
    End,
    /// Align the child to the center
    Center,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BoxAlignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BoxAlignment::{}",
            match *self {
                BoxAlignment::Start => "Start",
                BoxAlignment::End => "End",
                BoxAlignment::Center => "Center",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for BoxAlignment {
    type GlibType = ffi::ClutterBoxAlignment;

    fn to_glib(&self) -> ffi::ClutterBoxAlignment {
        match *self {
            BoxAlignment::Start => ffi::CLUTTER_BOX_ALIGNMENT_START,
            BoxAlignment::End => ffi::CLUTTER_BOX_ALIGNMENT_END,
            BoxAlignment::Center => ffi::CLUTTER_BOX_ALIGNMENT_CENTER,
            BoxAlignment::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterBoxAlignment> for BoxAlignment {
    fn from_glib(value: ffi::ClutterBoxAlignment) -> Self {
        match value {
            0 => BoxAlignment::Start,
            1 => BoxAlignment::End,
            2 => BoxAlignment::Center,
            value => BoxAlignment::__Unknown(value),
        }
    }
}

impl StaticType for BoxAlignment {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_box_alignment_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for BoxAlignment {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for BoxAlignment {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for BoxAlignment {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Controls the alignment of the `Content` inside a `Actor`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ContentGravity {
    /// Align the content to the top left corner
    TopLeft,
    /// Align the content to the top edge
    Top,
    /// Align the content to the top right corner
    TopRight,
    /// Align the content to the left edge
    Left,
    /// Align the content to the center
    Center,
    /// Align the content to the right edge
    Right,
    /// Align the content to the bottom left corner
    BottomLeft,
    /// Align the content to the bottom edge
    Bottom,
    /// Align the content to the bottom right corner
    BottomRight,
    /// Resize the content to fill the allocation
    ResizeFill,
    /// Resize the content to remain within the
    ///  allocation, while maintaining the aspect ratio
    ResizeAspect,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ContentGravity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ContentGravity::{}",
            match *self {
                ContentGravity::TopLeft => "TopLeft",
                ContentGravity::Top => "Top",
                ContentGravity::TopRight => "TopRight",
                ContentGravity::Left => "Left",
                ContentGravity::Center => "Center",
                ContentGravity::Right => "Right",
                ContentGravity::BottomLeft => "BottomLeft",
                ContentGravity::Bottom => "Bottom",
                ContentGravity::BottomRight => "BottomRight",
                ContentGravity::ResizeFill => "ResizeFill",
                ContentGravity::ResizeAspect => "ResizeAspect",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ContentGravity {
    type GlibType = ffi::ClutterContentGravity;

    fn to_glib(&self) -> ffi::ClutterContentGravity {
        match *self {
            ContentGravity::TopLeft => ffi::CLUTTER_CONTENT_GRAVITY_TOP_LEFT,
            ContentGravity::Top => ffi::CLUTTER_CONTENT_GRAVITY_TOP,
            ContentGravity::TopRight => ffi::CLUTTER_CONTENT_GRAVITY_TOP_RIGHT,
            ContentGravity::Left => ffi::CLUTTER_CONTENT_GRAVITY_LEFT,
            ContentGravity::Center => ffi::CLUTTER_CONTENT_GRAVITY_CENTER,
            ContentGravity::Right => ffi::CLUTTER_CONTENT_GRAVITY_RIGHT,
            ContentGravity::BottomLeft => ffi::CLUTTER_CONTENT_GRAVITY_BOTTOM_LEFT,
            ContentGravity::Bottom => ffi::CLUTTER_CONTENT_GRAVITY_BOTTOM,
            ContentGravity::BottomRight => ffi::CLUTTER_CONTENT_GRAVITY_BOTTOM_RIGHT,
            ContentGravity::ResizeFill => ffi::CLUTTER_CONTENT_GRAVITY_RESIZE_FILL,
            ContentGravity::ResizeAspect => ffi::CLUTTER_CONTENT_GRAVITY_RESIZE_ASPECT,
            ContentGravity::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterContentGravity> for ContentGravity {
    fn from_glib(value: ffi::ClutterContentGravity) -> Self {
        match value {
            0 => ContentGravity::TopLeft,
            1 => ContentGravity::Top,
            2 => ContentGravity::TopRight,
            3 => ContentGravity::Left,
            4 => ContentGravity::Center,
            5 => ContentGravity::Right,
            6 => ContentGravity::BottomLeft,
            7 => ContentGravity::Bottom,
            8 => ContentGravity::BottomRight,
            9 => ContentGravity::ResizeFill,
            10 => ContentGravity::ResizeAspect,
            value => ContentGravity::__Unknown(value),
        }
    }
}

impl StaticType for ContentGravity {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_content_gravity_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ContentGravity {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ContentGravity {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ContentGravity {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The axis of the constraint that should be applied on the
/// dragging action
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum DragAxis {
    /// No constraint
    AxisNone,
    /// Set a constraint on the X axis
    XAxis,
    /// Set a constraint on the Y axis
    YAxis,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DragAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DragAxis::{}",
            match *self {
                DragAxis::AxisNone => "AxisNone",
                DragAxis::XAxis => "XAxis",
                DragAxis::YAxis => "YAxis",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for DragAxis {
    type GlibType = ffi::ClutterDragAxis;

    fn to_glib(&self) -> ffi::ClutterDragAxis {
        match *self {
            DragAxis::AxisNone => ffi::CLUTTER_DRAG_AXIS_NONE,
            DragAxis::XAxis => ffi::CLUTTER_DRAG_X_AXIS,
            DragAxis::YAxis => ffi::CLUTTER_DRAG_Y_AXIS,
            DragAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterDragAxis> for DragAxis {
    fn from_glib(value: ffi::ClutterDragAxis) -> Self {
        match value {
            0 => DragAxis::AxisNone,
            1 => DragAxis::XAxis,
            2 => DragAxis::YAxis,
            value => DragAxis::__Unknown(value),
        }
    }
}

impl StaticType for DragAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_drag_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DragAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DragAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DragAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Types of events.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum EventType {
    /// Empty event
    Nothing,
    /// Key press event
    KeyPress,
    /// Key release event
    KeyRelease,
    /// Pointer motion event
    Motion,
    /// Actor enter event
    Enter,
    /// Actor leave event
    Leave,
    /// Pointer button press event
    ButtonPress,
    /// Pointer button release event
    ButtonRelease,
    /// Pointer scroll event
    Scroll,
    /// Stage state change event
    StageState,
    /// Destroy notification event
    DestroyNotify,
    /// Client message event
    ClientMessage,
    /// Stage delete event
    Delete,
    /// A new touch event sequence has started;
    ///  event added in 1.10
    TouchBegin,
    /// A touch event sequence has been updated;
    ///  event added in 1.10
    TouchUpdate,
    /// A touch event sequence has finished;
    ///  event added in 1.10
    TouchEnd,
    /// A touch event sequence has been canceled;
    ///  event added in 1.10
    TouchCancel,
    /// A pinch gesture event, the current state is
    ///  determined by its phase field; event added in 1.24
    TouchpadPinch,
    /// A swipe gesture event, the current state is
    ///  determined by its phase field; event added in 1.24
    TouchpadSwipe,
    /// Marks the end of the `EventType` enumeration;
    ///  added in 1.10
    EventLast,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EventType::{}",
            match *self {
                EventType::Nothing => "Nothing",
                EventType::KeyPress => "KeyPress",
                EventType::KeyRelease => "KeyRelease",
                EventType::Motion => "Motion",
                EventType::Enter => "Enter",
                EventType::Leave => "Leave",
                EventType::ButtonPress => "ButtonPress",
                EventType::ButtonRelease => "ButtonRelease",
                EventType::Scroll => "Scroll",
                EventType::StageState => "StageState",
                EventType::DestroyNotify => "DestroyNotify",
                EventType::ClientMessage => "ClientMessage",
                EventType::Delete => "Delete",
                EventType::TouchBegin => "TouchBegin",
                EventType::TouchUpdate => "TouchUpdate",
                EventType::TouchEnd => "TouchEnd",
                EventType::TouchCancel => "TouchCancel",
                EventType::TouchpadPinch => "TouchpadPinch",
                EventType::TouchpadSwipe => "TouchpadSwipe",
                EventType::EventLast => "EventLast",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for EventType {
    type GlibType = ffi::ClutterEventType;

    fn to_glib(&self) -> ffi::ClutterEventType {
        match *self {
            EventType::Nothing => ffi::CLUTTER_NOTHING,
            EventType::KeyPress => ffi::CLUTTER_KEY_PRESS,
            EventType::KeyRelease => ffi::CLUTTER_KEY_RELEASE,
            EventType::Motion => ffi::CLUTTER_MOTION,
            EventType::Enter => ffi::CLUTTER_ENTER,
            EventType::Leave => ffi::CLUTTER_LEAVE,
            EventType::ButtonPress => ffi::CLUTTER_BUTTON_PRESS,
            EventType::ButtonRelease => ffi::CLUTTER_BUTTON_RELEASE,
            EventType::Scroll => ffi::CLUTTER_SCROLL,
            EventType::StageState => ffi::CLUTTER_STAGE_STATE,
            EventType::DestroyNotify => ffi::CLUTTER_DESTROY_NOTIFY,
            EventType::ClientMessage => ffi::CLUTTER_CLIENT_MESSAGE,
            EventType::Delete => ffi::CLUTTER_DELETE,
            EventType::TouchBegin => ffi::CLUTTER_TOUCH_BEGIN,
            EventType::TouchUpdate => ffi::CLUTTER_TOUCH_UPDATE,
            EventType::TouchEnd => ffi::CLUTTER_TOUCH_END,
            EventType::TouchCancel => ffi::CLUTTER_TOUCH_CANCEL,
            EventType::TouchpadPinch => ffi::CLUTTER_TOUCHPAD_PINCH,
            EventType::TouchpadSwipe => ffi::CLUTTER_TOUCHPAD_SWIPE,
            EventType::EventLast => ffi::CLUTTER_EVENT_LAST,
            EventType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterEventType> for EventType {
    fn from_glib(value: ffi::ClutterEventType) -> Self {
        match value {
            0 => EventType::Nothing,
            1 => EventType::KeyPress,
            2 => EventType::KeyRelease,
            3 => EventType::Motion,
            4 => EventType::Enter,
            5 => EventType::Leave,
            6 => EventType::ButtonPress,
            7 => EventType::ButtonRelease,
            8 => EventType::Scroll,
            9 => EventType::StageState,
            10 => EventType::DestroyNotify,
            11 => EventType::ClientMessage,
            12 => EventType::Delete,
            13 => EventType::TouchBegin,
            14 => EventType::TouchUpdate,
            15 => EventType::TouchEnd,
            16 => EventType::TouchCancel,
            17 => EventType::TouchpadPinch,
            18 => EventType::TouchpadSwipe,
            19 => EventType::EventLast,
            value => EventType::__Unknown(value),
        }
    }
}

impl StaticType for EventType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_event_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EventType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EventType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for EventType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The direction of the arrangement of the children inside
/// a `FlowLayout`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum FlowOrientation {
    /// Arrange the children of the flow layout
    ///  horizontally first
    Horizontal,
    /// Arrange the children of the flow layout
    ///  vertically first
    Vertical,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for FlowOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FlowOrientation::{}",
            match *self {
                FlowOrientation::Horizontal => "Horizontal",
                FlowOrientation::Vertical => "Vertical",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for FlowOrientation {
    type GlibType = ffi::ClutterFlowOrientation;

    fn to_glib(&self) -> ffi::ClutterFlowOrientation {
        match *self {
            FlowOrientation::Horizontal => ffi::CLUTTER_FLOW_HORIZONTAL,
            FlowOrientation::Vertical => ffi::CLUTTER_FLOW_VERTICAL,
            FlowOrientation::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterFlowOrientation> for FlowOrientation {
    fn from_glib(value: ffi::ClutterFlowOrientation) -> Self {
        match value {
            0 => FlowOrientation::Horizontal,
            1 => FlowOrientation::Vertical,
            value => FlowOrientation::__Unknown(value),
        }
    }
}

impl StaticType for FlowOrientation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_flow_orientation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FlowOrientation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FlowOrientation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for FlowOrientation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Enum passed to the `GestureActionExt::set_threshold_trigger_edge`
/// function.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GestureTriggerEdge {
    /// Tell `GestureAction` that
    /// the gesture must begin immediately and there's no drag limit that
    /// will cause its cancellation;
    None,
    /// Tell `GestureAction` that
    /// it needs to wait until the drag threshold has been exceeded before
    /// considering that the gesture has begun;
    After,
    /// Tell `GestureAction` that
    /// the gesture must begin immediately and that it must be cancelled
    /// once the drag exceed the configured threshold.
    Before,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GestureTriggerEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GestureTriggerEdge::{}",
            match *self {
                GestureTriggerEdge::None => "None",
                GestureTriggerEdge::After => "After",
                GestureTriggerEdge::Before => "Before",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for GestureTriggerEdge {
    type GlibType = ffi::ClutterGestureTriggerEdge;

    fn to_glib(&self) -> ffi::ClutterGestureTriggerEdge {
        match *self {
            GestureTriggerEdge::None => ffi::CLUTTER_GESTURE_TRIGGER_EDGE_NONE,
            GestureTriggerEdge::After => ffi::CLUTTER_GESTURE_TRIGGER_EDGE_AFTER,
            GestureTriggerEdge::Before => ffi::CLUTTER_GESTURE_TRIGGER_EDGE_BEFORE,
            GestureTriggerEdge::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterGestureTriggerEdge> for GestureTriggerEdge {
    fn from_glib(value: ffi::ClutterGestureTriggerEdge) -> Self {
        match value {
            0 => GestureTriggerEdge::None,
            1 => GestureTriggerEdge::After,
            2 => GestureTriggerEdge::Before,
            value => GestureTriggerEdge::__Unknown(value),
        }
    }
}

impl StaticType for GestureTriggerEdge {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_gesture_trigger_edge_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GestureTriggerEdge {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GestureTriggerEdge {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GestureTriggerEdge {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Grid position modes.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum GridPosition {
    /// left position
    Left,
    /// right position
    Right,
    /// top position
    Top,
    /// bottom position
    Bottom,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for GridPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GridPosition::{}",
            match *self {
                GridPosition::Left => "Left",
                GridPosition::Right => "Right",
                GridPosition::Top => "Top",
                GridPosition::Bottom => "Bottom",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for GridPosition {
    type GlibType = ffi::ClutterGridPosition;

    fn to_glib(&self) -> ffi::ClutterGridPosition {
        match *self {
            GridPosition::Left => ffi::CLUTTER_GRID_POSITION_LEFT,
            GridPosition::Right => ffi::CLUTTER_GRID_POSITION_RIGHT,
            GridPosition::Top => ffi::CLUTTER_GRID_POSITION_TOP,
            GridPosition::Bottom => ffi::CLUTTER_GRID_POSITION_BOTTOM,
            GridPosition::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterGridPosition> for GridPosition {
    fn from_glib(value: ffi::ClutterGridPosition) -> Self {
        match value {
            0 => GridPosition::Left,
            1 => GridPosition::Right,
            2 => GridPosition::Top,
            3 => GridPosition::Bottom,
            value => GridPosition::__Unknown(value),
        }
    }
}

impl StaticType for GridPosition {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_grid_position_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for GridPosition {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for GridPosition {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for GridPosition {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error enumeration for `Image`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ImageError {
    /// Invalid data passed to the
    ///  `ImageExt::set_data` function.
    Data,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ImageError::{}",
            match *self {
                ImageError::Data => "Data",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ImageError {
    type GlibType = ffi::ClutterImageError;

    fn to_glib(&self) -> ffi::ClutterImageError {
        match *self {
            ImageError::Data => ffi::CLUTTER_IMAGE_ERROR_INVALID_DATA,
            ImageError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterImageError> for ImageError {
    fn from_glib(value: ffi::ClutterImageError) -> Self {
        match value {
            0 => ImageError::Data,
            value => ImageError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ImageError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::clutter_image_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(ImageError::Data),
            value => Some(ImageError::__Unknown(value)),
        }
    }
}

impl StaticType for ImageError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_image_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ImageError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ImageError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ImageError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error conditions returned by `clutter_init` and `clutter_init_with_args`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum InitError {
    /// Initialisation successful
    Success,
    /// Unknown error
    ErrorUnknown,
    /// Thread initialisation failed
    ErrorThreads,
    /// Backend initialisation failed
    ErrorBackend,
    /// Internal error
    ErrorInternal,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InitError::{}",
            match *self {
                InitError::Success => "Success",
                InitError::ErrorUnknown => "ErrorUnknown",
                InitError::ErrorThreads => "ErrorThreads",
                InitError::ErrorBackend => "ErrorBackend",
                InitError::ErrorInternal => "ErrorInternal",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for InitError {
    type GlibType = ffi::ClutterInitError;

    fn to_glib(&self) -> ffi::ClutterInitError {
        match *self {
            InitError::Success => ffi::CLUTTER_INIT_SUCCESS,
            InitError::ErrorUnknown => ffi::CLUTTER_INIT_ERROR_UNKNOWN,
            InitError::ErrorThreads => ffi::CLUTTER_INIT_ERROR_THREADS,
            InitError::ErrorBackend => ffi::CLUTTER_INIT_ERROR_BACKEND,
            InitError::ErrorInternal => ffi::CLUTTER_INIT_ERROR_INTERNAL,
            InitError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterInitError> for InitError {
    fn from_glib(value: ffi::ClutterInitError) -> Self {
        match value {
            1 => InitError::Success,
            0 => InitError::ErrorUnknown,
            -1 => InitError::ErrorThreads,
            -2 => InitError::ErrorBackend,
            -3 => InitError::ErrorInternal,
            value => InitError::__Unknown(value),
        }
    }
}

impl ErrorDomain for InitError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::clutter_init_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            1 => Some(InitError::Success),
            0 => Some(InitError::ErrorUnknown),
            -1 => Some(InitError::ErrorThreads),
            -2 => Some(InitError::ErrorBackend),
            -3 => Some(InitError::ErrorInternal),
            value => Some(InitError::__Unknown(value)),
        }
    }
}

impl StaticType for InitError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_init_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InitError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InitError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InitError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The type of axes Clutter recognizes on a `InputDevice`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum InputAxis {
    /// Unused axis
    Ignore,
    /// The position on the X axis
    X,
    /// The position of the Y axis
    Y,
    /// The pressure information
    Pressure,
    /// The tilt on the X axis
    Xtilt,
    /// The tile on the Y axis
    Ytilt,
    /// A wheel
    Wheel,
    /// Distance (Since 1.12)
    Distance,
    /// Last value of the enumeration; this value is
    ///  useful when iterating over the enumeration values (Since 1.12)
    Last,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InputAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InputAxis::{}",
            match *self {
                InputAxis::Ignore => "Ignore",
                InputAxis::X => "X",
                InputAxis::Y => "Y",
                InputAxis::Pressure => "Pressure",
                InputAxis::Xtilt => "Xtilt",
                InputAxis::Ytilt => "Ytilt",
                InputAxis::Wheel => "Wheel",
                InputAxis::Distance => "Distance",
                InputAxis::Last => "Last",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for InputAxis {
    type GlibType = ffi::ClutterInputAxis;

    fn to_glib(&self) -> ffi::ClutterInputAxis {
        match *self {
            InputAxis::Ignore => ffi::CLUTTER_INPUT_AXIS_IGNORE,
            InputAxis::X => ffi::CLUTTER_INPUT_AXIS_X,
            InputAxis::Y => ffi::CLUTTER_INPUT_AXIS_Y,
            InputAxis::Pressure => ffi::CLUTTER_INPUT_AXIS_PRESSURE,
            InputAxis::Xtilt => ffi::CLUTTER_INPUT_AXIS_XTILT,
            InputAxis::Ytilt => ffi::CLUTTER_INPUT_AXIS_YTILT,
            InputAxis::Wheel => ffi::CLUTTER_INPUT_AXIS_WHEEL,
            InputAxis::Distance => ffi::CLUTTER_INPUT_AXIS_DISTANCE,
            InputAxis::Last => ffi::CLUTTER_INPUT_AXIS_LAST,
            InputAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterInputAxis> for InputAxis {
    fn from_glib(value: ffi::ClutterInputAxis) -> Self {
        match value {
            0 => InputAxis::Ignore,
            1 => InputAxis::X,
            2 => InputAxis::Y,
            3 => InputAxis::Pressure,
            4 => InputAxis::Xtilt,
            5 => InputAxis::Ytilt,
            6 => InputAxis::Wheel,
            7 => InputAxis::Distance,
            8 => InputAxis::Last,
            value => InputAxis::__Unknown(value),
        }
    }
}

impl StaticType for InputAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_input_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InputAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InputAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InputAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The types of input devices available.
///
/// The `InputDeviceType` enumeration can be extended at later
/// date; not every platform supports every input device type.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum InputDeviceType {
    /// A pointer device
    PointerDevice,
    /// A keyboard device
    KeyboardDevice,
    /// A generic extension device
    ExtensionDevice,
    /// A joystick device
    JoystickDevice,
    /// A tablet device
    TabletDevice,
    /// A touchpad device
    TouchpadDevice,
    /// A touch screen device
    TouchscreenDevice,
    /// A pen device
    PenDevice,
    /// An eraser device
    EraserDevice,
    /// A cursor device
    CursorDevice,
    /// The number of device types
    NDeviceTypes,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InputDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InputDeviceType::{}",
            match *self {
                InputDeviceType::PointerDevice => "PointerDevice",
                InputDeviceType::KeyboardDevice => "KeyboardDevice",
                InputDeviceType::ExtensionDevice => "ExtensionDevice",
                InputDeviceType::JoystickDevice => "JoystickDevice",
                InputDeviceType::TabletDevice => "TabletDevice",
                InputDeviceType::TouchpadDevice => "TouchpadDevice",
                InputDeviceType::TouchscreenDevice => "TouchscreenDevice",
                InputDeviceType::PenDevice => "PenDevice",
                InputDeviceType::EraserDevice => "EraserDevice",
                InputDeviceType::CursorDevice => "CursorDevice",
                InputDeviceType::NDeviceTypes => "NDeviceTypes",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for InputDeviceType {
    type GlibType = ffi::ClutterInputDeviceType;

    fn to_glib(&self) -> ffi::ClutterInputDeviceType {
        match *self {
            InputDeviceType::PointerDevice => ffi::CLUTTER_POINTER_DEVICE,
            InputDeviceType::KeyboardDevice => ffi::CLUTTER_KEYBOARD_DEVICE,
            InputDeviceType::ExtensionDevice => ffi::CLUTTER_EXTENSION_DEVICE,
            InputDeviceType::JoystickDevice => ffi::CLUTTER_JOYSTICK_DEVICE,
            InputDeviceType::TabletDevice => ffi::CLUTTER_TABLET_DEVICE,
            InputDeviceType::TouchpadDevice => ffi::CLUTTER_TOUCHPAD_DEVICE,
            InputDeviceType::TouchscreenDevice => ffi::CLUTTER_TOUCHSCREEN_DEVICE,
            InputDeviceType::PenDevice => ffi::CLUTTER_PEN_DEVICE,
            InputDeviceType::EraserDevice => ffi::CLUTTER_ERASER_DEVICE,
            InputDeviceType::CursorDevice => ffi::CLUTTER_CURSOR_DEVICE,
            InputDeviceType::NDeviceTypes => ffi::CLUTTER_N_DEVICE_TYPES,
            InputDeviceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterInputDeviceType> for InputDeviceType {
    fn from_glib(value: ffi::ClutterInputDeviceType) -> Self {
        match value {
            0 => InputDeviceType::PointerDevice,
            1 => InputDeviceType::KeyboardDevice,
            2 => InputDeviceType::ExtensionDevice,
            3 => InputDeviceType::JoystickDevice,
            4 => InputDeviceType::TabletDevice,
            5 => InputDeviceType::TouchpadDevice,
            6 => InputDeviceType::TouchscreenDevice,
            7 => InputDeviceType::PenDevice,
            8 => InputDeviceType::EraserDevice,
            9 => InputDeviceType::CursorDevice,
            10 => InputDeviceType::NDeviceTypes,
            value => InputDeviceType::__Unknown(value),
        }
    }
}

impl StaticType for InputDeviceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_input_device_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InputDeviceType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InputDeviceType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InputDeviceType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The mode for input devices available.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum InputMode {
    /// A master, virtual device
    Master,
    /// A slave, physical device, attached to
    ///  a master device
    Slave,
    /// A slave, physical device, not attached
    ///  to a master device
    Floating,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InputMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InputMode::{}",
            match *self {
                InputMode::Master => "Master",
                InputMode::Slave => "Slave",
                InputMode::Floating => "Floating",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for InputMode {
    type GlibType = ffi::ClutterInputMode;

    fn to_glib(&self) -> ffi::ClutterInputMode {
        match *self {
            InputMode::Master => ffi::CLUTTER_INPUT_MODE_MASTER,
            InputMode::Slave => ffi::CLUTTER_INPUT_MODE_SLAVE,
            InputMode::Floating => ffi::CLUTTER_INPUT_MODE_FLOATING,
            InputMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterInputMode> for InputMode {
    fn from_glib(value: ffi::ClutterInputMode) -> Self {
        match value {
            0 => InputMode::Master,
            1 => InputMode::Slave,
            2 => InputMode::Floating,
            value => InputMode::__Unknown(value),
        }
    }
}

impl StaticType for InputMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_input_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for InputMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for InputMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for InputMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The states for the `ClickAction::long-press` signal.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum LongPressState {
    /// Queries the action whether it supports
    ///  long presses
    Query,
    /// Activates the action on a long press
    Activate,
    /// The long press was cancelled
    Cancel,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for LongPressState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LongPressState::{}",
            match *self {
                LongPressState::Query => "Query",
                LongPressState::Activate => "Activate",
                LongPressState::Cancel => "Cancel",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for LongPressState {
    type GlibType = ffi::ClutterLongPressState;

    fn to_glib(&self) -> ffi::ClutterLongPressState {
        match *self {
            LongPressState::Query => ffi::CLUTTER_LONG_PRESS_QUERY,
            LongPressState::Activate => ffi::CLUTTER_LONG_PRESS_ACTIVATE,
            LongPressState::Cancel => ffi::CLUTTER_LONG_PRESS_CANCEL,
            LongPressState::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterLongPressState> for LongPressState {
    fn from_glib(value: ffi::ClutterLongPressState) -> Self {
        match value {
            0 => LongPressState::Query,
            1 => LongPressState::Activate,
            2 => LongPressState::Cancel,
            value => LongPressState::__Unknown(value),
        }
    }
}

impl StaticType for LongPressState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_long_press_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for LongPressState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for LongPressState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for LongPressState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Represents the orientation of actors or layout managers.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum Orientation {
    /// An horizontal orientation
    Horizontal,
    /// A vertical orientation
    Vertical,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Orientation::{}",
            match *self {
                Orientation::Horizontal => "Horizontal",
                Orientation::Vertical => "Vertical",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for Orientation {
    type GlibType = ffi::ClutterOrientation;

    fn to_glib(&self) -> ffi::ClutterOrientation {
        match *self {
            Orientation::Horizontal => ffi::CLUTTER_ORIENTATION_HORIZONTAL,
            Orientation::Vertical => ffi::CLUTTER_ORIENTATION_VERTICAL,
            Orientation::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterOrientation> for Orientation {
    fn from_glib(value: ffi::ClutterOrientation) -> Self {
        match value {
            0 => Orientation::Horizontal,
            1 => Orientation::Vertical,
            value => Orientation::__Unknown(value),
        }
    }
}

impl StaticType for Orientation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_orientation_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Orientation {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Orientation {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Orientation {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The axis of the constraint that should be applied on the
/// panning action
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PanAxis {
    /// No constraint
    AxisNone,
    /// Set a constraint on the X axis
    XAxis,
    /// Set a constraint on the Y axis
    YAxis,
    /// Constrain panning automatically based on initial
    ///  movement (available since 1.24)
    AxisAuto,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PanAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PanAxis::{}",
            match *self {
                PanAxis::AxisNone => "AxisNone",
                PanAxis::XAxis => "XAxis",
                PanAxis::YAxis => "YAxis",
                PanAxis::AxisAuto => "AxisAuto",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PanAxis {
    type GlibType = ffi::ClutterPanAxis;

    fn to_glib(&self) -> ffi::ClutterPanAxis {
        match *self {
            PanAxis::AxisNone => ffi::CLUTTER_PAN_AXIS_NONE,
            PanAxis::XAxis => ffi::CLUTTER_PAN_X_AXIS,
            PanAxis::YAxis => ffi::CLUTTER_PAN_Y_AXIS,
            PanAxis::AxisAuto => ffi::CLUTTER_PAN_AXIS_AUTO,
            PanAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterPanAxis> for PanAxis {
    fn from_glib(value: ffi::ClutterPanAxis) -> Self {
        match value {
            0 => PanAxis::AxisNone,
            1 => PanAxis::XAxis,
            2 => PanAxis::YAxis,
            3 => PanAxis::AxisAuto,
            value => PanAxis::__Unknown(value),
        }
    }
}

impl StaticType for PanAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_pan_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PanAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PanAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PanAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Types of nodes in a `Path`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PathNodeType {
    /// jump to the given position
    MoveTo,
    /// create a line from the last node to the
    ///  given position
    LineTo,
    /// bezier curve using the last position and
    ///  three control points.
    CurveTo,
    /// create a line from the last node to the last
    ///  `PathNodeType::MoveTo` node.
    Close,
    /// same as `PathNodeType::MoveTo` but with
    ///  coordinates relative to the last node.
    RelMoveTo,
    /// same as `PathNodeType::LineTo` but with
    ///  coordinates relative to the last node.
    RelLineTo,
    /// same as `PathNodeType::CurveTo` but with
    ///  coordinates relative to the last node.
    RelCurveTo,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PathNodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PathNodeType::{}",
            match *self {
                PathNodeType::MoveTo => "MoveTo",
                PathNodeType::LineTo => "LineTo",
                PathNodeType::CurveTo => "CurveTo",
                PathNodeType::Close => "Close",
                PathNodeType::RelMoveTo => "RelMoveTo",
                PathNodeType::RelLineTo => "RelLineTo",
                PathNodeType::RelCurveTo => "RelCurveTo",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PathNodeType {
    type GlibType = ffi::ClutterPathNodeType;

    fn to_glib(&self) -> ffi::ClutterPathNodeType {
        match *self {
            PathNodeType::MoveTo => ffi::CLUTTER_PATH_MOVE_TO,
            PathNodeType::LineTo => ffi::CLUTTER_PATH_LINE_TO,
            PathNodeType::CurveTo => ffi::CLUTTER_PATH_CURVE_TO,
            PathNodeType::Close => ffi::CLUTTER_PATH_CLOSE,
            PathNodeType::RelMoveTo => ffi::CLUTTER_PATH_REL_MOVE_TO,
            PathNodeType::RelLineTo => ffi::CLUTTER_PATH_REL_LINE_TO,
            PathNodeType::RelCurveTo => ffi::CLUTTER_PATH_REL_CURVE_TO,
            PathNodeType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterPathNodeType> for PathNodeType {
    fn from_glib(value: ffi::ClutterPathNodeType) -> Self {
        match value {
            0 => PathNodeType::MoveTo,
            1 => PathNodeType::LineTo,
            2 => PathNodeType::CurveTo,
            3 => PathNodeType::Close,
            32 => PathNodeType::RelMoveTo,
            33 => PathNodeType::RelLineTo,
            34 => PathNodeType::RelCurveTo,
            value => PathNodeType::__Unknown(value),
        }
    }
}

impl StaticType for PathNodeType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_path_node_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PathNodeType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PathNodeType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PathNodeType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Controls the paint cycle of the scene graph when in pick mode
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum PickMode {
    /// Do not paint any actor
    None,
    /// Paint only the reactive actors
    Reactive,
    /// Paint all actors
    All,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PickMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PickMode::{}",
            match *self {
                PickMode::None => "None",
                PickMode::Reactive => "Reactive",
                PickMode::All => "All",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for PickMode {
    type GlibType = ffi::ClutterPickMode;

    fn to_glib(&self) -> ffi::ClutterPickMode {
        match *self {
            PickMode::None => ffi::CLUTTER_PICK_NONE,
            PickMode::Reactive => ffi::CLUTTER_PICK_REACTIVE,
            PickMode::All => ffi::CLUTTER_PICK_ALL,
            PickMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterPickMode> for PickMode {
    fn from_glib(value: ffi::ClutterPickMode) -> Self {
        match value {
            0 => PickMode::None,
            1 => PickMode::Reactive,
            2 => PickMode::All,
            value => PickMode::__Unknown(value),
        }
    }
}

impl StaticType for PickMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_pick_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PickMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PickMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for PickMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Specifies the type of requests for a `Actor`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RequestMode {
    /// Height for width requests
    HeightForWidth,
    /// Width for height requests
    WidthForHeight,
    /// Use the preferred size of the
    ///  `Content`, if it has any (available since 1.22)
    ContentSize,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RequestMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RequestMode::{}",
            match *self {
                RequestMode::HeightForWidth => "HeightForWidth",
                RequestMode::WidthForHeight => "WidthForHeight",
                RequestMode::ContentSize => "ContentSize",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RequestMode {
    type GlibType = ffi::ClutterRequestMode;

    fn to_glib(&self) -> ffi::ClutterRequestMode {
        match *self {
            RequestMode::HeightForWidth => ffi::CLUTTER_REQUEST_HEIGHT_FOR_WIDTH,
            RequestMode::WidthForHeight => ffi::CLUTTER_REQUEST_WIDTH_FOR_HEIGHT,
            RequestMode::ContentSize => ffi::CLUTTER_REQUEST_CONTENT_SIZE,
            RequestMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterRequestMode> for RequestMode {
    fn from_glib(value: ffi::ClutterRequestMode) -> Self {
        match value {
            0 => RequestMode::HeightForWidth,
            1 => RequestMode::WidthForHeight,
            2 => RequestMode::ContentSize,
            value => RequestMode::__Unknown(value),
        }
    }
}

impl StaticType for RequestMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_request_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RequestMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RequestMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RequestMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Axis of a rotation.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RotateAxis {
    /// Rotate around the X axis
    XAxis,
    /// Rotate around the Y axis
    YAxis,
    /// Rotate around the Z axis
    ZAxis,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RotateAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RotateAxis::{}",
            match *self {
                RotateAxis::XAxis => "XAxis",
                RotateAxis::YAxis => "YAxis",
                RotateAxis::ZAxis => "ZAxis",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RotateAxis {
    type GlibType = ffi::ClutterRotateAxis;

    fn to_glib(&self) -> ffi::ClutterRotateAxis {
        match *self {
            RotateAxis::XAxis => ffi::CLUTTER_X_AXIS,
            RotateAxis::YAxis => ffi::CLUTTER_Y_AXIS,
            RotateAxis::ZAxis => ffi::CLUTTER_Z_AXIS,
            RotateAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterRotateAxis> for RotateAxis {
    fn from_glib(value: ffi::ClutterRotateAxis) -> Self {
        match value {
            0 => RotateAxis::XAxis,
            1 => RotateAxis::YAxis,
            2 => RotateAxis::ZAxis,
            value => RotateAxis::__Unknown(value),
        }
    }
}

impl StaticType for RotateAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_rotate_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RotateAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RotateAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RotateAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RotateDirection {
    Cw,
    Ccw,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for RotateDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RotateDirection::{}",
            match *self {
                RotateDirection::Cw => "Cw",
                RotateDirection::Ccw => "Ccw",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for RotateDirection {
    type GlibType = ffi::ClutterRotateDirection;

    fn to_glib(&self) -> ffi::ClutterRotateDirection {
        match *self {
            RotateDirection::Cw => ffi::CLUTTER_ROTATE_CW,
            RotateDirection::Ccw => ffi::CLUTTER_ROTATE_CCW,
            RotateDirection::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterRotateDirection> for RotateDirection {
    fn from_glib(value: ffi::ClutterRotateDirection) -> Self {
        match value {
            0 => RotateDirection::Cw,
            1 => RotateDirection::Ccw,
            value => RotateDirection::__Unknown(value),
        }
    }
}

impl StaticType for RotateDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_rotate_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RotateDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RotateDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for RotateDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The scaling filters to be used with the `Actor:minification-filter`
/// and `Actor:magnification-filter` properties.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ScalingFilter {
    /// Linear interpolation filter
    Linear,
    /// Nearest neighbor interpolation filter
    Nearest,
    /// Trilinear minification filter, with
    ///  mipmap generation; this filter linearly interpolates on every axis,
    ///  as well as between mipmap levels.
    Trilinear,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScalingFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScalingFilter::{}",
            match *self {
                ScalingFilter::Linear => "Linear",
                ScalingFilter::Nearest => "Nearest",
                ScalingFilter::Trilinear => "Trilinear",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ScalingFilter {
    type GlibType = ffi::ClutterScalingFilter;

    fn to_glib(&self) -> ffi::ClutterScalingFilter {
        match *self {
            ScalingFilter::Linear => ffi::CLUTTER_SCALING_FILTER_LINEAR,
            ScalingFilter::Nearest => ffi::CLUTTER_SCALING_FILTER_NEAREST,
            ScalingFilter::Trilinear => ffi::CLUTTER_SCALING_FILTER_TRILINEAR,
            ScalingFilter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScalingFilter> for ScalingFilter {
    fn from_glib(value: ffi::ClutterScalingFilter) -> Self {
        match value {
            0 => ScalingFilter::Linear,
            1 => ScalingFilter::Nearest,
            2 => ScalingFilter::Trilinear,
            value => ScalingFilter::__Unknown(value),
        }
    }
}

impl StaticType for ScalingFilter {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_scaling_filter_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScalingFilter {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScalingFilter {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ScalingFilter {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// `Script` error enumeration.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ScriptError {
    /// Type function not found
    ///  or invalid
    TypeFunction,
    /// Property not found or invalid
    Property,
    /// Invalid value
    Value,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScriptError::{}",
            match *self {
                ScriptError::TypeFunction => "TypeFunction",
                ScriptError::Property => "Property",
                ScriptError::Value => "Value",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ScriptError {
    type GlibType = ffi::ClutterScriptError;

    fn to_glib(&self) -> ffi::ClutterScriptError {
        match *self {
            ScriptError::TypeFunction => ffi::CLUTTER_SCRIPT_ERROR_INVALID_TYPE_FUNCTION,
            ScriptError::Property => ffi::CLUTTER_SCRIPT_ERROR_INVALID_PROPERTY,
            ScriptError::Value => ffi::CLUTTER_SCRIPT_ERROR_INVALID_VALUE,
            ScriptError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScriptError> for ScriptError {
    fn from_glib(value: ffi::ClutterScriptError) -> Self {
        match value {
            0 => ScriptError::TypeFunction,
            1 => ScriptError::Property,
            2 => ScriptError::Value,
            value => ScriptError::__Unknown(value),
        }
    }
}

impl ErrorDomain for ScriptError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::clutter_script_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(ScriptError::TypeFunction),
            1 => Some(ScriptError::Property),
            2 => Some(ScriptError::Value),
            value => Some(ScriptError::__Unknown(value)),
        }
    }
}

impl StaticType for ScriptError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_script_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScriptError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScriptError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ScriptError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Direction of a pointer scroll event.
///
/// The `ScrollDirection::Smooth` value implies that the `ScrollEvent`
/// has precise scrolling delta information.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ScrollDirection {
    /// Scroll up
    Up,
    /// Scroll down
    Down,
    /// Scroll left
    Left,
    /// Scroll right
    Right,
    /// Precise scrolling delta (available in 1.10)
    Smooth,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScrollDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScrollDirection::{}",
            match *self {
                ScrollDirection::Up => "Up",
                ScrollDirection::Down => "Down",
                ScrollDirection::Left => "Left",
                ScrollDirection::Right => "Right",
                ScrollDirection::Smooth => "Smooth",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ScrollDirection {
    type GlibType = ffi::ClutterScrollDirection;

    fn to_glib(&self) -> ffi::ClutterScrollDirection {
        match *self {
            ScrollDirection::Up => ffi::CLUTTER_SCROLL_UP,
            ScrollDirection::Down => ffi::CLUTTER_SCROLL_DOWN,
            ScrollDirection::Left => ffi::CLUTTER_SCROLL_LEFT,
            ScrollDirection::Right => ffi::CLUTTER_SCROLL_RIGHT,
            ScrollDirection::Smooth => ffi::CLUTTER_SCROLL_SMOOTH,
            ScrollDirection::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScrollDirection> for ScrollDirection {
    fn from_glib(value: ffi::ClutterScrollDirection) -> Self {
        match value {
            0 => ScrollDirection::Up,
            1 => ScrollDirection::Down,
            2 => ScrollDirection::Left,
            3 => ScrollDirection::Right,
            4 => ScrollDirection::Smooth,
            value => ScrollDirection::__Unknown(value),
        }
    }
}

impl StaticType for ScrollDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_scroll_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScrollDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScrollDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ScrollDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The scroll source determines the source of the scroll event. Keep in mind
/// that the source device `InputDeviceType` is not enough to infer
/// the scroll source.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ScrollSource {
    /// Source of scroll events is unknown.
    Unknown,
    /// The scroll event is originated by a mouse wheel.
    Wheel,
    /// The scroll event is originated by one or more
    ///  fingers on the device (eg. touchpads).
    Finger,
    /// The scroll event is originated by the
    ///  motion of some device (eg. a scroll button is set).
    Continuous,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ScrollSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ScrollSource::{}",
            match *self {
                ScrollSource::Unknown => "Unknown",
                ScrollSource::Wheel => "Wheel",
                ScrollSource::Finger => "Finger",
                ScrollSource::Continuous => "Continuous",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ScrollSource {
    type GlibType = ffi::ClutterScrollSource;

    fn to_glib(&self) -> ffi::ClutterScrollSource {
        match *self {
            ScrollSource::Unknown => ffi::CLUTTER_SCROLL_SOURCE_UNKNOWN,
            ScrollSource::Wheel => ffi::CLUTTER_SCROLL_SOURCE_WHEEL,
            ScrollSource::Finger => ffi::CLUTTER_SCROLL_SOURCE_FINGER,
            ScrollSource::Continuous => ffi::CLUTTER_SCROLL_SOURCE_CONTINUOUS,
            ScrollSource::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScrollSource> for ScrollSource {
    fn from_glib(value: ffi::ClutterScrollSource) -> Self {
        match value {
            0 => ScrollSource::Unknown,
            1 => ScrollSource::Wheel,
            2 => ScrollSource::Finger,
            3 => ScrollSource::Continuous,
            value => ScrollSource::__Unknown(value),
        }
    }
}

impl StaticType for ScrollSource {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_scroll_source_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScrollSource {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScrollSource {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ScrollSource {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The type of GLSL shader program
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ShaderType {
    /// a vertex shader
    VertexShader,
    /// a fragment shader
    FragmentShader,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ShaderType::{}",
            match *self {
                ShaderType::VertexShader => "VertexShader",
                ShaderType::FragmentShader => "FragmentShader",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ShaderType {
    type GlibType = ffi::ClutterShaderType;

    fn to_glib(&self) -> ffi::ClutterShaderType {
        match *self {
            ShaderType::VertexShader => ffi::CLUTTER_VERTEX_SHADER,
            ShaderType::FragmentShader => ffi::CLUTTER_FRAGMENT_SHADER,
            ShaderType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterShaderType> for ShaderType {
    fn from_glib(value: ffi::ClutterShaderType) -> Self {
        match value {
            0 => ShaderType::VertexShader,
            1 => ShaderType::FragmentShader,
            value => ShaderType::__Unknown(value),
        }
    }
}

impl StaticType for ShaderType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_shader_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ShaderType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ShaderType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ShaderType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The edge to snap
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum SnapEdge {
    /// the top edge
    Top,
    /// the right edge
    Right,
    /// the bottom edge
    Bottom,
    /// the left edge
    Left,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for SnapEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SnapEdge::{}",
            match *self {
                SnapEdge::Top => "Top",
                SnapEdge::Right => "Right",
                SnapEdge::Bottom => "Bottom",
                SnapEdge::Left => "Left",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for SnapEdge {
    type GlibType = ffi::ClutterSnapEdge;

    fn to_glib(&self) -> ffi::ClutterSnapEdge {
        match *self {
            SnapEdge::Top => ffi::CLUTTER_SNAP_EDGE_TOP,
            SnapEdge::Right => ffi::CLUTTER_SNAP_EDGE_RIGHT,
            SnapEdge::Bottom => ffi::CLUTTER_SNAP_EDGE_BOTTOM,
            SnapEdge::Left => ffi::CLUTTER_SNAP_EDGE_LEFT,
            SnapEdge::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterSnapEdge> for SnapEdge {
    fn from_glib(value: ffi::ClutterSnapEdge) -> Self {
        match value {
            0 => SnapEdge::Top,
            1 => SnapEdge::Right,
            2 => SnapEdge::Bottom,
            3 => SnapEdge::Left,
            value => SnapEdge::__Unknown(value),
        }
    }
}

impl StaticType for SnapEdge {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_snap_edge_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SnapEdge {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SnapEdge {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for SnapEdge {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Named colors, for accessing global colors defined by Clutter
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum StaticColor {
    /// White color (ffffffff)
    White,
    /// Black color (000000ff)
    Black,
    /// Red color (ff0000ff)
    Red,
    /// Dark red color (800000ff)
    DarkRed,
    /// Green color (00ff00ff)
    Green,
    /// Dark green color (008000ff)
    DarkGreen,
    /// Blue color (0000ffff)
    Blue,
    /// Dark blue color (000080ff)
    DarkBlue,
    /// Cyan color (00ffffff)
    Cyan,
    /// Dark cyan color (008080ff)
    DarkCyan,
    /// Magenta color (ff00ffff)
    Magenta,
    /// Dark magenta color (800080ff)
    DarkMagenta,
    /// Yellow color (ffff00ff)
    Yellow,
    /// Dark yellow color (808000ff)
    DarkYellow,
    /// Gray color (a0a0a4ff)
    Gray,
    /// Dark Gray color (808080ff)
    DarkGray,
    /// Light gray color (c0c0c0ff)
    LightGray,
    /// Butter color (edd400ff)
    Butter,
    /// Light butter color (fce94fff)
    ButterLight,
    /// Dark butter color (c4a000ff)
    ButterDark,
    /// Orange color (f57900ff)
    Orange,
    /// Light orange color (fcaf3fff)
    OrangeLight,
    /// Dark orange color (ce5c00ff)
    OrangeDark,
    /// Chocolate color (c17d11ff)
    Chocolate,
    /// Light chocolate color (e9b96eff)
    ChocolateLight,
    /// Dark chocolate color (8f5902ff)
    ChocolateDark,
    /// Chameleon color (73d216ff)
    Chameleon,
    /// Light chameleon color (8ae234ff)
    ChameleonLight,
    /// Dark chameleon color (4e9a06ff)
    ChameleonDark,
    /// Sky color (3465a4ff)
    SkyBlue,
    /// Light sky color (729fcfff)
    SkyBlueLight,
    /// Dark sky color (204a87ff)
    SkyBlueDark,
    /// Plum color (75507bff)
    Plum,
    /// Light plum color (ad7fa8ff)
    PlumLight,
    /// Dark plum color (5c3566ff)
    PlumDark,
    /// Scarlet red color (cc0000ff)
    ScarletRed,
    /// Light scarlet red color (ef2929ff)
    ScarletRedLight,
    /// Dark scarlet red color (a40000ff)
    ScarletRedDark,
    /// Aluminium, first variant (eeeeecff)
    Aluminium1,
    /// Aluminium, second variant (d3d7cfff)
    Aluminium2,
    /// Aluminium, third variant (babdb6ff)
    Aluminium3,
    /// Aluminium, fourth variant (888a85ff)
    Aluminium4,
    /// Aluminium, fifth variant (555753ff)
    Aluminium5,
    /// Aluminium, sixth variant (2e3436ff)
    Aluminium6,
    /// Transparent color (00000000)
    Transparent,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for StaticColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StaticColor::{}",
            match *self {
                StaticColor::White => "White",
                StaticColor::Black => "Black",
                StaticColor::Red => "Red",
                StaticColor::DarkRed => "DarkRed",
                StaticColor::Green => "Green",
                StaticColor::DarkGreen => "DarkGreen",
                StaticColor::Blue => "Blue",
                StaticColor::DarkBlue => "DarkBlue",
                StaticColor::Cyan => "Cyan",
                StaticColor::DarkCyan => "DarkCyan",
                StaticColor::Magenta => "Magenta",
                StaticColor::DarkMagenta => "DarkMagenta",
                StaticColor::Yellow => "Yellow",
                StaticColor::DarkYellow => "DarkYellow",
                StaticColor::Gray => "Gray",
                StaticColor::DarkGray => "DarkGray",
                StaticColor::LightGray => "LightGray",
                StaticColor::Butter => "Butter",
                StaticColor::ButterLight => "ButterLight",
                StaticColor::ButterDark => "ButterDark",
                StaticColor::Orange => "Orange",
                StaticColor::OrangeLight => "OrangeLight",
                StaticColor::OrangeDark => "OrangeDark",
                StaticColor::Chocolate => "Chocolate",
                StaticColor::ChocolateLight => "ChocolateLight",
                StaticColor::ChocolateDark => "ChocolateDark",
                StaticColor::Chameleon => "Chameleon",
                StaticColor::ChameleonLight => "ChameleonLight",
                StaticColor::ChameleonDark => "ChameleonDark",
                StaticColor::SkyBlue => "SkyBlue",
                StaticColor::SkyBlueLight => "SkyBlueLight",
                StaticColor::SkyBlueDark => "SkyBlueDark",
                StaticColor::Plum => "Plum",
                StaticColor::PlumLight => "PlumLight",
                StaticColor::PlumDark => "PlumDark",
                StaticColor::ScarletRed => "ScarletRed",
                StaticColor::ScarletRedLight => "ScarletRedLight",
                StaticColor::ScarletRedDark => "ScarletRedDark",
                StaticColor::Aluminium1 => "Aluminium1",
                StaticColor::Aluminium2 => "Aluminium2",
                StaticColor::Aluminium3 => "Aluminium3",
                StaticColor::Aluminium4 => "Aluminium4",
                StaticColor::Aluminium5 => "Aluminium5",
                StaticColor::Aluminium6 => "Aluminium6",
                StaticColor::Transparent => "Transparent",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for StaticColor {
    type GlibType = ffi::ClutterStaticColor;

    fn to_glib(&self) -> ffi::ClutterStaticColor {
        match *self {
            StaticColor::White => ffi::CLUTTER_COLOR_WHITE,
            StaticColor::Black => ffi::CLUTTER_COLOR_BLACK,
            StaticColor::Red => ffi::CLUTTER_COLOR_RED,
            StaticColor::DarkRed => ffi::CLUTTER_COLOR_DARK_RED,
            StaticColor::Green => ffi::CLUTTER_COLOR_GREEN,
            StaticColor::DarkGreen => ffi::CLUTTER_COLOR_DARK_GREEN,
            StaticColor::Blue => ffi::CLUTTER_COLOR_BLUE,
            StaticColor::DarkBlue => ffi::CLUTTER_COLOR_DARK_BLUE,
            StaticColor::Cyan => ffi::CLUTTER_COLOR_CYAN,
            StaticColor::DarkCyan => ffi::CLUTTER_COLOR_DARK_CYAN,
            StaticColor::Magenta => ffi::CLUTTER_COLOR_MAGENTA,
            StaticColor::DarkMagenta => ffi::CLUTTER_COLOR_DARK_MAGENTA,
            StaticColor::Yellow => ffi::CLUTTER_COLOR_YELLOW,
            StaticColor::DarkYellow => ffi::CLUTTER_COLOR_DARK_YELLOW,
            StaticColor::Gray => ffi::CLUTTER_COLOR_GRAY,
            StaticColor::DarkGray => ffi::CLUTTER_COLOR_DARK_GRAY,
            StaticColor::LightGray => ffi::CLUTTER_COLOR_LIGHT_GRAY,
            StaticColor::Butter => ffi::CLUTTER_COLOR_BUTTER,
            StaticColor::ButterLight => ffi::CLUTTER_COLOR_BUTTER_LIGHT,
            StaticColor::ButterDark => ffi::CLUTTER_COLOR_BUTTER_DARK,
            StaticColor::Orange => ffi::CLUTTER_COLOR_ORANGE,
            StaticColor::OrangeLight => ffi::CLUTTER_COLOR_ORANGE_LIGHT,
            StaticColor::OrangeDark => ffi::CLUTTER_COLOR_ORANGE_DARK,
            StaticColor::Chocolate => ffi::CLUTTER_COLOR_CHOCOLATE,
            StaticColor::ChocolateLight => ffi::CLUTTER_COLOR_CHOCOLATE_LIGHT,
            StaticColor::ChocolateDark => ffi::CLUTTER_COLOR_CHOCOLATE_DARK,
            StaticColor::Chameleon => ffi::CLUTTER_COLOR_CHAMELEON,
            StaticColor::ChameleonLight => ffi::CLUTTER_COLOR_CHAMELEON_LIGHT,
            StaticColor::ChameleonDark => ffi::CLUTTER_COLOR_CHAMELEON_DARK,
            StaticColor::SkyBlue => ffi::CLUTTER_COLOR_SKY_BLUE,
            StaticColor::SkyBlueLight => ffi::CLUTTER_COLOR_SKY_BLUE_LIGHT,
            StaticColor::SkyBlueDark => ffi::CLUTTER_COLOR_SKY_BLUE_DARK,
            StaticColor::Plum => ffi::CLUTTER_COLOR_PLUM,
            StaticColor::PlumLight => ffi::CLUTTER_COLOR_PLUM_LIGHT,
            StaticColor::PlumDark => ffi::CLUTTER_COLOR_PLUM_DARK,
            StaticColor::ScarletRed => ffi::CLUTTER_COLOR_SCARLET_RED,
            StaticColor::ScarletRedLight => ffi::CLUTTER_COLOR_SCARLET_RED_LIGHT,
            StaticColor::ScarletRedDark => ffi::CLUTTER_COLOR_SCARLET_RED_DARK,
            StaticColor::Aluminium1 => ffi::CLUTTER_COLOR_ALUMINIUM_1,
            StaticColor::Aluminium2 => ffi::CLUTTER_COLOR_ALUMINIUM_2,
            StaticColor::Aluminium3 => ffi::CLUTTER_COLOR_ALUMINIUM_3,
            StaticColor::Aluminium4 => ffi::CLUTTER_COLOR_ALUMINIUM_4,
            StaticColor::Aluminium5 => ffi::CLUTTER_COLOR_ALUMINIUM_5,
            StaticColor::Aluminium6 => ffi::CLUTTER_COLOR_ALUMINIUM_6,
            StaticColor::Transparent => ffi::CLUTTER_COLOR_TRANSPARENT,
            StaticColor::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterStaticColor> for StaticColor {
    fn from_glib(value: ffi::ClutterStaticColor) -> Self {
        match value {
            0 => StaticColor::White,
            1 => StaticColor::Black,
            2 => StaticColor::Red,
            3 => StaticColor::DarkRed,
            4 => StaticColor::Green,
            5 => StaticColor::DarkGreen,
            6 => StaticColor::Blue,
            7 => StaticColor::DarkBlue,
            8 => StaticColor::Cyan,
            9 => StaticColor::DarkCyan,
            10 => StaticColor::Magenta,
            11 => StaticColor::DarkMagenta,
            12 => StaticColor::Yellow,
            13 => StaticColor::DarkYellow,
            14 => StaticColor::Gray,
            15 => StaticColor::DarkGray,
            16 => StaticColor::LightGray,
            17 => StaticColor::Butter,
            18 => StaticColor::ButterLight,
            19 => StaticColor::ButterDark,
            20 => StaticColor::Orange,
            21 => StaticColor::OrangeLight,
            22 => StaticColor::OrangeDark,
            23 => StaticColor::Chocolate,
            24 => StaticColor::ChocolateLight,
            25 => StaticColor::ChocolateDark,
            26 => StaticColor::Chameleon,
            27 => StaticColor::ChameleonLight,
            28 => StaticColor::ChameleonDark,
            29 => StaticColor::SkyBlue,
            30 => StaticColor::SkyBlueLight,
            31 => StaticColor::SkyBlueDark,
            32 => StaticColor::Plum,
            33 => StaticColor::PlumLight,
            34 => StaticColor::PlumDark,
            35 => StaticColor::ScarletRed,
            36 => StaticColor::ScarletRedLight,
            37 => StaticColor::ScarletRedDark,
            38 => StaticColor::Aluminium1,
            39 => StaticColor::Aluminium2,
            40 => StaticColor::Aluminium3,
            41 => StaticColor::Aluminium4,
            42 => StaticColor::Aluminium5,
            43 => StaticColor::Aluminium6,
            44 => StaticColor::Transparent,
            value => StaticColor::__Unknown(value),
        }
    }
}

impl StaticType for StaticColor {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_static_color_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StaticColor {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StaticColor {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for StaticColor {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Change the value transition of a step function.
///
/// See `TimelineExt::set_step_progress`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum StepMode {
    /// The change in the value of a
    ///  `CLUTTER_STEP` progress mode should occur at the start of
    ///  the transition
    Start,
    /// The change in the value of a
    ///  `CLUTTER_STEP` progress mode should occur at the end of
    ///  the transition
    End,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for StepMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StepMode::{}",
            match *self {
                StepMode::Start => "Start",
                StepMode::End => "End",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for StepMode {
    type GlibType = ffi::ClutterStepMode;

    fn to_glib(&self) -> ffi::ClutterStepMode {
        match *self {
            StepMode::Start => ffi::CLUTTER_STEP_MODE_START,
            StepMode::End => ffi::CLUTTER_STEP_MODE_END,
            StepMode::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterStepMode> for StepMode {
    fn from_glib(value: ffi::ClutterStepMode) -> Self {
        match value {
            0 => StepMode::Start,
            1 => StepMode::End,
            value => StepMode::__Unknown(value),
        }
    }
}

impl StaticType for StepMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_step_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StepMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StepMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for StepMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The text direction to be used by `Actor`<!-- -->s
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TextDirection {
    /// Use the default setting, as returned
    ///  by `clutter_get_default_text_direction`
    Default,
    /// Use left-to-right text direction
    Ltr,
    /// Use right-to-left text direction
    Rtl,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextDirection::{}",
            match *self {
                TextDirection::Default => "Default",
                TextDirection::Ltr => "Ltr",
                TextDirection::Rtl => "Rtl",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TextDirection {
    type GlibType = ffi::ClutterTextDirection;

    fn to_glib(&self) -> ffi::ClutterTextDirection {
        match *self {
            TextDirection::Default => ffi::CLUTTER_TEXT_DIRECTION_DEFAULT,
            TextDirection::Ltr => ffi::CLUTTER_TEXT_DIRECTION_LTR,
            TextDirection::Rtl => ffi::CLUTTER_TEXT_DIRECTION_RTL,
            TextDirection::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterTextDirection> for TextDirection {
    fn from_glib(value: ffi::ClutterTextDirection) -> Self {
        match value {
            0 => TextDirection::Default,
            1 => TextDirection::Ltr,
            2 => TextDirection::Rtl,
            value => TextDirection::__Unknown(value),
        }
    }
}

impl StaticType for TextDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_text_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TextDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// Error enumeration for `Texture`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TextureError {
    /// OOM condition
    OutOfMemory,
    /// YUV operation attempted but no YUV support
    ///  found
    NoYuv,
    /// The requested format for
    /// clutter_texture_set_from_rgb_data or
    /// clutter_texture_set_from_yuv_data is unsupported.
    BadFormat,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TextureError::{}",
            match *self {
                TextureError::OutOfMemory => "OutOfMemory",
                TextureError::NoYuv => "NoYuv",
                TextureError::BadFormat => "BadFormat",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TextureError {
    type GlibType = ffi::ClutterTextureError;

    fn to_glib(&self) -> ffi::ClutterTextureError {
        match *self {
            TextureError::OutOfMemory => ffi::CLUTTER_TEXTURE_ERROR_OUT_OF_MEMORY,
            TextureError::NoYuv => ffi::CLUTTER_TEXTURE_ERROR_NO_YUV,
            TextureError::BadFormat => ffi::CLUTTER_TEXTURE_ERROR_BAD_FORMAT,
            TextureError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterTextureError> for TextureError {
    fn from_glib(value: ffi::ClutterTextureError) -> Self {
        match value {
            0 => TextureError::OutOfMemory,
            1 => TextureError::NoYuv,
            2 => TextureError::BadFormat,
            value => TextureError::__Unknown(value),
        }
    }
}

impl ErrorDomain for TextureError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::clutter_texture_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(TextureError::OutOfMemory),
            1 => Some(TextureError::NoYuv),
            2 => Some(TextureError::BadFormat),
            value => Some(TextureError::__Unknown(value)),
        }
    }
}

impl StaticType for TextureError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_texture_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TextureError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TextureError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TextureError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The direction of a `Timeline`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TimelineDirection {
    /// forward direction for a timeline
    Forward,
    /// backward direction for a timeline
    Backward,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TimelineDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimelineDirection::{}",
            match *self {
                TimelineDirection::Forward => "Forward",
                TimelineDirection::Backward => "Backward",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TimelineDirection {
    type GlibType = ffi::ClutterTimelineDirection;

    fn to_glib(&self) -> ffi::ClutterTimelineDirection {
        match *self {
            TimelineDirection::Forward => ffi::CLUTTER_TIMELINE_FORWARD,
            TimelineDirection::Backward => ffi::CLUTTER_TIMELINE_BACKWARD,
            TimelineDirection::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterTimelineDirection> for TimelineDirection {
    fn from_glib(value: ffi::ClutterTimelineDirection) -> Self {
        match value {
            0 => TimelineDirection::Forward,
            1 => TimelineDirection::Backward,
            value => TimelineDirection::__Unknown(value),
        }
    }
}

impl StaticType for TimelineDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_timeline_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TimelineDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TimelineDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TimelineDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The phase of a touchpad gesture event. All gestures are guaranteed to
/// begin with an event of type `TouchpadGesturePhase::Begin`,
/// followed by a number of `TouchpadGesturePhase::Update` (possibly 0).
///
/// A finished gesture may have 2 possible outcomes, an event with phase
/// `TouchpadGesturePhase::End` will be emitted when the gesture is
/// considered successful, this should be used as the hint to perform any
/// permanent changes.
///
/// Cancelled gestures may be so for a variety of reasons, due to hardware,
/// or due to the gesture recognition layers hinting the gesture did not
/// finish resolutely (eg. a 3rd finger being added during a pinch gesture).
/// In these cases, the last event with report the phase
/// `TouchpadGesturePhase::Cancel`, this should be used as a hint
/// to undo any visible/permanent changes that were done throughout the
/// progress of the gesture.
///
/// See also `TouchpadPinchEvent` and `TouchpadPinchEvent`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum TouchpadGesturePhase {
    /// The gesture has begun.
    Begin,
    /// The gesture has been updated.
    Update,
    /// The gesture was finished, changes
    ///  should be permanently applied.
    End,
    /// The gesture was cancelled, all
    ///  changes should be undone.
    Cancel,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for TouchpadGesturePhase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TouchpadGesturePhase::{}",
            match *self {
                TouchpadGesturePhase::Begin => "Begin",
                TouchpadGesturePhase::Update => "Update",
                TouchpadGesturePhase::End => "End",
                TouchpadGesturePhase::Cancel => "Cancel",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for TouchpadGesturePhase {
    type GlibType = ffi::ClutterTouchpadGesturePhase;

    fn to_glib(&self) -> ffi::ClutterTouchpadGesturePhase {
        match *self {
            TouchpadGesturePhase::Begin => ffi::CLUTTER_TOUCHPAD_GESTURE_PHASE_BEGIN,
            TouchpadGesturePhase::Update => ffi::CLUTTER_TOUCHPAD_GESTURE_PHASE_UPDATE,
            TouchpadGesturePhase::End => ffi::CLUTTER_TOUCHPAD_GESTURE_PHASE_END,
            TouchpadGesturePhase::Cancel => ffi::CLUTTER_TOUCHPAD_GESTURE_PHASE_CANCEL,
            TouchpadGesturePhase::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterTouchpadGesturePhase> for TouchpadGesturePhase {
    fn from_glib(value: ffi::ClutterTouchpadGesturePhase) -> Self {
        match value {
            0 => TouchpadGesturePhase::Begin,
            1 => TouchpadGesturePhase::Update,
            2 => TouchpadGesturePhase::End,
            3 => TouchpadGesturePhase::Cancel,
            value => TouchpadGesturePhase::__Unknown(value),
        }
    }
}

impl StaticType for TouchpadGesturePhase {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_touchpad_gesture_phase_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TouchpadGesturePhase {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TouchpadGesturePhase {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for TouchpadGesturePhase {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The type of unit in which a value is expressed
///
/// This enumeration might be expanded at later date
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum UnitType {
    /// Unit expressed in pixels (with subpixel precision)
    Pixel,
    /// Unit expressed in em
    Em,
    /// Unit expressed in millimeters
    Mm,
    /// Unit expressed in points
    Point,
    /// Unit expressed in centimeters
    Cm,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UnitType::{}",
            match *self {
                UnitType::Pixel => "Pixel",
                UnitType::Em => "Em",
                UnitType::Mm => "Mm",
                UnitType::Point => "Point",
                UnitType::Cm => "Cm",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for UnitType {
    type GlibType = ffi::ClutterUnitType;

    fn to_glib(&self) -> ffi::ClutterUnitType {
        match *self {
            UnitType::Pixel => ffi::CLUTTER_UNIT_PIXEL,
            UnitType::Em => ffi::CLUTTER_UNIT_EM,
            UnitType::Mm => ffi::CLUTTER_UNIT_MM,
            UnitType::Point => ffi::CLUTTER_UNIT_POINT,
            UnitType::Cm => ffi::CLUTTER_UNIT_CM,
            UnitType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterUnitType> for UnitType {
    fn from_glib(value: ffi::ClutterUnitType) -> Self {
        match value {
            0 => UnitType::Pixel,
            1 => UnitType::Em,
            2 => UnitType::Mm,
            3 => UnitType::Point,
            4 => UnitType::Cm,
            value => UnitType::__Unknown(value),
        }
    }
}

impl StaticType for UnitType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_unit_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for UnitType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for UnitType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for UnitType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

/// The axis of the constraint that should be applied by the
/// zooming action.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ZoomAxis {
    /// Scale only on the X axis
    XAxis,
    /// Scale only on the Y axis
    YAxis,
    /// Scale on both axis
    Both,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ZoomAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ZoomAxis::{}",
            match *self {
                ZoomAxis::XAxis => "XAxis",
                ZoomAxis::YAxis => "YAxis",
                ZoomAxis::Both => "Both",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ZoomAxis {
    type GlibType = ffi::ClutterZoomAxis;

    fn to_glib(&self) -> ffi::ClutterZoomAxis {
        match *self {
            ZoomAxis::XAxis => ffi::CLUTTER_ZOOM_X_AXIS,
            ZoomAxis::YAxis => ffi::CLUTTER_ZOOM_Y_AXIS,
            ZoomAxis::Both => ffi::CLUTTER_ZOOM_BOTH,
            ZoomAxis::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterZoomAxis> for ZoomAxis {
    fn from_glib(value: ffi::ClutterZoomAxis) -> Self {
        match value {
            0 => ZoomAxis::XAxis,
            1 => ZoomAxis::YAxis,
            2 => ZoomAxis::Both,
            value => ZoomAxis::__Unknown(value),
        }
    }
}

impl StaticType for ZoomAxis {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_zoom_axis_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ZoomAxis {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ZoomAxis {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ZoomAxis {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
