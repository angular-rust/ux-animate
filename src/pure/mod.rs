#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into,
    clippy::upper_case_acronyms,
    clippy::new_ret_no_self,
    clippy::wrong_self_convention
)]

use crate::prelude;

mod action;
pub use self::action::Action;

mod actor;
pub use self::actor::{Actor, ActorExt};

mod actor_meta;
pub use self::actor_meta::{ActorMeta, ActorMetaExt};

mod align_constraint;
pub use self::align_constraint::AlignConstraint;

mod alpha;
pub use self::alpha::Alpha;

mod animatable;
pub use self::animatable::{Animatable, AnimatableExt};

mod backend;
pub use self::backend::Backend;

mod behaviour;
pub use self::behaviour::Behaviour;

mod behaviour_ellipse;
pub use self::behaviour_ellipse::{BehaviourEllipse, BehaviourEllipseExt};

mod behaviour_rotate;
pub use self::behaviour_rotate::{BehaviourRotate, BehaviourRotateExt};

mod bin_layout;
pub use self::bin_layout::BinLayout;

mod bind_constraint;
pub use self::bind_constraint::BindConstraint;

mod binding_pool;
pub use self::binding_pool::BindingPool;

mod blur_effect;
pub use self::blur_effect::BlurEffect;

mod box_;
pub use self::box_::Box;

mod box_layout;
pub use self::box_layout::{BoxLayout, BoxLayoutExt};

mod brightness_contrast_effect;
pub use self::brightness_contrast_effect::BrightnessContrastEffect;

mod actor_canvas;
pub use self::actor_canvas::{ActorCanvas, ActorCanvasExt};

mod child_meta;
pub use self::child_meta::{ChildMeta, ChildMetaExt};

mod click_action;
pub use self::click_action::{ClickAction, ClickActionExt};

mod clip_node;
pub use self::clip_node::{ClipNode};

mod clone;
pub use self::clone::{Clone, CloneExt};

mod color_node;
pub use self::color_node::ColorNode;

mod colorize_effect;
pub use self::colorize_effect::ColorizeEffect;

mod constraint;
pub use self::constraint::Constraint;

mod container;
pub use self::container::{Container, ContainerExt};

mod content;
pub use self::content::{Content, ContentExt};

mod deform_effect;
pub use self::deform_effect::{DeformEffect, DeformEffectExt};

mod desaturate_effect;
pub use self::desaturate_effect::DesaturateEffect;

mod device_manager;
pub use self::device_manager::{DeviceManager, DeviceManagerExt};

mod drag_action;
pub use self::drag_action::{DragAction, DragActionExt};

mod drop_action;
pub use self::drop_action::{DropAction, DropActionExt};

mod effect;
pub use self::effect::{Effect, EffectExt};

mod event_any;
pub use event_any::AnyEvent;

mod event_button;
pub use event_button::ButtonEvent;

mod event_crossing;
pub use event_crossing::CrossingEvent;

mod event_key;
pub use event_key::KeyEvent;

mod event_motion;
pub use event_motion::MotionEvent;

mod event_scroll;
pub use event_scroll::ScrollEvent;

mod event_stage_state;
pub use event_stage_state::StageStateEvent;

mod event_touch;
pub use event_touch::TouchEvent;

mod event_touchpad_pinch;
pub use event_touchpad_pinch::TouchpadPinchEvent;

mod event_touchpad_swipe;
pub use event_touchpad_swipe::TouchpadSwipeEvent;

mod event;
pub use event::{Event, FromEvent};

mod fixed_layout;
pub use self::fixed_layout::FixedLayout;

mod flow_layout;
pub use self::flow_layout::{FlowLayout, FlowLayoutExt};

mod gesture_action;
pub use self::gesture_action::{GestureAction, GestureActionExt};

mod grid_layout;
pub use self::grid_layout::{GridLayout, GridLayoutExt};

mod group;
pub use self::group::Group;

mod image;
pub use self::image::{Image, ImageExt};

mod input_device;
pub use self::input_device::InputDevice;

mod interval;
pub use self::interval::{Interval, IntervalExt};

mod keyframe_transition;
pub use self::keyframe_transition::{KeyframeTransition, KeyframeTransitionExt};

