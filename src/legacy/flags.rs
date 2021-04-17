use glib::{
    translate::*,
    value::{FromValue, FromValueOptional, SetValue, Value},
    StaticType, Type,
};

bitflags! {
    pub struct ActorFlags: u32 {
        const MAPPED = 2;
        const REALIZED = 4;
        const REACTIVE = 8;
        const VISIBLE = 16;
        const NO_LAYOUT = 32;
    }
}

#[doc(hidden)]
impl ToGlib for ActorFlags {
    type GlibType = ffi::ClutterActorFlags;

    fn to_glib(&self) -> ffi::ClutterActorFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterActorFlags> for ActorFlags {
    fn from_glib(value: ffi::ClutterActorFlags) -> ActorFlags {
        ActorFlags::from_bits_truncate(value)
    }
}

impl StaticType for ActorFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_actor_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ActorFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ActorFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ActorFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct AllocationFlags: u32 {
        const ALLOCATION_NONE = 0;
        const ABSOLUTE_ORIGIN_CHANGED = 2;
        const DELEGATE_LAYOUT = 4;
    }
}

#[doc(hidden)]
impl ToGlib for AllocationFlags {
    type GlibType = ffi::ClutterAllocationFlags;

    fn to_glib(&self) -> ffi::ClutterAllocationFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterAllocationFlags> for AllocationFlags {
    fn from_glib(value: ffi::ClutterAllocationFlags) -> AllocationFlags {
        AllocationFlags::from_bits_truncate(value)
    }
}

impl StaticType for AllocationFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_allocation_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AllocationFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AllocationFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for AllocationFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ContentRepeat: u32 {
        const NONE = 0;
        const X_AXIS = 1;
        const Y_AXIS = 2;
        const BOTH = 3;
    }
}

#[doc(hidden)]
impl ToGlib for ContentRepeat {
    type GlibType = ffi::ClutterContentRepeat;

