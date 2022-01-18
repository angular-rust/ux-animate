use std::fmt;

/// Controls how a `Actor` should align itself inside the extra space
/// assigned to it during the allocation.
///
/// Alignment only matters if the allocated space given to an actor is
/// bigger than its natural size; for example, when the `Actor:x-expand`
/// or the `Actor:y-expand` properties of `Actor` are set to `true`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// Specifies the axis on which `AlignConstraint` should maintain
/// the alignment.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum AlignAxis {
    /// Maintain the alignment on the X axis
    XAxis,
    /// Maintain the alignment on the Y axis
    YAxis,
    /// Maintain the alignment on both the X and Y axis
    Both,
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
            }
        )
    }
}

/// The animation modes used by `Alpha` and `Animation`. This
/// enumeration can be expanded in later versions of .
///
/// <figure id="easing-modes">
///  `<title>`Easing modes provided by `</title>`
///  <graphic fileref="easing-modes.png" format="PNG"/>
/// `</figure>`
///
/// Every global alpha function registered using `Alpha::register_func`
/// or `Alpha::register_closure` will have a logical id greater than
/// `AnimationMode::AnimationLast`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
}

impl Default for AnimationMode {
    fn default() -> Self {
        Self::Linear
    }
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
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum BinAlignment {
    Fixed,
    Fill,
    Start,
    End,
    Center,
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
            }
        )
    }
}

/// Specifies which property should be used in a binding
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
    ///  `BindCoordinate::Y` (added in  1.6)
    Position,
    /// Equivalent to `BindCoordinate::Width` and
    ///  `BindCoordinate::Height` (added in  1.6)
    Size,
    /// Equivalent to `BindCoordinate::Position` and
    ///  `BindCoordinate::Size` (added in  1.10)
    All,
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
            }
        )
    }
}

/// The alignment policies available on each axis of the `BoxLayout`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum BoxAlignment {
    /// Align the child to the top or to
    ///  to the left, depending on the used axis
    Start,
    /// Align the child to the bottom or to
    ///  the right, depending on the used axis
    End,
    /// Align the child to the center
    Center,
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
            }
        )
    }
}

/// Controls the alignment of the `Content` inside a `Actor`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
}

impl Default for ContentGravity {
    fn default() -> Self {
        Self::TopLeft
    }
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
            }
        )
    }
}

/// The axis of the constraint that should be applied on the
/// dragging action
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum DragAxis {
    /// No constraint
    None,
    /// Set a constraint on the X axis
    XAxis,
    /// Set a constraint on the Y axis
    YAxis,
}

impl fmt::Display for DragAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DragAxis::{}",
            match *self {
                DragAxis::None => "None",
                DragAxis::XAxis => "XAxis",
                DragAxis::YAxis => "YAxis",
            }
        )
    }
}

impl Default for DragAxis {
    fn default() -> Self {
        Self::None
    }
}

/// Types of events.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
}

impl Default for EventType {
    fn default() -> Self {
        Self::Nothing
    }
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
            }
        )
    }
}

/// The direction of the arrangement of the children inside
/// a `FlowLayout`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum FlowOrientation {
    /// Arrange the children of the flow layout
    ///  horizontally first
    Horizontal,
    /// Arrange the children of the flow layout
    ///  vertically first
    Vertical,
}

impl fmt::Display for FlowOrientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FlowOrientation::{}",
            match *self {
                FlowOrientation::Horizontal => "Horizontal",
                FlowOrientation::Vertical => "Vertical",
            }
        )
    }
}

/// Enum passed to the `GestureActionExt::set_threshold_trigger_edge`
/// function.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
}

impl Default for GestureTriggerEdge {
    fn default() -> Self {
        Self::None
    }
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
            }
        )
    }
}

/// Grid position modes.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum GridPosition {
    /// left position
    Left,
    /// right position
    Right,
    /// top position
    Top,
    /// bottom position
    Bottom,
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
            }
        )
    }
}

/// Error enumeration for `Image`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ImageError {
    /// Invalid data passed to the
    ///  `ImageExt::set_data` function.
    Data,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ImageError::{}",
            match *self {
                ImageError::Data => "Data",
            }
        )
    }
}

/// Error conditions returned by `init` and `init_with_args`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// The type of axes  recognizes on a `InputDevice`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// The types of input devices available.
///
/// The `InputDeviceType` enumeration can be extended at later
/// date; not every platform supports every input device type.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// The mode for input devices available.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum InputMode {
    /// A master, virtual device
    Master,
    /// A slave, physical device, attached to
    ///  a master device
    Slave,
    /// A slave, physical device, not attached
    ///  to a master device
    Floating,
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
            }
        )
    }
}

/// The states for the `ClickAction::long-press` signal.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum LongPressState {
    /// Queries the action whether it supports
    ///  long presses
    Query,
    /// Activates the action on a long press
    Activate,
    /// The long press was cancelled
    Cancel,
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
            }
        )
    }
}

/// Represents the orientation of actors or layout managers.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum Orientation {
    /// An horizontal orientation
    Horizontal,
    /// A vertical orientation
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Vertical        
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Orientation::{}",
            match *self {
                Orientation::Horizontal => "Horizontal",
                Orientation::Vertical => "Vertical",
            }
        )
    }
}

/// The axis of the constraint that should be applied on the
/// panning action
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum PanAxis {
    /// No constraint
    None,
    /// Set a constraint on the X axis
    XAxis,
    /// Set a constraint on the Y axis
    YAxis,
    /// Constrain panning automatically based on initial
    ///  movement (available since 1.24)
    AxisAuto,
}

