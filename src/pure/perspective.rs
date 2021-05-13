// * Perspective:
// * @fovy: the field of view angle, in degrees, in the y direction
// * @aspect: the aspect ratio that determines the field of view in the x
// *   direction. The aspect ratio is the ratio of x (width) to y (height)
// * @z_near: the distance from the viewer to the near clipping
// *   plane (always positive)
// * @z_far: the distance from the viewer to the far clipping
// *   plane (always positive)
// *
// * Stage perspective definition. #Perspective is only used by
// * the fixed point version of clutter_stage_set_perspective().
// *
#[derive(Debug, Clone)]
pub struct Perspective {
    fovy: f32,
    aspect: f32,
    z_near: f32,
    z_far: f32,
}

// * Fog:
// * @z_near: starting distance from the viewer to the near clipping
// *   plane (always positive)
// * @z_far: final distance from the viewer to the far clipping
// *   plane (always positive)
// *
// * Fog settings used to create the depth cueing effect.
// *
// * Since: 0.6
// *
// * Deprecated: 1.10: The fog-related API in #Stage has been
// *   deprecated as well.
#[derive(Debug, Clone)]
pub struct Fog {
    z_near: f32,
    z_far: f32,
}