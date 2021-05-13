bitflags! {
    pub struct ActorFlags: u32 {
        const MAPPED = 2;
        const REALIZED = 4;
        const REACTIVE = 8;
        const VISIBLE = 16;
        const NO_LAYOUT = 32;
    }
}

bitflags! {
    pub struct AllocationFlags: u32 {
        const ALLOCATION_NONE = 0;
        const ABSOLUTE_ORIGIN_CHANGED = 2;
        const DELEGATE_LAYOUT = 4;
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

bitflags! {
    pub struct EffectPaintFlags: u32 {
        const ACTOR_DIRTY = 1;
    }
}

bitflags! {
    pub struct EventFlags: u32 {
        const NONE = 0;
        const FLAG_SYNTHETIC = 1;
    }
}

bitflags! {
    pub struct FeatureFlags: u32 {
        const NONE = 0;
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

bitflags! {
    pub struct OffscreenRedirect: u32 {
        const AUTOMATIC_FOR_OPACITY = 1;
        const ALWAYS = 2;
    }
}

bitflags! {
    pub struct RepaintFlags: u32 {
        const PRE_PAINT = 1;
        const POST_PAINT = 2;
        const QUEUE_REDRAW_ON_ADD = 4;
    }
}

bitflags! {
    pub struct ScrollFinishFlags: u32 {
        const NONE = 0;
        const HORIZONTAL = 1;
        const VERTICAL = 2;
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

bitflags! {
    pub struct StageState: u32 {
        const FULLSCREEN = 2;
        const OFFSCREEN = 4;
        const ACTIVATED = 8;
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