impl Default for PanAxis {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for PanAxis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PanAxis::{}",
            match *self {
                PanAxis::None => "AxisNone",
                PanAxis::XAxis => "XAxis",
                PanAxis::YAxis => "YAxis",
                PanAxis::AxisAuto => "AxisAuto",
            }
        )
    }
}

/// Types of nodes in a `Path`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// Controls the paint cycle of the scene graph when in pick mode
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum PickMode {
    /// Do not paint any actor
    None,
    /// Paint only the reactive actors
    Reactive,
    /// Paint all actors
    All,
}

impl Default for PickMode {
    fn default() -> Self {
        PickMode::None
    }
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
            }
        )
    }
}

/// Specifies the type of requests for a `Actor`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum RequestMode {
    /// Height for width requests
    HeightForWidth,
    /// Width for height requests
    WidthForHeight,
    /// Use the preferred size of the
    ///  `Content`, if it has any (available since 1.22)
    ContentSize,
}

impl Default for RequestMode {
    fn default() -> Self {
        Self::HeightForWidth
    }
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
            }
        )
    }
}

/// Axis of a rotation.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum RotateAxis {
    /// Rotate around the X axis
    XAxis,
    /// Rotate around the Y axis
    YAxis,
    /// Rotate around the Z axis
    ZAxis,
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
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum RotateDirection {
    Cw,
    Ccw,
}

impl fmt::Display for RotateDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RotateDirection::{}",
            match *self {
                RotateDirection::Cw => "Cw",
                RotateDirection::Ccw => "Ccw",
            }
        )
    }
}

/// The scaling filters to be used with the `Actor:minification-filter`
/// and `Actor:magnification-filter` properties.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ScalingFilter {
    /// Linear interpolation filter
    Linear,
    /// Nearest neighbor interpolation filter
    Nearest,
    /// Trilinear minification filter, with
    ///  mipmap generation; this filter linearly interpolates on every axis,
    ///  as well as between mipmap levels.
    Trilinear,
}

impl Default for ScalingFilter {
    fn default() -> Self {
        Self::Linear
    }
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
            }
        )
    }
}

/// `Script` error enumeration.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ScriptError {
    /// Type function not found
    ///  or invalid
    TypeFunction,
    /// Property not found or invalid
    Property,
    /// Invalid value
    Value,
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
            }
        )
    }
}

/// Direction of a pointer scroll event.
///
/// The `ScrollDirection::Smooth` value implies that the `ScrollEvent`
/// has precise scrolling delta information.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
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
            }
        )
    }
}

/// The type of GLSL shader program
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ShaderType {
    /// a vertex shader
    VertexShader,
    /// a fragment shader
    FragmentShader,
}

impl fmt::Display for ShaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ShaderType::{}",
            match *self {
                ShaderType::VertexShader => "VertexShader",
                ShaderType::FragmentShader => "FragmentShader",
            }
        )
    }
}

/// The edge to snap
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum SnapEdge {
    /// the top edge
    Top,
    /// the right edge
    Right,
    /// the bottom edge
    Bottom,
    /// the left edge
    Left,
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
            }
        )
    }
}

/// Named colors, for accessing global colors defined by 
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// Change the value transition of a step function.
///
/// See `TimelineExt::set_step_progress`.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum StepMode {
    /// The change in the value of a
    ///  `STEP` progress mode should occur at the start of
    ///  the transition
    Start,
    /// The change in the value of a
    ///  `STEP` progress mode should occur at the end of
    ///  the transition
    End,
}

impl Default for StepMode {
    fn default() -> Self {
        Self::Start
    }
}

impl fmt::Display for StepMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "StepMode::{}",
            match *self {
                StepMode::Start => "Start",
                StepMode::End => "End",
            }
        )
    }
}

/// The text direction to be used by `Actor`<!-- -->s
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum TextDirection {
    /// Use the default setting, as returned
    ///  by `get_default_text_direction`
    Default,
    /// Use left-to-right text direction
    Ltr,
    /// Use right-to-left text direction
    Rtl,
}

impl Default for TextDirection {
    fn default() -> Self {
        Self::Default
    }
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
            }
        )
    }
}

/// Error enumeration for `Texture`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum TextureError {
    /// OOM condition
    OutOfMemory,
    /// YUV operation attempted but no YUV support
    ///  found
    NoYuv,
    /// The requested format for
    /// texture_set_from_rgb_data or
    /// texture_set_from_yuv_data is unsupported.
    BadFormat,
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
            }
        )
    }
}

/// The direction of a `Timeline`
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum TimelineDirection {
    /// forward direction for a timeline
    Forward,
    /// backward direction for a timeline
    Backward,
}

impl Default for TimelineDirection {
    fn default() -> Self {
        Self::Forward
    }
}

impl fmt::Display for TimelineDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TimelineDirection::{}",
            match *self {
                TimelineDirection::Forward => "Forward",
                TimelineDirection::Backward => "Backward",
            }
        )
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
            }
        )
    }
}

/// The type of unit in which a value is expressed
///
/// This enumeration might be expanded at later date
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
            }
        )
    }
}

/// The axis of the constraint that should be applied by the
/// zooming action.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub enum ZoomAxis {
    /// Scale only on the X axis
    XAxis,
    /// Scale only on the Y axis
    YAxis,
    /// Scale on both axis
    Both,
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
            }
        )
    }
}
