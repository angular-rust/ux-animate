use crate::{ActorMeta, Effect, OffscreenEffect, ShaderType};
use glib::{
    object as gobject,
    object::{Cast, IsA},
    translate::*,
};
use std::fmt;

glib_wrapper! {
    pub struct ShaderEffect(Object<ffi::ClutterShaderEffect, ffi::ClutterShaderEffectClass, ShaderEffectClass>) @extends OffscreenEffect, Effect, ActorMeta, gobject::InitiallyUnowned;

    match fn {
        get_type => || ffi::clutter_shader_effect_get_type(),
    }
}

impl ShaderEffect {
    /// Creates a new `ShaderEffect`, to be applied to an actor using
    /// `ActorExt::add_effect`.
    ///
    /// The effect will be empty until `ShaderEffectExt::set_shader_source`
    /// is called.
    /// ## `shader_type`
    /// the type of the shader, either `ShaderType::FragmentShader`,
    ///  or `ShaderType::VertexShader`
    ///
    /// # Returns
    ///
    /// the newly created `ShaderEffect`.
    ///  Use `gobject::ObjectExt::unref` when done.
    pub fn new(shader_type: ShaderType) -> ShaderEffect {
        unsafe {
            Effect::from_glib_none(ffi::clutter_shader_effect_new(shader_type.to_glib()))
                .unsafe_cast()
        }
    }
}

/// Trait containing all `ShaderEffect` methods.
///
/// # Implementors
///
/// [`ShaderEffect`](struct.ShaderEffect.html)
pub trait ShaderEffectExt: 'static {
    //fn get_program(&self) -> /*Unimplemented*/Option<dx::Handle>;

    //fn get_shader(&self) -> /*Unimplemented*/Option<dx::Handle>;

    /// Sets the source of the GLSL shader used by `self`
    ///
    /// This function should only be called by implementations of
    /// the `ShaderEffect` class, and not by application code.
    ///
    /// This function can only be called once; subsequent calls will
    /// yield no result.
    /// ## `source`
    /// the source of a GLSL shader
    ///
    /// # Returns
    ///
    /// `true` if the source was set
    fn set_shader_source(&self, source: &str) -> bool;

    //fn set_uniform(&self, name: &str, gtype: glib::types::Type, n_values: usize, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Sets `value` as the payload for the uniform `name` inside the shader
    /// effect
    ///
    /// The `glib::Type` of the `value` must be one of: `G_TYPE_INT`, for a single
    /// integer value; `G_TYPE_FLOAT`, for a single floating point value;
    /// `TYPE_SHADER_INT`, for an array of integer values;
    /// `TYPE_SHADER_FLOAT`, for an array of floating point values;
    /// and `TYPE_SHADER_MATRIX`, for a matrix of floating point
    /// values. It also accepts `G_TYPE_DOUBLE` for compatibility with other
    /// languages than C.
    /// ## `name`
    /// the name of the uniform to set
    /// ## `value`
    /// a `gobject::Value` with the value of the uniform to set
    fn set_uniform_value(&self, name: &str, value: &glib::Value);
}

impl<O: IsA<ShaderEffect>> ShaderEffectExt for O {
    //fn get_program(&self) -> /*Unimplemented*/Option<dx::Handle> {
    //    unsafe { TODO: call clutter_sys:clutter_shader_effect_get_program() }
    //}

    //fn get_shader(&self) -> /*Unimplemented*/Option<dx::Handle> {
    //    unsafe { TODO: call clutter_sys:clutter_shader_effect_get_shader() }
    //}

    fn set_shader_source(&self, source: &str) -> bool {
        unsafe {
            from_glib(ffi::clutter_shader_effect_set_shader_source(
                self.as_ref().to_glib_none().0,
                source.to_glib_none().0,
            ))
        }
    }

    //fn set_uniform(&self, name: &str, gtype: glib::types::Type, n_values: usize, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call clutter_sys:clutter_shader_effect_set_uniform() }
    //}

    fn set_uniform_value(&self, name: &str, value: &glib::Value) {
        unsafe {
            ffi::clutter_shader_effect_set_uniform_value(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ShaderEffect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ShaderEffect")
    }
}