mod layout_manager;
pub use self::layout_manager::{LayoutManager, LayoutManagerExt};

mod layout_meta;
pub use self::layout_meta::{LayoutMeta, LayoutMetaExt};

mod media;
pub use self::media::{Media, MediaExt};

mod offscreen_effect;
pub use self::offscreen_effect::{OffscreenEffect, OffscreenEffectExt};

mod page_turn_effect;
pub use self::page_turn_effect::PageTurnEffect;

mod paint_node;
pub use self::paint_node::{PaintNode, PaintNodeExt};

mod pan_action;
pub use self::pan_action::{PanAction, PanActionExt};

// mod param_spec_color;
// pub use self::param_spec_color::{ParamSpecColor};

// mod param_spec_unit;
// pub use self::param_spec_unit::{ParamSpecUnit};

mod path;
pub use self::path::{Path, PathExt};

mod path_constraint;
pub use self::path_constraint::PathConstraint;

mod pipeline_node;
pub use self::pipeline_node::PipelineNode;

mod property_transition;
pub use self::property_transition::{PropertyTransition, PropertyTransitionExt};

mod rectangle;
pub use self::rectangle::{Rectangle, RectangleExt};

mod rotate_action;
pub use self::rotate_action::{RotateAction, RotateActionExt};

mod score;
pub use self::score::Score;

// mod script;
// pub use self::script::{Script, ScriptExt};

// mod scriptable;
// pub use self::scriptable::{Scriptable, ScriptableExt};

mod scroll_actor;
pub use self::scroll_actor::{ScrollActor, ScrollActorExt};

mod settings;
pub use self::settings::Settings;

mod shader_effect;
pub use self::shader_effect::{ShaderEffect, ShaderEffectExt};

mod shader_float;
pub use self::shader_float::ShaderFloat;

mod shader_int;
pub use self::shader_int::ShaderInt;

mod shader_matrix;
pub use self::shader_matrix::ShaderMatrix;

mod snap_constraint;
pub use self::snap_constraint::SnapConstraint;

mod stage;
pub use self::stage::{Stage, StageExt};

mod stage_manager;
pub use self::stage_manager::{StageManager, StageManagerExt};

mod state;
pub use self::state::State;

mod swipe_action;
pub use self::swipe_action::{SwipeAction, SwipeActionExt};

mod tap_action;
pub use self::tap_action::{TapAction, TapActionExt};

mod text;
pub use self::text::{Text, TextExt};

mod text_buffer;
pub use self::text_buffer::{TextBuffer, TextBufferExt};

mod text_node;
pub use self::text_node::TextNode;

mod texture;
pub use self::texture::{Texture, TextureExt};

mod texture_node;
pub use self::texture_node::TextureNode;

mod timeline;
pub use self::timeline::{Timeline, TimelineExt};

mod transition;
pub use self::transition::{Transition, TransitionExt};

mod transition_group;
pub use self::transition_group::{TransitionGroup, TransitionGroupExt};

mod zoom_action;
pub use self::zoom_action::{ZoomAction, ZoomActionExt};

mod actor_box;
pub use self::actor_box::ActorBox;

// mod color;
// pub(crate) use self::color::InternalColor;

mod event_sequence;
pub use self::event_sequence::EventSequence;

mod geometry;
pub use self::geometry::Geometry;

mod knot;
pub use self::knot::Knot;

mod margin;
pub use self::margin::Margin;

mod matrix;
pub use self::matrix::Matrix;

mod list_model;
pub use self::list_model::ListModel;

mod model;
pub use self::model::Model;

mod model_iter;
pub use self::model_iter::ModelIter;

mod paint_volume;
pub use self::paint_volume::PaintVolume;

mod path_node;
pub use self::path_node::PathNode;

mod perspective;
pub use self::perspective::Perspective;

mod point;
pub(crate) use self::point::InternalPoint;

mod rect;
pub(crate) use self::rect::InternalRect;

mod size;
pub(crate) use self::size::InternalSize;

mod state_key;
pub use self::state_key::StateKey;

mod units;
pub use self::units::Units;

mod vertex;
pub use self::vertex::Vertex;