    fn to_glib(&self) -> ffi::ClutterContentRepeat {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterContentRepeat> for ContentRepeat {
    fn from_glib(value: ffi::ClutterContentRepeat) -> ContentRepeat {
        ContentRepeat::from_bits_truncate(value)
    }
}

impl StaticType for ContentRepeat {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_content_repeat_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ContentRepeat {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ContentRepeat {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ContentRepeat {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct EffectPaintFlags: u32 {
        const ACTOR_DIRTY = 1;
    }
}

#[doc(hidden)]
impl ToGlib for EffectPaintFlags {
    type GlibType = ffi::ClutterEffectPaintFlags;

    fn to_glib(&self) -> ffi::ClutterEffectPaintFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterEffectPaintFlags> for EffectPaintFlags {
    fn from_glib(value: ffi::ClutterEffectPaintFlags) -> EffectPaintFlags {
        EffectPaintFlags::from_bits_truncate(value)
    }
}

impl StaticType for EffectPaintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_effect_paint_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EffectPaintFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EffectPaintFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for EffectPaintFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct EventFlags: u32 {
        const NONE = 0;
        const FLAG_SYNTHETIC = 1;
    }
}

#[doc(hidden)]
impl ToGlib for EventFlags {
    type GlibType = ffi::ClutterEventFlags;

    fn to_glib(&self) -> ffi::ClutterEventFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterEventFlags> for EventFlags {
    fn from_glib(value: ffi::ClutterEventFlags) -> EventFlags {
        EventFlags::from_bits_truncate(value)
    }
}

impl StaticType for EventFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_event_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EventFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EventFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for EventFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct FeatureFlags: u32 {
        const TEXTURE_NPOT = 4;
        const SYNC_TO_VBLANK = 8;
        const TEXTURE_YUV = 16;
        const TEXTURE_READ_PIXELS = 32;
        const STAGE_STATIC = 64;
        const STAGE_USER_RESIZE = 128;
        const STAGE_CURSOR = 256;
        const SHADERS_GLSL = 512;
        const OFFSCREEN = 1024;
        const STAGE_MULTIPLE = 2048;
        const SWAP_EVENTS = 4096;
    }
}

#[doc(hidden)]
impl ToGlib for FeatureFlags {
    type GlibType = ffi::ClutterFeatureFlags;

    fn to_glib(&self) -> ffi::ClutterFeatureFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterFeatureFlags> for FeatureFlags {
    fn from_glib(value: ffi::ClutterFeatureFlags) -> FeatureFlags {
        FeatureFlags::from_bits_truncate(value)
    }
}

impl StaticType for FeatureFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_feature_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FeatureFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FeatureFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FeatureFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ModifierType: u32 {
        const SHIFT_MASK = 1;
        const LOCK_MASK = 2;
        const CONTROL_MASK = 4;
        const MOD1_MASK = 8;
        const MOD2_MASK = 16;
        const MOD3_MASK = 32;
        const MOD4_MASK = 64;
        const MOD5_MASK = 128;
        const BUTTON1_MASK = 256;
        const BUTTON2_MASK = 512;
        const BUTTON3_MASK = 1024;
        const BUTTON4_MASK = 2048;
        const BUTTON5_MASK = 4096;
        const MODIFIER_RESERVED_13_MASK = 8192;
        const MODIFIER_RESERVED_14_MASK = 16384;
        const MODIFIER_RESERVED_15_MASK = 32768;
        const MODIFIER_RESERVED_16_MASK = 65536;
        const MODIFIER_RESERVED_17_MASK = 131072;
        const MODIFIER_RESERVED_18_MASK = 262144;
        const MODIFIER_RESERVED_19_MASK = 524288;
        const MODIFIER_RESERVED_20_MASK = 1048576;
        const MODIFIER_RESERVED_21_MASK = 2097152;
        const MODIFIER_RESERVED_22_MASK = 4194304;
        const MODIFIER_RESERVED_23_MASK = 8388608;
        const MODIFIER_RESERVED_24_MASK = 16777216;
        const MODIFIER_RESERVED_25_MASK = 33554432;
        const SUPER_MASK = 67108864;
        const HYPER_MASK = 134217728;
        const META_MASK = 268435456;
        const MODIFIER_RESERVED_29_MASK = 536870912;
        const RELEASE_MASK = 1073741824;
        const MODIFIER_MASK = 1543512063;
    }
}

#[doc(hidden)]
impl ToGlib for ModifierType {
    type GlibType = ffi::ClutterModifierType;

    fn to_glib(&self) -> ffi::ClutterModifierType {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterModifierType> for ModifierType {
    fn from_glib(value: ffi::ClutterModifierType) -> ModifierType {
        ModifierType::from_bits_truncate(value)
    }
}

impl StaticType for ModifierType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_modifier_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ModifierType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ModifierType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ModifierType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct OffscreenRedirect: u32 {
        const AUTOMATIC_FOR_OPACITY = 1;
        const ALWAYS = 2;
    }
}

#[doc(hidden)]
impl ToGlib for OffscreenRedirect {
    type GlibType = ffi::ClutterOffscreenRedirect;

    fn to_glib(&self) -> ffi::ClutterOffscreenRedirect {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterOffscreenRedirect> for OffscreenRedirect {
    fn from_glib(value: ffi::ClutterOffscreenRedirect) -> OffscreenRedirect {
        OffscreenRedirect::from_bits_truncate(value)
    }
}

impl StaticType for OffscreenRedirect {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_offscreen_redirect_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for OffscreenRedirect {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for OffscreenRedirect {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for OffscreenRedirect {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct RepaintFlags: u32 {
        const PRE_PAINT = 1;
        const POST_PAINT = 2;
        const QUEUE_REDRAW_ON_ADD = 4;
    }
}

#[doc(hidden)]
impl ToGlib for RepaintFlags {
    type GlibType = ffi::ClutterRepaintFlags;

    fn to_glib(&self) -> ffi::ClutterRepaintFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterRepaintFlags> for RepaintFlags {
    fn from_glib(value: ffi::ClutterRepaintFlags) -> RepaintFlags {
        RepaintFlags::from_bits_truncate(value)
    }
}

impl StaticType for RepaintFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_repaint_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for RepaintFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for RepaintFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for RepaintFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ScrollFinishFlags: u32 {
        const NONE = 0;
        const HORIZONTAL = 1;
        const VERTICAL = 2;
    }
}

#[doc(hidden)]
impl ToGlib for ScrollFinishFlags {
    type GlibType = ffi::ClutterScrollFinishFlags;

    fn to_glib(&self) -> ffi::ClutterScrollFinishFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScrollFinishFlags> for ScrollFinishFlags {
    fn from_glib(value: ffi::ClutterScrollFinishFlags) -> ScrollFinishFlags {
        ScrollFinishFlags::from_bits_truncate(value)
    }
}

impl StaticType for ScrollFinishFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_scroll_finish_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScrollFinishFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScrollFinishFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ScrollFinishFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct ScrollMode: u32 {
        const NONE = 0;
        const HORIZONTALLY = 1;
        const VERTICALLY = 2;
        const BOTH = 3;
    }
}

#[doc(hidden)]
impl ToGlib for ScrollMode {
    type GlibType = ffi::ClutterScrollMode;

    fn to_glib(&self) -> ffi::ClutterScrollMode {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterScrollMode> for ScrollMode {
    fn from_glib(value: ffi::ClutterScrollMode) -> ScrollMode {
        ScrollMode::from_bits_truncate(value)
    }
}

impl StaticType for ScrollMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_scroll_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ScrollMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ScrollMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for ScrollMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct StageState: u32 {
        const FULLSCREEN = 2;
        const OFFSCREEN = 4;
        const ACTIVATED = 8;
    }
}

#[doc(hidden)]
impl ToGlib for StageState {
    type GlibType = ffi::ClutterStageState;

    fn to_glib(&self) -> ffi::ClutterStageState {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterStageState> for StageState {
    fn from_glib(value: ffi::ClutterStageState) -> StageState {
        StageState::from_bits_truncate(value)
    }
}

impl StaticType for StageState {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_stage_state_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for StageState {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for StageState {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for StageState {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

bitflags! {
    pub struct SwipeDirection: u32 {
        const UP = 1;
        const DOWN = 2;
        const LEFT = 4;
        const RIGHT = 8;
    }
}

#[doc(hidden)]
impl ToGlib for SwipeDirection {
    type GlibType = ffi::ClutterSwipeDirection;

    fn to_glib(&self) -> ffi::ClutterSwipeDirection {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ClutterSwipeDirection> for SwipeDirection {
    fn from_glib(value: ffi::ClutterSwipeDirection) -> SwipeDirection {
        SwipeDirection::from_bits_truncate(value)
    }
}

impl StaticType for SwipeDirection {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::clutter_swipe_direction_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SwipeDirection {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SwipeDirection {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for SwipeDirection {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}