mod enums;
pub use self::enums::ActorAlign;
pub use self::enums::AlignAxis;
pub use self::enums::AnimationMode;
pub use self::enums::BinAlignment;
pub use self::enums::BindCoordinate;
pub use self::enums::BoxAlignment;
pub use self::enums::ContentGravity;
pub use self::enums::DragAxis;
pub use self::enums::EventType;
pub use self::enums::FlowOrientation;
pub use self::enums::GestureTriggerEdge;
pub use self::enums::GridPosition;
pub use self::enums::ImageError;
pub use self::enums::InitError;
pub use self::enums::InputAxis;
pub use self::enums::InputDeviceType;
pub use self::enums::InputMode;
pub use self::enums::LongPressState;
pub use self::enums::Orientation;
pub use self::enums::PanAxis;
pub use self::enums::PathNodeType;
pub use self::enums::PickMode;
pub use self::enums::RequestMode;
pub use self::enums::RotateAxis;
pub use self::enums::RotateDirection;
pub use self::enums::ScalingFilter;
pub use self::enums::ScriptError;
pub use self::enums::ScrollDirection;
pub use self::enums::ScrollSource;
pub use self::enums::ShaderType;
pub use self::enums::SnapEdge;
pub use self::enums::StaticColor;
pub use self::enums::StepMode;
pub use self::enums::TextDirection;
pub use self::enums::TextureError;
pub use self::enums::TimelineDirection;
pub use self::enums::TouchpadGesturePhase;
pub use self::enums::UnitType;
pub use self::enums::ZoomAxis;

mod flags;
pub use self::flags::ActorFlags;
pub use self::flags::AllocationFlags;
pub use self::flags::ContentRepeat;
pub use self::flags::EffectPaintFlags;
pub use self::flags::EventFlags;
pub use self::flags::FeatureFlags;
pub use self::flags::ModifierType;
pub use self::flags::OffscreenRedirect;
pub use self::flags::RepaintFlags;
pub use self::flags::ScrollFinishFlags;
pub use self::flags::ScrollMode;
pub use self::flags::StageState;
pub use self::flags::SwipeDirection;

pub type ActorCreateChildFunc = ffi::ClutterActorCreateChildFunc;
pub type BindingActionFunc = ffi::ClutterBindingActionFunc;

#[doc(hidden)]
pub mod traits {
    pub use super::ActorCanvasExt;
    pub use super::ActorExt;
    pub use super::ActorMetaExt;
    pub use super::AnimatableExt;
    pub use super::BehaviourEllipseExt;
    pub use super::BehaviourRotateExt;
    pub use super::BoxLayoutExt;
    pub use super::ChildMetaExt;
    pub use super::ClickActionExt;
    pub use super::CloneExt;
    pub use super::ContainerExt;
    pub use super::ContentExt;
    pub use super::DeformEffectExt;
    pub use super::DeviceManagerExt;
    pub use super::DragActionExt;
    pub use super::DropActionExt;
    pub use super::EffectExt;
    pub use super::FlowLayoutExt;
    pub use super::GestureActionExt;
    pub use super::GridLayoutExt;
    pub use super::ImageExt;
    pub use super::IntervalExt;
    pub use super::KeyframeTransitionExt;
    pub use super::LayoutManagerExt;
    pub use super::LayoutMetaExt;
    pub use super::MediaExt;
    pub use super::OffscreenEffectExt;
    pub use super::PaintNodeExt;
    pub use super::PanActionExt;
    pub use super::PathExt;
    pub use super::PropertyTransitionExt;
    pub use super::RectangleExt;
    pub use super::RotateActionExt;
    // pub use super::ScriptExt;
    // pub use super::ScriptableExt;
    pub use super::ScrollActorExt;
    pub use super::ShaderEffectExt;
    pub use super::StageExt;
    pub use super::StageManagerExt;
    pub use super::SwipeActionExt;
    pub use super::TapActionExt;
    pub use super::TextBufferExt;
    pub use super::TextExt;
    pub use super::TextureExt;
    pub use super::TimelineExt;
    pub use super::TransitionExt;
    pub use super::TransitionGroupExt;
    pub use super::ZoomActionExt;
}

pub use glib::timeout_add_local as interval;

impl prelude::Object for dx::Texture {}
impl prelude::Is<dx::Texture> for dx::Texture {}
