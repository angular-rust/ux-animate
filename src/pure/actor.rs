use super::{
    Action, ActorAlign, ActorBox, ActorFlags, AllocationFlags, AnimationMode, ButtonEvent,
    Constraint, Content, ContentGravity, ContentRepeat, CrossingEvent, Effect, Event, InternalRect,
    KeyEvent, LayoutManager, Margin, Matrix, MotionEvent, OffscreenRedirect, Orientation,
    PaintVolume, RequestMode, RotateAxis, ScalingFilter, ScrollEvent, Stage, TextDirection,
    Transition, Vertex,
};
use crate::prelude::*;
use crate::Color;
use glib::signal::SignalHandlerId;
use std::fmt;

// @implements Animatable, Container
#[derive(Debug, Clone)]
pub struct Actor {}

impl Actor {
    /// Creates a new `Actor`.
    ///
    /// A newly created actor has a floating reference, which will be sunk
    /// when it is added to another actor.
    ///
    /// # Returns
    ///
    /// the newly created `Actor`
    pub fn new() -> Actor {
        // unsafe { from_glib_none(ffi::clutter_actor_new()) }
        unimplemented!()
    }
}

impl Object for Actor {}
impl Is<Actor> for Actor {}

impl AsRef<Actor> for Actor {
    fn as_ref(&self) -> &Actor {
        self
    }
}

impl Default for Actor {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait containing all `Actor` methods.
///
/// # Implementors
///
/// [`Actor`](struct.Actor.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Group`](struct.Group.html), [`Rectangle`](struct.Rectangle.html), [`ScrollActor`](struct.ScrollActor.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html)
pub trait ActorExt: 'static {
    /// Adds `action` to the list of actions applied to `self`
    ///
    /// A `Action` can only belong to one actor at a time
    ///
    /// The `Actor` will hold a reference on `action` until either
    /// `ActorExt::remove_action` or `ActorExt::clear_actions`
    /// is called
    /// ## `action`
    /// a `Action`
    fn add_action<P: Is<Action>>(&self, action: &P);

    /// A convenience function for setting the name of a `Action`
    /// while adding it to the list of actions applied to `self`
    ///
    /// This function is the logical equivalent of:
    ///
    ///
    /// ```C
    ///   actor_meta_set_name (ACTOR_META (action), name);
    ///   actor_add_action (self, action);
    /// ```
    /// ## `name`
    /// the name to set on the action
    /// ## `action`
    /// a `Action`
    fn add_action_with_name<P: Is<Action>>(&self, name: &str, action: &P);

    /// Adds `child` to the children of `self`.
    ///
    /// This function will acquire a reference on `child` that will only
    /// be released when calling `ActorExt::remove_child`.
    ///
    /// This function will take into consideration the `Actor:depth`
    /// of `child`, and will keep the list of children sorted.
    ///
    /// This function will emit the `Container::actor-added` signal
    /// on `self`.
    /// ## `child`
    /// a `Actor`
    fn add_child<P: Is<Actor>>(&self, child: &P);

    /// Adds `constraint` to the list of `Constraint`<!-- -->s applied
    /// to `self`
    ///
    /// The `Actor` will hold a reference on the `constraint` until
    /// either `ActorExt::remove_constraint` or
    /// `ActorExt::clear_constraints` is called.
    /// ## `constraint`
    /// a `Constraint`
    fn add_constraint<P: Is<Constraint>>(&self, constraint: &P);

    /// A convenience function for setting the name of a `Constraint`
    /// while adding it to the list of constraints applied to `self`
    ///
    /// This function is the logical equivalent of:
    ///
    ///
    /// ```C
    ///   actor_meta_set_name (ACTOR_META (constraint), name);
    ///   actor_add_constraint (self, constraint);
    /// ```
    /// ## `name`
    /// the name to set on the constraint
    /// ## `constraint`
    /// a `Constraint`
    fn add_constraint_with_name<P: Is<Constraint>>(&self, name: &str, constraint: &P);

    /// Adds `effect` to the list of `Effect`<!-- -->s applied to `self`
    ///
    /// The `Actor` will hold a reference on the `effect` until either
    /// `ActorExt::remove_effect` or `ActorExt::clear_effects` is
    /// called.
    ///
    /// Note that as `Effect` is initially unowned,
    /// `ActorExt::add_effect` will sink any floating reference on `effect`.
    /// ## `effect`
    /// a `Effect`
    fn add_effect<P: Is<Effect>>(&self, effect: &P);

    /// A convenience function for setting the name of a `Effect`
    /// while adding it to the list of effects applied to `self`.
    ///
    /// Note that as `Effect` is initially unowned,
    /// `ActorExt::add_effect_with_name` will sink any floating
    /// reference on `effect`.
    ///
    /// This function is the logical equivalent of:
    ///
    ///
    /// ```C
    ///   actor_meta_set_name (ACTOR_META (effect), name);
    ///   actor_add_effect (self, effect);
    /// ```
    /// ## `name`
    /// the name to set on the effect
    /// ## `effect`
    /// a `Effect`
    fn add_effect_with_name<P: Is<Effect>>(&self, name: &str, effect: &P);

    /// Adds a `transition` to the `Actor`'s list of animations.
    ///
    /// The `name` string is a per-actor unique identifier of the `transition`: only
    /// one `Transition` can be associated to the specified `name`.
    ///
    /// The `transition` will be started once added.
    ///
    /// This function will take a reference on the `transition`.
    ///
    /// This function is usually called implicitly when modifying an animatable
    /// property.
    /// ## `name`
    /// the name of the transition to add
    /// ## `transition`
    /// the `Transition` to add
    fn add_transition<P: Is<Transition>>(&self, name: &str, transition: &P);

    /// Assigns the size of a `Actor` from the given `box_`.
    ///
    /// This function should only be called on the children of an actor when
    /// overriding the `ActorClass.allocate`() virtual function.
    ///
    /// This function will adjust the stored allocation to take into account
    /// the alignment flags set in the `Actor:x-align` and
    /// `Actor:y-align` properties, as well as the margin values set in
    /// the `Actor:margin-top`, `Actor:margin-right`,
    /// `Actor:margin-bottom`, and `Actor:margin-left` properties.
    ///
    /// This function will respect the easing state of the `Actor` and
    /// interpolate between the current allocation and the new one if the
    /// easing state duration is a positive value.
    ///
    /// Actors can know from their allocation box whether they have moved
    /// with respect to their parent actor. The `flags` parameter describes
    /// additional information about the allocation, for instance whether
    /// the parent has moved with respect to the stage, for example because
    /// a grandparent's origin has moved.
    /// ## `box_`
    /// new allocation of the actor, in parent-relative coordinates
    /// ## `flags`
    /// flags that control the allocation
    fn allocate(&self, box_: &ActorBox, flags: AllocationFlags);

    /// Allocates `self` by taking into consideration the available allocation
    /// area; an alignment factor on either axis; and whether the actor should
    /// fill the allocation on either axis.
    ///
    /// The `box_` should contain the available allocation width and height;
    /// if the x1 and y1 members of `ActorBox` are not set to 0, the
    /// allocation will be offset by their value.
    ///
    /// This function takes into consideration the geometry request specified by
    /// the `Actor:request-mode` property, and the text direction.
    ///
    /// This function is useful for fluid layout managers using legacy alignment
    /// flags. Newly written layout managers should use the `Actor:x-align`
    /// and `Actor:y-align` properties, instead, and just call
    /// `ActorExt::allocate` inside their `ActorClass.allocate`()
    /// implementation.
    /// ## `box_`
    /// a `ActorBox`, containing the available width and height
    /// ## `x_align`
    /// the horizontal alignment, between 0 and 1
    /// ## `y_align`
    /// the vertical alignment, between 0 and 1
    /// ## `x_fill`
    /// whether the actor should fill horizontally
    /// ## `y_fill`
    /// whether the actor should fill vertically
    /// ## `flags`
    /// allocation flags to be passed to `ActorExt::allocate`
    fn allocate_align_fill(
        &self,
        box_: &ActorBox,
        x_align: f64,
        y_align: f64,
        x_fill: bool,
        y_fill: bool,
        flags: AllocationFlags,
    );

    /// Allocates `self` taking into account the `Actor`'s
    /// preferred size, but limiting it to the maximum available width
    /// and height provided.
    ///
    /// This function will do the right thing when dealing with the
    /// actor's request mode.
    ///
    /// The implementation of this function is equivalent to:
    ///
    ///
    /// ```C
    ///   if (request_mode == REQUEST_HEIGHT_FOR_WIDTH)
    ///     {
    ///       actor_get_preferred_width (self, available_height,
    ///                                          &min_width,
    ///                                          &natural_width);
    ///       width = CLAMP (natural_width, min_width, available_width);
    ///
    ///       actor_get_preferred_height (self, width,
    ///                                           &min_height,
    ///                                           &natural_height);
    ///       height = CLAMP (natural_height, min_height, available_height);
    ///     }
    ///   else if (request_mode == REQUEST_WIDTH_FOR_HEIGHT)
    ///     {
    ///       actor_get_preferred_height (self, available_width,
    ///                                           &min_height,
    ///                                           &natural_height);
    ///       height = CLAMP (natural_height, min_height, available_height);
    ///
    ///       actor_get_preferred_width (self, height,
    ///                                          &min_width,
    ///                                          &natural_width);
    ///       width = CLAMP (natural_width, min_width, available_width);
    ///     }
    ///   else if (request_mode == REQUEST_CONTENT_SIZE)
    ///     {
    ///       content_get_preferred_size (content, &natural_width, &natural_height);
    ///
    ///       width = CLAMP (natural_width, 0, available_width);
    ///       height = CLAMP (natural_height, 0, available_height);
    ///     }
    ///
    ///   box.x1 = x; box.y1 = y;
    ///   box.x2 = box.x1 + available_width;
    ///   box.y2 = box.y1 + available_height;
    ///   actor_allocate (self, &box, flags);
    /// ```
    ///
    /// This function can be used by fluid layout managers to allocate
    /// an actor's preferred size without making it bigger than the area
    /// available for the container.
    /// ## `x`
    /// the actor's X coordinate
    /// ## `y`
    /// the actor's Y coordinate
    /// ## `available_width`
    /// the maximum available width, or -1 to use the
    ///  actor's natural width
    /// ## `available_height`
    /// the maximum available height, or -1 to use the
    ///  actor's natural height
    /// ## `flags`
    /// flags controlling the allocation
    fn allocate_available_size(
        &self,
        x: f32,
        y: f32,
        available_width: f32,
        available_height: f32,
        flags: AllocationFlags,
    );

    /// Allocates the natural size of `self`.
    ///
    /// This function is a utility call for `Actor` implementations
    /// that allocates the actor's preferred natural size. It can be used
    /// by fixed layout managers (like `Group` or so called
    /// 'composite actors') inside the Actor::allocate
    /// implementation to give each child exactly how much space it
    /// requires, regardless of the size of the parent.
    ///
    /// This function is not meant to be used by applications. It is also
    /// not meant to be used outside the implementation of the
    /// `ActorClass.allocate` virtual function.
    /// ## `flags`
    /// flags controlling the allocation
    fn allocate_preferred_size(&self, flags: AllocationFlags);

    /// Transforms `point` in coordinates relative to the actor into
    /// ancestor-relative coordinates using the relevant transform
    /// stack (i.e. scale, rotation, etc).
    ///
    /// If `ancestor` is `None` the ancestor will be the `Stage`. In
    /// this case, the coordinates returned will be the coordinates on
    /// the stage before the projection is applied. This is different from
    /// the behaviour of `ActorExt::apply_transform_to_point`.
    /// ## `ancestor`
    /// A `Actor` ancestor, or `None` to use the
    ///  default `Stage`
    /// ## `point`
    /// A point as `Vertex`
    /// ## `vertex`
    /// The translated `Vertex`
    fn apply_relative_transform_to_point<P: Is<Actor>>(
        &self,
        ancestor: Option<&P>,
        point: &Vertex,
    ) -> Vertex;

    /// Transforms `point` in coordinates relative to the actor
    /// into screen-relative coordinates with the current actor
    /// transformation (i.e. scale, rotation, etc)
    /// ## `point`
    /// A point as `Vertex`
    /// ## `vertex`
    /// The translated `Vertex`
    fn apply_transform_to_point(&self, point: &Vertex) -> Vertex;

    // /// Binds a `gio::ListModel` to a `Actor`.
    // ///
    // /// If the `Actor` was already bound to a `gio::ListModel`, the previous
    // /// binding is destroyed.
    // ///
    // /// The existing children of `Actor` are destroyed when setting a
    // /// model, and new children are created and added, representing the contents
    // /// of the `model`. The `Actor` is updated whenever the `model` changes.
    // /// If `model` is `None`, the `Actor` is left empty.
    // ///
    // /// When a `Actor` is bound to a model, adding and removing children
    // /// directly is undefined behaviour.
    // /// ## `model`
    // /// a `gio::ListModel`
    // /// ## `create_child_func`
    // /// a function that creates `Actor` instances
    // ///  from the contents of the `model`
    // /// ## `user_data`
    // /// user data passed to `create_child_func`
    // /// ## `notify`
    // /// function called when unsetting the `model`
    // fn bind_model<P: Is<gio::ListModel>, Q: Fn(&glib::Object) -> Actor + 'static>(
    //     &self,
    //     model: Option<&P>,
    //     create_child_func: Q,
    // );

    //fn bind_model_with_properties<P: Is<gio::ListModel>>(&self, model: &P, child_type: glib::types::Type, first_model_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    /// Clears the list of actions applied to `self`
    fn clear_actions(&self);

    /// Clears the list of constraints applied to `self`
    fn clear_constraints(&self);

    /// Clears the list of effects applied to `self`
    fn clear_effects(&self);

    /// Determines if `descendant` is contained inside `self` (either as an
    /// immediate child, or as a deeper descendant). If `self` and
    /// `descendant` point to the same actor then it will also return `true`.
    /// ## `descendant`
    /// A `Actor`, possibly contained in `self`
    ///
    /// # Returns
    ///
    /// whether `descendent` is contained within `self`
    fn contains<P: Is<Actor>>(&self, descendant: &P) -> bool;

    /// Run the next stage of the paint sequence. This function should only
    /// be called within the implementation of the ‘run’ virtual of a
    /// `Effect`. It will cause the run method of the next effect to
    /// be applied, or it will paint the actual actor if the current effect
    /// is the last effect in the chain.
    fn continue_paint(&self);

    /// Creates a `pango::Context` for the given actor. The `pango::Context`
    /// is already configured using the appropriate font map, resolution
    /// and font options.
    ///
    /// See also `ActorExt::get_pango_context`.
    ///
    /// # Returns
    ///
    /// the newly created `pango::Context`.
    ///  Use `gobject::ObjectExt::unref` on the returned value to deallocate its
    ///  resources
    fn create_pango_context(&self) -> Option<pango::Context>;

    /// Creates a new `pango::Layout` from the same `pango::Context` used
    /// by the `Actor`. The `pango::Layout` is already configured
    /// with the font map, resolution and font options, and the
    /// given `text`.
    ///
    /// If you want to keep around a `pango::Layout` created by this
    /// function you will have to connect to the `Backend::font-changed`
    /// and `Backend::resolution-changed` signals, and call
    /// `pango::Layout::context_changed` in response to them.
    /// ## `text`
    /// the text to set on the `pango::Layout`, or `None`
    ///
    /// # Returns
    ///
    /// the newly created `pango::Layout`.
    ///  Use `gobject::ObjectExt::unref` when done
    fn create_pango_layout(&self, text: Option<&str>) -> Option<pango::Layout>;

    /// Destroys an actor. When an actor is destroyed, it will break any
    /// references it holds to other objects. If the actor is inside a
    /// container, the actor will be removed.
    ///
    /// When you destroy a container, its children will be destroyed as well.
    ///
    /// Note: you cannot destroy the `Stage` returned by
    /// `Stage::get_default`.
    fn destroy(&self);

    /// Destroys all children of `self`.
    ///
    /// This function releases the reference added by inserting a child
    /// actor in the list of children of `self`, and ensures that the
    /// `Actor::destroy` signal is emitted on each child of the
    /// actor.
    ///
    /// By default, `Actor` will emit the `Actor::destroy` signal
    /// when its reference count drops to 0; the default handler of the
    /// `Actor::destroy` signal will destroy all the children of an
    /// actor. This function ensures that all children are destroyed, instead
    /// of just removed from `self`, unlike `ActorExt::remove_all_children`
    /// which will merely release the reference and remove each child.
    ///
    /// Unless you acquired an additional reference on each child of `self`
    /// prior to calling `ActorExt::remove_all_children` and want to reuse
    /// the actors, you should use `ActorExt::destroy_all_children` in
    /// order to make sure that children are destroyed and signal handlers
    /// are disconnected even in cases where circular references prevent this
    /// from automatically happening through reference counting alone.
    fn destroy_all_children(&self);

    /// This function is used to emit an event on the main stage.
    /// You should rarely need to use this function, except for
    /// synthetising events.
    /// ## `event`
    /// a `Event`
    /// ## `capture`
    /// `true` if event in in capture phase, `false` otherwise.
    ///
    /// # Returns
    ///
    /// the return value from the signal emission: `true`
    ///  if the actor handled the event, or `false` if the event was
    ///  not handled
    fn event(&self, event: &Event, capture: bool) -> bool;

    //fn get_abs_allocation_vertices(&self, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4);

    /// Retrieves the `Action` with the given name in the list
    /// of actions applied to `self`
    /// ## `name`
    /// the name of the action to retrieve
    ///
    /// # Returns
    ///
    /// a `Action` for the given
    ///  name, or `None`. The returned `Action` is owned by the
    ///  actor and it should not be unreferenced directly
    fn get_action(&self, name: &str) -> Option<Action>;

    /// Retrieves the list of actions applied to `self`
    ///
    /// # Returns
    ///
    /// a copy
    ///  of the list of `Action`<!-- -->s. The contents of the list are
    ///  owned by the `Actor`. Use `glib::List::free` to free the resources
    ///  allocated by the returned `glib::List`
    fn get_actions(&self) -> Vec<Action>;

    /// Gets the layout box an actor has been assigned. The allocation can
    /// only be assumed valid inside a `paint` method; anywhere else, it
    /// may be out-of-date.
    ///
    /// An allocation does not incorporate the actor's scale or anchor point;
    /// those transformations do not affect layout, only rendering.
    ///
    /// Do not call any of the actor_get_allocation_*() family
    /// of functions inside the implementation of the `get_preferred_width`
    /// or `get_preferred_height` virtual functions.
    /// ## `box_`
    /// the function fills this in with the actor's allocation
    fn get_allocation_box(&self) -> ActorBox;

    //fn get_allocation_vertices<P: Is<Actor>>(&self, ancestor: Option<&P>, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4);

    /// Retrieves the color set using `ActorExt::set_background_color`.
    /// ## `color`
    /// return location for a `Color`
    fn get_background_color(&self) -> Color;

    /// Retrieves the actor at the given `index_` inside the list of
    /// children of `self`.
    /// ## `index_`
    /// the position in the list of children
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor`, or `None`
    fn get_child_at_index(&self, index_: i32) -> Option<Actor>;

    /// Retrieves the child transformation matrix set using
    /// `ActorExt::set_child_transform`; if none is currently set,
    /// the `transform` matrix will be initialized to the identity matrix.
    /// ## `transform`
    /// a `Matrix`
    fn get_child_transform(&self) -> Matrix;

    /// Retrieves the list of children of `self`.
    ///
    /// # Returns
    ///
    /// A newly
    ///  allocated `glib::List` of `Actor`<!-- -->s. Use `glib::List::free` when
    ///  done.
    fn get_children(&self) -> Vec<Actor>;

    /// Gets the clip area for `self`, if any is set.
    /// ## `xoff`
    /// return location for the X offset of
    ///  the clip rectangle, or `None`
    /// ## `yoff`
    /// return location for the Y offset of
    ///  the clip rectangle, or `None`
    /// ## `width`
    /// return location for the width of
    ///  the clip rectangle, or `None`
    /// ## `height`
    /// return location for the height of
    ///  the clip rectangle, or `None`
    fn get_clip(&self) -> (f32, f32, f32, f32);

    /// Retrieves the value set using `ActorExt::set_clip_to_allocation`
    ///
    /// # Returns
    ///
    /// `true` if the `Actor` is clipped to its allocation
    fn get_clip_to_allocation(&self) -> bool;

    /// Retrieves the `Constraint` with the given name in the list
    /// of constraints applied to `self`
    /// ## `name`
    /// the name of the constraint to retrieve
    ///
    /// # Returns
    ///
    /// a `Constraint` for the given
    ///  name, or `None`. The returned `Constraint` is owned by the
    ///  actor and it should not be unreferenced directly
    fn get_constraint(&self, name: &str) -> Option<Constraint>;

    /// Retrieves the list of constraints applied to `self`
    ///
    /// # Returns
    ///
    /// a copy
    ///  of the list of `Constraint`<!-- -->s. The contents of the list are
    ///  owned by the `Actor`. Use `glib::List::free` to free the resources
    ///  allocated by the returned `glib::List`
    fn get_constraints(&self) -> Vec<Constraint>;

    /// Retrieves the contents of `self`.
    ///
    /// # Returns
    ///
    /// a pointer to the `Content` instance,
    ///  or `None` if none was set
    fn get_content(&self) -> Option<Content>;

    /// Retrieves the bounding box for the `Content` of `self`.
    ///
    /// The bounding box is relative to the actor's allocation.
    ///
    /// If no `Content` is set for `self`, or if `self` has not been
    /// allocated yet, then the result is undefined.
    ///
    /// The content box is guaranteed to be, at most, as big as the allocation
    /// of the `Actor`.
    ///
    /// If the `Content` used by the actor has a preferred size, then
    /// it is possible to modify the content box by using the
    /// `Actor:content-gravity` property.
    /// ## `box_`
    /// the return location for the bounding
    ///  box for the `Content`
    fn get_content_box(&self) -> ActorBox;

    /// Retrieves the content gravity as set using
    /// `ActorExt::set_content_gravity`.
    ///
    /// # Returns
    ///
    /// the content gravity
    fn get_content_gravity(&self) -> ContentGravity;

    /// Retrieves the repeat policy for a `Actor` set by
    /// `ActorExt::set_content_repeat`.
    ///
    /// # Returns
    ///
    /// the content repeat policy
    fn get_content_repeat(&self) -> ContentRepeat;

    /// Retrieves the values set using `ActorExt::set_content_scaling_filters`.
    /// ## `min_filter`
    /// return location for the minification
    ///  filter, or `None`
    /// ## `mag_filter`
    /// return location for the magnification
    ///  filter, or `None`
    fn get_content_scaling_filters(&self) -> (ScalingFilter, ScalingFilter);

    /// Retrieves the default paint volume for `self`.
    ///
    /// This function provides the same `PaintVolume` that would be
    /// computed by the default implementation inside `Actor` of the
    /// `ActorClass.get_paint_volume`() virtual function.
    ///
    /// This function should only be used by `Actor` subclasses that
    /// cannot chain up to the parent implementation when computing their
    /// paint volume.
    ///
    /// # Returns
    ///
    /// a pointer to the default
    ///  `PaintVolume`, relative to the `Actor`, or `None` if
    ///  the actor could not compute a valid paint volume. The returned value
    ///  is not guaranteed to be stable across multiple frames, so if you
    ///  want to retain it, you will need to copy it using
    ///  `PaintVolume::copy`.
    fn get_default_paint_volume(&self) -> Option<PaintVolume>;

    /// Retrieves the delay that should be applied when tweening animatable
    /// properties.
    ///
    /// # Returns
    ///
    /// a delay, in milliseconds
    fn get_easing_delay(&self) -> u32;

    /// Retrieves the duration of the tweening for animatable
    /// properties of `self` for the current easing state.
    ///
    /// # Returns
    ///
    /// the duration of the tweening, in milliseconds
    fn get_easing_duration(&self) -> u32;

    /// Retrieves the easing mode for the tweening of animatable properties
    /// of `self` for the current easing state.
    ///
    /// # Returns
    ///
    /// an easing mode
    fn get_easing_mode(&self) -> AnimationMode;

    /// Retrieves the `Effect` with the given name in the list
    /// of effects applied to `self`
    /// ## `name`
    /// the name of the effect to retrieve
    ///
    /// # Returns
    ///
    /// a `Effect` for the given
    ///  name, or `None`. The returned `Effect` is owned by the
    ///  actor and it should not be unreferenced directly
    fn get_effect(&self, name: &str) -> Option<Effect>;

    /// Retrieves the `Effect`<!-- -->s applied on `self`, if any
    ///
    /// # Returns
    ///
    /// a list
    ///  of `Effect`<!-- -->s, or `None`. The elements of the returned
    ///  list are owned by internals and they should not be freed. You should
    ///  free the returned list using `glib::List::free` when done
    fn get_effects(&self) -> Vec<Effect>;

    /// Retrieves the first child of `self`.
    ///
    /// The returned pointer is only valid until the scene graph changes; it
    /// is not safe to modify the list of children of `self` while iterating
    /// it.
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor`, or `None`
    fn get_first_child(&self) -> Option<Actor>;

    /// Checks whether an actor has a fixed position set (and will thus be
    /// unaffected by any layout manager).
    ///
    /// # Returns
    ///
    /// `true` if the fixed position is set on the actor
    fn get_fixed_position_set(&self) -> bool;

    /// Retrieves the flags set on `self`
    ///
    /// # Returns
    ///
    /// a bitwise or of `ActorFlags` or 0
    fn get_flags(&self) -> ActorFlags;

    /// Retrieves the height of a `Actor`.
    ///
    /// If the actor has a valid allocation, this function will return the
    /// height of the allocated area given to the actor.
    ///
    /// If the actor does not have a valid allocation, this function will
    /// return the actor's natural height, that is the preferred height of
    /// the actor.
    ///
    /// If you care whether you get the preferred height or the height that
    /// has been assigned to the actor, you should probably call a different
    /// function like `ActorExt::get_allocation_box` to retrieve the
    /// allocated size or `ActorExt::get_preferred_height` to retrieve the
    /// preferred height.
    ///
    /// If an actor has a fixed height, for instance a height that has been
    /// assigned using `ActorExt::set_height`, the height returned will
    /// be the same value.
    ///
    /// # Returns
    ///
    /// the height of the actor, in pixels
    fn get_height(&self) -> f32;

    /// Retrieves the last child of `self`.
    ///
    /// The returned pointer is only valid until the scene graph changes; it
    /// is not safe to modify the list of children of `self` while iterating
    /// it.
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor`, or `None`
    fn get_last_child(&self) -> Option<Actor>;

    /// Retrieves the `LayoutManager` used by `self`.
    ///
    /// # Returns
    ///
    /// a pointer to the `LayoutManager`,
    ///  or `None`
    fn get_layout_manager(&self) -> Option<LayoutManager>;

    /// Retrieves all the components of the margin of a `Actor`.
    /// ## `margin`
    /// return location for a `Margin`
    fn get_margin(&self) -> Margin;

    /// Retrieves the bottom margin of a `Actor`.
    ///
    /// # Returns
    ///
    /// the bottom margin
    fn get_margin_bottom(&self) -> f32;

    /// Retrieves the left margin of a `Actor`.
    ///
    /// # Returns
    ///
    /// the left margin
    fn get_margin_left(&self) -> f32;

    /// Retrieves the right margin of a `Actor`.
    ///
    /// # Returns
    ///
    /// the right margin
    fn get_margin_right(&self) -> f32;

    /// Retrieves the top margin of a `Actor`.
    ///
    /// # Returns
    ///
    /// the top margin
    fn get_margin_top(&self) -> f32;

    /// Retrieves the number of children of `self`.
    ///
    /// # Returns
    ///
    /// the number of children of an actor
    fn get_n_children(&self) -> i32;

    /// Retrieves the name of `self`.
    ///
    /// # Returns
    ///
    /// the name of the actor, or `None`. The returned string is
    ///  owned by the actor and should not be modified or freed.
    fn get_name(&self) -> Option<String>;

    /// Retrieves the sibling of `self` that comes after it in the list
    /// of children of `self`'s parent.
    ///
    /// The returned pointer is only valid until the scene graph changes; it
    /// is not safe to modify the list of children of `self` while iterating
    /// it.
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor`, or `None`
    fn get_next_sibling(&self) -> Option<Actor>;

    /// Retrieves whether to redirect the actor to an offscreen buffer, as
    /// set by `ActorExt::set_offscreen_redirect`.
    ///
    /// # Returns
    ///
    /// the value of the offscreen-redirect property of the actor
    fn get_offscreen_redirect(&self) -> OffscreenRedirect;

    /// Retrieves the opacity value of an actor, as set by
    /// `ActorExt::set_opacity`.
    ///
    /// For retrieving the absolute opacity of the actor inside a paint
    /// virtual function, see `ActorExt::get_paint_opacity`.
    ///
    /// # Returns
    ///
    /// the opacity of the actor
    fn get_opacity(&self) -> u8;

    /// Retrieves the paint volume of the passed `Actor`, and
    /// transforms it into a 2D bounding box in stage coordinates.
    ///
    /// This function is useful to determine the on screen area occupied by
    /// the actor. The box is only an approximation and may often be
    /// considerably larger due to the optimizations used to calculate the
    /// box. The box is never smaller though, so it can reliably be used
    /// for culling.
    ///
    /// There are times when a 2D paint box can't be determined, e.g.
    /// because the actor isn't yet parented under a stage or because
    /// the actor is unable to determine a paint volume.
    /// ## `box_`
    /// return location for a `ActorBox`
    ///
    /// # Returns
    ///
    /// `true` if a 2D paint box could be determined, else
    /// `false`.
    fn get_paint_box(&self) -> Option<ActorBox>;

    /// Retrieves the absolute opacity of the actor, as it appears on the stage.
    ///
    /// This function traverses the hierarchy chain and composites the opacity of
    /// the actor with that of its parents.
    ///
    /// This function is intended for subclasses to use in the paint virtual
    /// function, to paint themselves with the correct opacity.
    ///
    /// # Returns
    ///
    /// The actor opacity value.
    fn get_paint_opacity(&self) -> u8;

    /// Retrieves the 'paint' visibility of an actor recursively checking for non
    /// visible parents.
    ///
    /// This is by definition the same as `ACTOR_IS_MAPPED`.
    ///
    /// # Returns
    ///
    /// `true` if the actor is visibile and will be painted.
    fn get_paint_visibility(&self) -> bool;

    /// Retrieves the paint volume of the passed `Actor`, or `None`
    /// when a paint volume can't be determined.
    ///
    /// The paint volume is defined as the 3D space occupied by an actor
    /// when being painted.
    ///
    /// This function will call the `ActorClass.get_paint_volume`()
    /// virtual function of the `Actor` class. Sub-classes of `Actor`
    /// should not usually care about overriding the default implementation,
    /// unless they are, for instance: painting outside their allocation, or
    /// actors with a depth factor (not in terms of `Actor:depth` but real
    /// 3D depth).
    ///
    /// Note: 2D actors overriding `ActorClass.get_paint_volume`()
    /// should ensure that their volume has a depth of 0. (This will be true
    /// as long as you don't call `PaintVolume::set_depth`.)
    ///
    /// # Returns
    ///
    /// a pointer to a `PaintVolume`,
    ///  or `None` if no volume could be determined. The returned pointer
    ///  is not guaranteed to be valid across multiple frames; if you want
    ///  to keep it, you will need to copy it using `PaintVolume::copy`.
    fn get_paint_volume(&self) -> Option<PaintVolume>;

    /// Retrieves the `pango::Context` for `self`. The actor's `pango::Context`
    /// is already configured using the appropriate font map, resolution
    /// and font options.
    ///
    /// Unlike `ActorExt::create_pango_context`, this context is owend
    /// by the `Actor` and it will be updated each time the options
    /// stored by the `Backend` change.
    ///
    /// You can use the returned `pango::Context` to create a `pango::Layout`
    /// and render text using `pango_render_layout` to reuse the
    /// glyphs cache also used by internal.
    ///
    /// # Returns
    ///
    /// the `pango::Context` for a `Actor`.
    ///  The returned `pango::Context` is owned by the actor and should not be
    ///  unreferenced by the application code
    fn get_pango_context(&self) -> Option<pango::Context>;

    /// Retrieves the parent of `self`.
    ///
    /// # Returns
    ///
    /// The `Actor` parent, or `None`
    ///  if no parent is set
    fn get_parent(&self) -> Option<Actor>;

    /// Retrieves the coordinates of the `Actor:pivot-point`.
    /// ## `pivot_x`
    /// return location for the normalized X
    ///  coordinate of the pivot point, or `None`
    /// ## `pivot_y`
    /// return location for the normalized Y
    ///  coordinate of the pivot point, or `None`
    fn get_pivot_point(&self) -> (f32, f32);

    /// Retrieves the Z component of the `Actor:pivot-point`.
    fn get_pivot_point_z(&self) -> f32;

    /// This function tries to "do what you mean" and tell you where the
    /// actor is, prior to any transformations. Retrieves the fixed
    /// position of an actor in pixels, if one has been set; otherwise, if
    /// the allocation is valid, returns the actor's allocated position;
    /// otherwise, returns 0,0.
    ///
    /// The returned position is in pixels.
    /// ## `x`
    /// return location for the X coordinate, or `None`
    /// ## `y`
    /// return location for the Y coordinate, or `None`
    fn get_position(&self) -> (f32, f32);

    /// Computes the requested minimum and natural heights for an actor,
    /// or if they are already computed, returns the cached values.
    ///
    /// An actor may not get its request - depending on the layout
    /// manager that's in effect.
    ///
    /// A request should not incorporate the actor's scale or anchor point;
    /// those transformations do not affect layout, only rendering.
    /// ## `for_width`
    /// available width to assume in computing desired height,
    ///  or a negative value to indicate that no width is defined
    /// ## `min_height_p`
    /// return location for minimum height,
    ///  or `None`
    /// ## `natural_height_p`
    /// return location for natural
    ///  height, or `None`
    fn get_preferred_height(&self, for_width: f32) -> (f32, f32);

    /// Computes the preferred minimum and natural size of an actor, taking into
    /// account the actor's geometry management (either height-for-width
    /// or width-for-height).
    ///
    /// The width and height used to compute the preferred height and preferred
    /// width are the actor's natural ones.
    ///
    /// If you need to control the height for the preferred width, or the width for
    /// the preferred height, you should use `ActorExt::get_preferred_width`
    /// and `ActorExt::get_preferred_height`, and check the actor's preferred
    /// geometry management using the `Actor:request-mode` property.
    /// ## `min_width_p`
    /// return location for the minimum
    ///  width, or `None`
    /// ## `min_height_p`
    /// return location for the minimum
    ///  height, or `None`
    /// ## `natural_width_p`
    /// return location for the natural
    ///  width, or `None`
    /// ## `natural_height_p`
    /// return location for the natural
    ///  height, or `None`
    fn get_preferred_size(&self) -> (f32, f32, f32, f32);

    /// Computes the requested minimum and natural widths for an actor,
    /// optionally depending on the specified height, or if they are
    /// already computed, returns the cached values.
    ///
    /// An actor may not get its request - depending on the layout
    /// manager that's in effect.
    ///
    /// A request should not incorporate the actor's scale or anchor point;
    /// those transformations do not affect layout, only rendering.
    /// ## `for_height`
    /// available height when computing the preferred width,
    ///  or a negative value to indicate that no height is defined
    /// ## `min_width_p`
    /// return location for minimum width,
    ///  or `None`
    /// ## `natural_width_p`
    /// return location for the natural
    ///  width, or `None`
    fn get_preferred_width(&self, for_height: f32) -> (f32, f32);

    /// Retrieves the sibling of `self` that comes before it in the list
    /// of children of `self`'s parent.
    ///
    /// The returned pointer is only valid until the scene graph changes; it
    /// is not safe to modify the list of children of `self` while iterating
    /// it.
    ///
    /// # Returns
    ///
    /// a pointer to a `Actor`, or `None`
    fn get_previous_sibling(&self) -> Option<Actor>;

    /// Checks whether `self` is marked as reactive.
    ///
    /// # Returns
    ///
    /// `true` if the actor is reactive
    fn get_reactive(&self) -> bool;

    /// Retrieves the geometry request mode of `self`
    ///
    /// # Returns
    ///
    /// the request mode for the actor
    fn get_request_mode(&self) -> RequestMode;

    /// Retrieves the angle of rotation set by `ActorExt::set_rotation_angle`.
    /// ## `axis`
    /// the axis of the rotation
    ///
    /// # Returns
    ///
    /// the angle of rotation, in degrees
    fn get_rotation_angle(&self, axis: RotateAxis) -> f64;

    /// Retrieves an actors scale factors.
    /// ## `scale_x`
    /// Location to store horizonal
    ///  scale factor, or `None`.
    /// ## `scale_y`
    /// Location to store vertical
    ///  scale factor, or `None`.
    fn get_scale(&self) -> (f64, f64);

    /// Retrieves the scaling factor along the Z axis, as set using
    /// `ActorExt::set_scale_z`.
    ///
    /// # Returns
    ///
    /// the scaling factor along the Z axis
    fn get_scale_z(&self) -> f64;

    /// This function tries to "do what you mean" and return
    /// the size an actor will have. If the actor has a valid
    /// allocation, the allocation will be returned; otherwise,
    /// the actors natural size request will be returned.
    ///
    /// If you care whether you get the request vs. the allocation, you
    /// should probably call a different function like
    /// `ActorExt::get_allocation_box` or
    /// `ActorExt::get_preferred_width`.
    /// ## `width`
    /// return location for the width, or `None`.
    /// ## `height`
    /// return location for the height, or `None`.
    fn get_size(&self) -> (f32, f32);

    /// Retrieves the `Stage` where `self` is contained.
    ///
    /// # Returns
    ///
    /// the stage
    ///  containing the actor, or `None`
    fn get_stage(&self) -> Option<Stage>;

    /// Retrieves the value set using `ActorExt::set_text_direction`
    ///
    /// If no text direction has been previously set, the default text
    /// direction, as returned by `get_default_text_direction`, will
    /// be returned instead
    ///
    /// # Returns
    ///
    /// the `TextDirection` for the actor
    fn get_text_direction(&self) -> TextDirection;

    /// Retrieves the current transformation matrix of a `Actor`.
    /// ## `transform`
    /// a `Matrix`
    fn get_transform(&self) -> Matrix;

    /// Retrieves the 3D paint volume of an actor like
    /// `ActorExt::get_paint_volume` does (Please refer to the
    /// documentation of `ActorExt::get_paint_volume` for more
    /// details.) and it additionally transforms the paint volume into the
    /// coordinate space of `relative_to_ancestor`. (Or the stage if `None`
    /// is passed for `relative_to_ancestor`)
    ///
    /// This can be used by containers that base their paint volume on
    /// the volume of their children. Such containers can query the
    /// transformed paint volume of all of its children and union them
    /// together using `PaintVolume::union`.
    /// ## `relative_to_ancestor`
    /// A `Actor` that is an ancestor of `self`
    ///  (or `None` for the stage)
    ///
    /// # Returns
    ///
    /// a pointer to a `PaintVolume`,
    ///  or `None` if no volume could be determined. The returned pointer is
    ///  not guaranteed to be valid across multiple frames; if you wish to
    ///  keep it, you will have to copy it using `PaintVolume::copy`.
    fn get_transformed_paint_volume<P: Is<Actor>>(
        &self,
        relative_to_ancestor: &P,
    ) -> Option<PaintVolume>;

    /// Gets the absolute position of an actor, in pixels relative to the stage.
    /// ## `x`
    /// return location for the X coordinate, or `None`
    /// ## `y`
    /// return location for the Y coordinate, or `None`
    fn get_transformed_position(&self) -> (f32, f32);

    /// Gets the absolute size of an actor in pixels, taking into account the
    /// scaling factors.
    ///
    /// If the actor has a valid allocation, the allocated size will be used.
    /// If the actor has not a valid allocation then the preferred size will
    /// be transformed and returned.
    ///
    /// If you want the transformed allocation, see
    /// `ActorExt::get_abs_allocation_vertices` instead.
    ///
    /// When the actor (or one of its ancestors) is rotated around the
    /// X or Y axis, it no longer appears as on the stage as a rectangle, but
    /// as a generic quadrangle; in that case this function returns the size
    /// of the smallest rectangle that encapsulates the entire quad. Please
    /// note that in this case no assumptions can be made about the relative
    /// position of this envelope to the absolute position of the actor, as
    /// returned by `ActorExt::get_transformed_position`; if you need this
    /// information, you need to use `ActorExt::get_abs_allocation_vertices`
    /// to get the coords of the actual quadrangle.
    /// ## `width`
    /// return location for the width, or `None`
    /// ## `height`
    /// return location for the height, or `None`
    fn get_transformed_size(&self) -> (f32, f32);

    /// Retrieves the `Transition` of a `Actor` by using the
    /// transition `name`.
    ///
    /// Transitions created for animatable properties use the name of the
    /// property itself, for instance the code below:
    ///
    ///
    /// ```C
    ///   actor_set_easing_duration (actor, 1000);
    ///   actor_set_rotation (actor, Y_AXIS, 360.0, x, y, z);
    ///
    ///   transition = actor_get_transition (actor, "rotation-angle-y");
    ///   g_signal_connect (transition, "stopped",
    ///                     G_CALLBACK (on_transition_stopped),
    ///                     actor);
    /// ```
    ///
    /// will call the `on_transition_stopped` callback when the transition
    /// is finished.
    ///
    /// If you just want to get notifications of the completion of a transition,
    /// you should use the `Actor::transition-stopped` signal, using the
    /// transition name as the signal detail.
    /// ## `name`
    /// the name of the transition
    ///
    /// # Returns
    ///
    /// a `Transition`, or `None` is none
    ///  was found to match the passed name; the returned instance is owned
    ///  by internal and it should not be freed
    fn get_transition(&self, name: &str) -> Option<Transition>;

    /// Retrieves the translation set using `ActorExt::set_translation`.
    /// ## `translate_x`
    /// return location for the X component
    ///  of the translation, or `None`
    /// ## `translate_y`
    /// return location for the Y component
    ///  of the translation, or `None`
    /// ## `translate_z`
    /// return location for the Z component
    ///  of the translation, or `None`
    fn get_translation(&self) -> (f32, f32, f32);

    /// Retrieves the width of a `Actor`.
    ///
    /// If the actor has a valid allocation, this function will return the
    /// width of the allocated area given to the actor.
    ///
    /// If the actor does not have a valid allocation, this function will
    /// return the actor's natural width, that is the preferred width of
    /// the actor.
    ///
    /// If you care whether you get the preferred width or the width that
    /// has been assigned to the actor, you should probably call a different
    /// function like `ActorExt::get_allocation_box` to retrieve the
    /// allocated size or `ActorExt::get_preferred_width` to retrieve the
    /// preferred width.
    ///
    /// If an actor has a fixed width, for instance a width that has been
    /// assigned using `ActorExt::set_width`, the width returned will
    /// be the same value.
    ///
    /// # Returns
    ///
    /// the width of the actor, in pixels
    fn get_width(&self) -> f32;

    /// Retrieves the X coordinate of a `Actor`.
    ///
    /// This function tries to "do what you mean", by returning the
    /// correct value depending on the actor's state.
    ///
    /// If the actor has a valid allocation, this function will return
    /// the X coordinate of the origin of the allocation box.
    ///
    /// If the actor has any fixed coordinate set using `ActorExt::set_x`,
    /// `ActorExt::set_position` or `Actor::set_geometry`, this
    /// function will return that coordinate.
    ///
    /// If both the allocation and a fixed position are missing, this function
    /// will return 0.
    ///
    /// # Returns
    ///
    /// the X coordinate, in pixels, ignoring any
    ///  transformation (i.e. scaling, rotation)
    fn get_x(&self) -> f32;

    /// Retrieves the horizontal alignment policy set using
    /// `ActorExt::set_x_align`.
    ///
    /// # Returns
    ///
    /// the horizontal alignment policy.
    fn get_x_align(&self) -> ActorAlign;

    /// Retrieves the value set with `ActorExt::set_x_expand`.
    ///
    /// See also: `ActorExt::needs_expand`
    ///
    /// # Returns
    ///
    /// `true` if the actor has been set to expand
    fn get_x_expand(&self) -> bool;

    /// Retrieves the Y coordinate of a `Actor`.
    ///
    /// This function tries to "do what you mean", by returning the
    /// correct value depending on the actor's state.
    ///
    /// If the actor has a valid allocation, this function will return
    /// the Y coordinate of the origin of the allocation box.
    ///
    /// If the actor has any fixed coordinate set using `ActorExt::set_y`,
    /// `ActorExt::set_position` or `Actor::set_geometry`, this
    /// function will return that coordinate.
    ///
    /// If both the allocation and a fixed position are missing, this function
    /// will return 0.
    ///
    /// # Returns
    ///
    /// the Y coordinate, in pixels, ignoring any
    ///  transformation (i.e. scaling, rotation)
    fn get_y(&self) -> f32;

    /// Retrieves the vertical alignment policy set using
    /// `ActorExt::set_y_align`.
    ///
    /// # Returns
    ///
    /// the vertical alignment policy.
    fn get_y_align(&self) -> ActorAlign;

    /// Retrieves the value set with `ActorExt::set_y_expand`.
    ///
    /// See also: `ActorExt::needs_expand`
    ///
    /// # Returns
    ///
    /// `true` if the actor has been set to expand
    fn get_y_expand(&self) -> bool;

    /// Retrieves the actor's position on the Z axis.
    ///
    /// # Returns
    ///
    /// the position on the Z axis.
    fn get_z_position(&self) -> f32;

    /// Sets the key focus of the `Stage` including `self`
    /// to this `Actor`.
    fn grab_key_focus(&self);

    /// Returns whether the actor has any actions applied.
    ///
    /// # Returns
    ///
    /// `true` if the actor has any actions,
    ///  `false` otherwise
    fn has_actions(&self) -> bool;

    /// Checks if the actor has an up-to-date allocation assigned to
    /// it. This means that the actor should have an allocation: it's
    /// visible and has a parent. It also means that there is no
    /// outstanding relayout request in progress for the actor or its
    /// children (There might be other outstanding layout requests in
    /// progress that will cause the actor to get a new allocation
    /// when the stage is laid out, however).
    ///
    /// If this function returns `false`, then the actor will normally
    /// be allocated before it is next drawn on the screen.
    ///
    /// # Returns
    ///
    /// `true` if the actor has an up-to-date allocation
    fn has_allocation(&self) -> bool;

    /// Determines whether the actor has a clip area set or not.
    ///
    /// # Returns
    ///
    /// `true` if the actor has a clip area set.
    fn has_clip(&self) -> bool;

    /// Returns whether the actor has any constraints applied.
    ///
    /// # Returns
    ///
    /// `true` if the actor has any constraints,
    ///  `false` otherwise
    fn has_constraints(&self) -> bool;

    /// Returns whether the actor has any effects applied.
    ///
    /// # Returns
    ///
    /// `true` if the actor has any effects,
    ///  `false` otherwise
    fn has_effects(&self) -> bool;

    /// Checks whether `self` is the `Actor` that has key focus
    ///
    /// # Returns
    ///
    /// `true` if the actor has key focus, and `false` otherwise
    fn has_key_focus(&self) -> bool;

    /// Asks the actor's implementation whether it may contain overlapping
    /// primitives.
    ///
    /// For example; it may use this to determine whether the painting
    /// should be redirected to an offscreen buffer to correctly implement
    /// the opacity property.
    ///
    /// Custom actors can override the default response by implementing the
    /// `ActorClass.has_overlaps`() virtual function. See
    /// `ActorExt::set_offscreen_redirect` for more information.
    ///
    /// # Returns
    ///
    /// `true` if the actor may have overlapping primitives, and
    ///  `false` otherwise
    fn has_overlaps(&self) -> bool;

    /// Checks whether an actor contains the pointer of a
    /// `InputDevice`
    ///
    /// # Returns
    ///
    /// `true` if the actor contains the pointer, and
    ///  `false` otherwise
    fn has_pointer(&self) -> bool;

    /// Flags an actor to be hidden. A hidden actor will not be
    /// rendered on the stage.
    ///
    /// Actors are visible by default.
    ///
    /// If this function is called on an actor without a parent, the
    /// `Actor:show-on-set-parent` property will be set to `false`
    /// as a side-effect.
    fn hide(&self);

    /// Inserts `child` into the list of children of `self`, above another
    /// child of `self` or, if `sibling` is `None`, above all the children
    /// of `self`.
    ///
    /// This function will acquire a reference on `child` that will only
    /// be released when calling `ActorExt::remove_child`.
    ///
    /// This function will not take into consideration the `Actor:depth`
    /// of `child`.
    ///
    /// This function will emit the `Container::actor-added` signal
    /// on `self`.
    /// ## `child`
    /// a `Actor`
    /// ## `sibling`
    /// a child of `self`, or `None`
    fn insert_child_above<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>);

    /// Inserts `child` into the list of children of `self`, using the
    /// given `index_`. If `index_` is greater than the number of children
    /// in `self`, or is less than 0, then the new child is added at the end.
    ///
    /// This function will acquire a reference on `child` that will only
    /// be released when calling `ActorExt::remove_child`.
    ///
    /// This function will not take into consideration the `Actor:depth`
    /// of `child`.
    ///
    /// This function will emit the `Container::actor-added` signal
    /// on `self`.
    /// ## `child`
    /// a `Actor`
    /// ## `index_`
    /// the index
    fn insert_child_at_index<P: Is<Actor>>(&self, child: &P, index_: i32);

    /// Inserts `child` into the list of children of `self`, below another
    /// child of `self` or, if `sibling` is `None`, below all the children
    /// of `self`.
    ///
    /// This function will acquire a reference on `child` that will only
    /// be released when calling `ActorExt::remove_child`.
    ///
    /// This function will not take into consideration the `Actor:depth`
    /// of `child`.
    ///
    /// This function will emit the `Container::actor-added` signal
    /// on `self`.
    /// ## `child`
    /// a `Actor`
    /// ## `sibling`
    /// a child of `self`, or `None`
    fn insert_child_below<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>);

    /// Checks whether `self` is being currently painted by a `Clone`
    ///
    /// This function is useful only inside the ::paint virtual function
    /// implementations or within handlers for the `Actor::paint`
    /// signal
    ///
    /// This function should not be used by applications
    ///
    /// # Returns
    ///
    /// `true` if the `Actor` is currently being painted
    ///  by a `Clone`, and `false` otherwise
    fn is_in_clone_paint(&self) -> bool;

    /// Checks whether a `Actor` has been set as mapped.
    ///
    /// See also `ACTOR_IS_MAPPED` and `Actor:mapped`
    ///
    /// # Returns
    ///
    /// `true` if the actor is mapped
    fn is_mapped(&self) -> bool;

    /// Checks whether a `Actor` is realized.
    ///
    /// See also `ACTOR_IS_REALIZED` and `Actor:realized`.
    ///
    /// # Returns
    ///
    /// `true` if the actor is realized
    fn is_realized(&self) -> bool;

    /// Checks whether any rotation is applied to the actor.
    ///
    /// # Returns
    ///
    /// `true` if the actor is rotated.
    fn is_rotated(&self) -> bool;

    /// Checks whether the actor is scaled in either dimension.
    ///
    /// # Returns
    ///
    /// `true` if the actor is scaled.
    fn is_scaled(&self) -> bool;

    /// Checks whether an actor is marked as visible.
    ///
    /// See also `ACTOR_IS_VISIBLE` and `Actor:visible`.
    ///
    /// # Returns
    ///
    /// `true` if the actor visible
    fn is_visible(&self) -> bool;

    /// Sets the `ActorFlags::Mapped` flag on the actor and possibly maps
    /// and realizes its children if they are visible. Does nothing if the
    /// actor is not visible.
    ///
    /// Calling this function is strongly disencouraged: the default
    /// implementation of `ActorClass.map`() will map all the children
    /// of an actor when mapping its parent.
    ///
    /// When overriding map, it is mandatory to chain up to the parent
    /// implementation.
    fn map(&self);

    /// Moves an actor by the specified distance relative to its current
    /// position in pixels.
    ///
    /// This function modifies the fixed position of an actor and thus removes
    /// it from any layout management. Another way to move an actor is with an
    /// anchor point, see `Actor::set_anchor_point`, or with an additional
    /// translation, using `ActorExt::set_translation`.
    /// ## `dx`
    /// Distance to move Actor on X axis.
    /// ## `dy`
    /// Distance to move Actor on Y axis.
    fn move_by(&self, dx: f32, dy: f32);

    /// Checks whether an actor, or any of its children, is set to expand
    /// horizontally or vertically.
    ///
    /// This function should only be called by layout managers that can
    /// assign extra space to their children.
    ///
    /// If you want to know whether the actor was explicitly set to expand,
    /// use `ActorExt::get_x_expand` or `ActorExt::get_y_expand`.
    /// ## `orientation`
    /// the direction of expansion
    ///
    /// # Returns
    ///
    /// `true` if the actor should expand
    fn needs_expand(&self, orientation: Orientation) -> bool;

    /// Renders the actor to display.
    ///
    /// This function should not be called directly by applications.
    /// Call `ActorExt::queue_redraw` to queue paints, instead.
    ///
    /// This function is context-aware, and will either cause a
    /// regular paint or a pick paint.
    ///
    /// This function will emit the `Actor::paint` signal or
    /// the `Actor::pick` signal, depending on the context.
    ///
    /// This function does not paint the actor if the actor is set to 0,
    /// unless it is performing a pick paint.
    fn paint(&self);

    /// Queues up a redraw of an actor and any children. The redraw occurs
    /// once the main loop becomes idle (after the current batch of events
    /// has been processed, roughly).
    ///
    /// Applications rarely need to call this, as redraws are handled
    /// automatically by modification functions.
    ///
    /// This function will not do anything if `self` is not visible, or
    /// if the actor is inside an invisible part of the scenegraph.
    ///
    /// Also be aware that painting is a NOP for actors with an opacity of
    /// 0
    ///
    /// When you are implementing a custom actor you must queue a redraw
    /// whenever some private state changes that will affect painting or
    /// picking of your actor.
    fn queue_redraw(&self);

    /// Queues a redraw on `self` limited to a specific, actor-relative
    /// rectangular area.
    ///
    /// If `clip` is `None` this function is equivalent to
    /// `ActorExt::queue_redraw`.
    /// ## `clip`
    /// a rectangular clip region, or `None`
    fn queue_redraw_with_clip(&self, clip: Option<&cairo::RectangleInt>);

    /// Indicates that the actor's size request or other layout-affecting
    /// properties may have changed. This function is used inside `Actor`
    /// subclass implementations, not by applications directly.
    ///
    /// Queueing a new layout automatically queues a redraw as well.
    fn queue_relayout(&self);

    /// Removes `action` from the list of actions applied to `self`
    ///
    /// The reference held by `self` on the `Action` will be released
    /// ## `action`
    /// a `Action`
    fn remove_action<P: Is<Action>>(&self, action: &P);

    /// Removes the `Action` with the given name from the list
    /// of actions applied to `self`
    /// ## `name`
    /// the name of the action to remove
    fn remove_action_by_name(&self, name: &str);

    /// Removes all children of `self`.
    ///
    /// This function releases the reference added by inserting a child actor
    /// in the list of children of `self`.
    ///
    /// If the reference count of a child drops to zero, the child will be
    /// destroyed. If you want to ensure the destruction of all the children
    /// of `self`, use `ActorExt::destroy_all_children`.
    fn remove_all_children(&self);

    /// Removes all transitions associated to `self`.
    fn remove_all_transitions(&self);

    /// Removes `child` from the children of `self`.
    ///
    /// This function will release the reference added by
    /// `ActorExt::add_child`, so if you want to keep using `child`
    /// you will have to acquire a referenced on it before calling this
    /// function.
    ///
    /// This function will emit the `Container::actor-removed`
    /// signal on `self`.
    /// ## `child`
    /// a `Actor`
    fn remove_child<P: Is<Actor>>(&self, child: &P);

    /// Removes clip area from `self`.
    fn remove_clip(&self);

    /// Removes `constraint` from the list of constraints applied to `self`
    ///
    /// The reference held by `self` on the `Constraint` will be released
    /// ## `constraint`
    /// a `Constraint`
    fn remove_constraint<P: Is<Constraint>>(&self, constraint: &P);

    /// Removes the `Constraint` with the given name from the list
    /// of constraints applied to `self`
    /// ## `name`
    /// the name of the constraint to remove
    fn remove_constraint_by_name(&self, name: &str);

    /// Removes `effect` from the list of effects applied to `self`
    ///
    /// The reference held by `self` on the `Effect` will be released
    /// ## `effect`
    /// a `Effect`
    fn remove_effect<P: Is<Effect>>(&self, effect: &P);

    /// Removes the `Effect` with the given name from the list
    /// of effects applied to `self`
    /// ## `name`
    /// the name of the effect to remove
    fn remove_effect_by_name(&self, name: &str);

    /// Removes the transition stored inside a `Actor` using `name`
    /// identifier.
    ///
    /// If the transition is currently in progress, it will be stopped.
    ///
    /// This function releases the reference acquired when the transition
    /// was added to the `Actor`.
    /// ## `name`
    /// the name of the transition to remove
    fn remove_transition(&self, name: &str);

    /// Replaces `old_child` with `new_child` in the list of children of `self`.
    /// ## `old_child`
    /// the child of `self` to replace
    /// ## `new_child`
    /// the `Actor` to replace `old_child`
    fn replace_child<P: Is<Actor>, Q: Is<Actor>>(&self, old_child: &P, new_child: &Q);

    /// Restores the easing state as it was prior to a call to
    /// `ActorExt::save_easing_state`.
    fn restore_easing_state(&self);

    /// Saves the current easing state for animatable properties, and creates
    /// a new state with the default values for easing mode and duration.
    ///
    /// New transitions created after calling this function will inherit the
    /// duration, easing mode, and delay of the new easing state; this also
    /// applies to transitions modified in flight.
    fn save_easing_state(&self);

    /// Stores the allocation of `self` as defined by `box_`.
    ///
    /// This function can only be called from within the implementation of
    /// the `ActorClass.allocate`() virtual function.
    ///
    /// The allocation should have been adjusted to take into account constraints,
    /// alignment, and margin properties. If you are implementing a `Actor`
    /// subclass that provides its own layout management policy for its children
    /// instead of using a `LayoutManager` delegate, you should not call
    /// this function on the children of `self`; instead, you should call
    /// `ActorExt::allocate`, which will adjust the allocation box for
    /// you.
    ///
    /// This function should only be used by subclasses of `Actor`
    /// that wish to store their allocation but cannot chain up to the
    /// parent's implementation; the default implementation of the
    /// `ActorClass.allocate`() virtual function will call this
    /// function.
    ///
    /// It is important to note that, while chaining up was the recommended
    /// behaviour for `Actor` subclasses prior to the introduction of
    /// this function, it is recommended to call `ActorExt::set_allocation`
    /// instead.
    ///
    /// If the `Actor` is using a `LayoutManager` delegate object
    /// to handle the allocation of its children, this function will call
    /// the `LayoutManagerExt::allocate` function only if the
    /// `AllocationFlags::DelegateLayout` flag is set on `flags`, otherwise it is
    /// expected that the subclass will call `LayoutManagerExt::allocate`
    /// by itself. For instance, the following code:
    ///
    ///
    /// ```C
    /// static void
    /// my_actor_allocate (Actor *actor,
    ///                    const ActorBox *allocation,
    ///                    AllocationFlags flags)
    /// {
    ///   ActorBox new_alloc;
    ///   AllocationFlags new_flags;
    ///
    ///   adjust_allocation (allocation, &new_alloc);
    ///
    ///   new_flags = flags | DELEGATE_LAYOUT;
    ///
    ///   // this will use the layout manager set on the actor
    ///   actor_set_allocation (actor, &new_alloc, new_flags);
    /// }
    /// ```
    ///
    /// is equivalent to this:
    ///
    ///
    /// ```C
    /// static void
    /// my_actor_allocate (Actor *actor,
    ///                    const ActorBox *allocation,
    ///                    AllocationFlags flags)
    /// {
    ///   LayoutManager *layout;
    ///   ActorBox new_alloc;
    ///
    ///   adjust_allocation (allocation, &new_alloc);
    ///
    ///   actor_set_allocation (actor, &new_alloc, flags);
    ///
    ///   layout = actor_get_layout_manager (actor);
    ///   layout_manager_allocate (layout,
    ///                                    CONTAINER (actor),
    ///                                    &new_alloc,
    ///                                    flags);
    /// }
    /// ```
    /// ## `box_`
    /// a `ActorBox`
    /// ## `flags`
    /// allocation flags
    fn set_allocation(&self, box_: &ActorBox, flags: AllocationFlags);

    /// Sets the background color of a `Actor`.
    ///
    /// The background color will be used to cover the whole allocation of the
    /// actor. The default background color of an actor is transparent.
    ///
    /// To check whether an actor has a background color, you can use the
    /// `Actor:background-color-set` actor property.
    ///
    /// The `Actor:background-color` property is animatable.
    /// ## `color`
    /// a `Color`, or `None` to unset a previously
    ///  set color
    fn set_background_color(&self, color: Option<Color>);

    /// Sets `child` to be above `sibling` in the list of children of `self`.
    ///
    /// If `sibling` is `None`, `child` will be the new last child of `self`.
    ///
    /// This function is logically equivalent to removing `child` and using
    /// `ActorExt::insert_child_above`, but it will not emit signals
    /// or change state on `child`.
    /// ## `child`
    /// a `Actor` child of `self`
    /// ## `sibling`
    /// a `Actor` child of `self`, or `None`
    fn set_child_above_sibling<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>);

    /// Changes the index of `child` in the list of children of `self`.
    ///
    /// This function is logically equivalent to removing `child` and
    /// calling `ActorExt::insert_child_at_index`, but it will not
    /// emit signals or change state on `child`.
    /// ## `child`
    /// a `Actor` child of `self`
    /// ## `index_`
    /// the new index for `child`
    fn set_child_at_index<P: Is<Actor>>(&self, child: &P, index_: i32);

    /// Sets `child` to be below `sibling` in the list of children of `self`.
    ///
    /// If `sibling` is `None`, `child` will be the new first child of `self`.
    ///
    /// This function is logically equivalent to removing `self` and using
    /// `ActorExt::insert_child_below`, but it will not emit signals
    /// or change state on `child`.
    /// ## `child`
    /// a `Actor` child of `self`
    /// ## `sibling`
    /// a `Actor` child of `self`, or `None`
    fn set_child_below_sibling<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>);

    /// Sets the transformation matrix to be applied to all the children
    /// of `self` prior to their own transformations. The default child
    /// transformation is the identity matrix.
    ///
    /// If `transform` is `None`, the child transform will be unset.
    ///
    /// The `Actor:child-transform` property is animatable.
    /// ## `transform`
    /// a `Matrix`, or `None`
    fn set_child_transform(&self, transform: Option<&Matrix>);

    /// Sets clip area for `self`. The clip area is always computed from the
    /// upper left corner of the actor, even if the anchor point is set
    /// otherwise.
    /// ## `xoff`
    /// X offset of the clip rectangle
    /// ## `yoff`
    /// Y offset of the clip rectangle
    /// ## `width`
    /// Width of the clip rectangle
    /// ## `height`
    /// Height of the clip rectangle
    fn set_clip(&self, xoff: f32, yoff: f32, width: f32, height: f32);

    /// Sets whether `self` should be clipped to the same size as its
    /// allocation
    /// ## `clip_set`
    /// `true` to apply a clip tracking the allocation
    fn set_clip_to_allocation(&self, clip_set: bool);

    /// Sets the contents of a `Actor`.
    /// ## `content`
    /// a `Content`, or `None`
    fn set_content<P: Is<Content>>(&self, content: Option<&P>);

    /// Sets the gravity of the `Content` used by `self`.
    ///
    /// See the description of the `Actor:content-gravity` property for
    /// more information.
    ///
    /// The `Actor:content-gravity` property is animatable.
    /// ## `gravity`
    /// the `ContentGravity`
    fn set_content_gravity(&self, gravity: ContentGravity);

    /// Sets the policy for repeating the `Actor:content` of a
    /// `Actor`. The behaviour is deferred to the `Content`
    /// implementation.
    /// ## `repeat`
    /// the repeat policy
    fn set_content_repeat(&self, repeat: ContentRepeat);

    /// Sets the minification and magnification filter to be applied when
    /// scaling the `Actor:content` of a `Actor`.
    ///
    /// The `Actor:minification-filter` will be used when reducing
    /// the size of the content; the `Actor:magnification-filter`
    /// will be used when increasing the size of the content.
    /// ## `min_filter`
    /// the minification filter for the content
    /// ## `mag_filter`
    /// the magnification filter for the content
    fn set_content_scaling_filters(&self, min_filter: ScalingFilter, mag_filter: ScalingFilter);

    /// Sets the delay that should be applied before tweening animatable
    /// properties.
    /// ## `msecs`
    /// the delay before the start of the tweening, in milliseconds
    fn set_easing_delay(&self, msecs: u32);

    /// Sets the duration of the tweening for animatable properties
    /// of `self` for the current easing state.
    /// ## `msecs`
    /// the duration of the easing, or `None`
    fn set_easing_duration(&self, msecs: u32);

    /// Sets the easing mode for the tweening of animatable properties
    /// of `self`.
    /// ## `mode`
    /// an easing mode, excluding `AnimationMode::CustomMode`
    fn set_easing_mode(&self, mode: AnimationMode);

    /// Sets whether an actor has a fixed position set (and will thus be
    /// unaffected by any layout manager).
    /// ## `is_set`
    /// whether to use fixed position
    fn set_fixed_position_set(&self, is_set: bool);

    /// Sets `flags` on `self`
    ///
    /// This function will emit notifications for the changed properties
    /// ## `flags`
    /// the flags to set
    fn set_flags(&self, flags: ActorFlags);

    /// Forces a height on an actor, causing the actor's preferred width
    /// and height (if any) to be ignored.
    ///
    /// If `height` is -1 the actor will use its preferred height instead of
    /// overriding it, i.e. you can "unset" the height with -1.
    ///
    /// This function sets both the minimum and natural size of the actor.
    /// ## `height`
    /// Requested new height for the actor, in pixels, or -1
    fn set_height(&self, height: f32);

    /// Sets the `LayoutManager` delegate object that will be used to
    /// lay out the children of `self`.
    ///
    /// The `Actor` will take a reference on the passed `manager` which
    /// will be released either when the layout manager is removed, or when
    /// the actor is destroyed.
    /// ## `manager`
    /// a `LayoutManager`, or `None` to unset it
    fn set_layout_manager<P: Is<LayoutManager>>(&self, manager: Option<&P>);

    /// Sets all the components of the margin of a `Actor`.
    /// ## `margin`
    /// a `Margin`
    fn set_margin(&self, margin: &Margin);

    /// Sets the margin from the bottom of a `Actor`.
    ///
    /// The `Actor:margin-bottom` property is animatable.
    /// ## `margin`
    /// the bottom margin
    fn set_margin_bottom(&self, margin: f32);

    /// Sets the margin from the left of a `Actor`.
    ///
    /// The `Actor:margin-left` property is animatable.
    /// ## `margin`
    /// the left margin
    fn set_margin_left(&self, margin: f32);

    /// Sets the margin from the right of a `Actor`.
    ///
    /// The `Actor:margin-right` property is animatable.
    /// ## `margin`
    /// the right margin
    fn set_margin_right(&self, margin: f32);

    /// Sets the margin from the top of a `Actor`.
    ///
    /// The `Actor:margin-top` property is animatable.
    /// ## `margin`
    /// the top margin
    fn set_margin_top(&self, margin: f32);

    /// Sets the given name to `self`. The name can be used to identify
    /// a `Actor`.
    /// ## `name`
    /// Textual tag to apply to actor
    fn set_name(&self, name: &str);

    /// Defines the circumstances where the actor should be redirected into
    /// an offscreen image. The offscreen image is used to flatten the
    /// actor into a single image while painting for two main reasons.
    /// Firstly, when the actor is painted a second time without any of its
    /// contents changing it can simply repaint the cached image without
    /// descending further down the actor hierarchy. Secondly, it will make
    /// the opacity look correct even if there are overlapping primitives
    /// in the actor.
    ///
    /// Caching the actor could in some cases be a performance win and in
    /// some cases be a performance lose so it is important to determine
    /// which value is right for an actor before modifying this value. For
    /// example, there is never any reason to flatten an actor that is just
    /// a single texture (such as a `Texture`) because it is
    /// effectively already cached in an image so the offscreen would be
    /// redundant. Also if the actor contains primitives that are far apart
    /// with a large transparent area in the middle (such as a large
    /// CluterGroup with a small actor in the top left and a small actor in
    /// the bottom right) then the cached image will contain the entire
    /// image of the large area and the paint will waste time blending all
    /// of the transparent pixels in the middle.
    ///
    /// The default method of implementing opacity on a container simply
    /// forwards on the opacity to all of the children. If the children are
    /// overlapping then it will appear as if they are two separate glassy
    /// objects and there will be a break in the color where they
    /// overlap. By redirecting to an offscreen buffer it will be as if the
    /// two opaque objects are combined into one and then made transparent
    /// which is usually what is expected.
    ///
    /// The image below demonstrates the difference between redirecting and
    /// not. The image shows two groups, each containing a red and
    /// a green rectangle which overlap. The opacity on the group is set to
    /// 128 (which is 50%). When the offscreen redirect is not used, the
    /// red rectangle can be seen through the blue rectangle as if the two
    /// rectangles were separately transparent. When the redirect is used
    /// the group as a whole is transparent instead so the red rectangle is
    /// not visible where they overlap.
    ///
    /// <figure id="offscreen-redirect">
    ///  `<title>`Sample of using an offscreen redirect for transparency`</title>`
    ///  <graphic fileref="offscreen-redirect.png" format="PNG"/>
    /// `</figure>`
    ///
    /// The default value for this property is 0, so we effectively will
    /// never redirect an actor offscreen by default. This means that there
    /// are times that transparent actors may look glassy as described
    /// above. The reason this is the default is because there is a
    /// performance trade off between quality and performance here. In many
    /// cases the default form of glassy opacity looks good enough, but if
    /// it's not you will need to set the
    /// `OffscreenRedirect::AutomaticForOpacity` flag to enable
    /// redirection for opacity.
    ///
    /// Custom actors that don't contain any overlapping primitives are
    /// recommended to override the `has_overlaps` virtual to return `false`
    /// for maximum efficiency.
    /// ## `redirect`
    /// New offscreen redirect flags for the actor.
    fn set_offscreen_redirect(&self, redirect: OffscreenRedirect);

    /// Sets the actor's opacity, with zero being completely transparent and
    /// 255 (0xff) being fully opaque.
    ///
    /// The `Actor:opacity` property is animatable.
    /// ## `opacity`
    /// New opacity value for the actor.
    fn set_opacity(&self, opacity: u8);

    /// Sets the position of the `Actor:pivot-point` around which the
    /// scaling and rotation transformations occur.
    ///
    /// The pivot point's coordinates are in normalized space, with the (0, 0)
    /// point being the top left corner of the actor, and the (1, 1) point being
    /// the bottom right corner.
    /// ## `pivot_x`
    /// the normalized X coordinate of the pivot point
    /// ## `pivot_y`
    /// the normalized Y coordinate of the pivot point
    fn set_pivot_point(&self, pivot_x: f32, pivot_y: f32);

    /// Sets the component on the Z axis of the `Actor:pivot-point` around
    /// which the scaling and rotation transformations occur.
    ///
    /// The `pivot_z` value is expressed as a distance along the Z axis.
    /// ## `pivot_z`
    /// the Z coordinate of the actor's pivot point
    fn set_pivot_point_z(&self, pivot_z: f32);

    /// Sets the actor's fixed position in pixels relative to any parent
    /// actor.
    ///
    /// If a layout manager is in use, this position will override the
    /// layout manager and force a fixed position.
    /// ## `x`
    /// New left position of actor in pixels.
    /// ## `y`
    /// New top position of actor in pixels.
    fn set_position(&self, x: f32, y: f32);

    /// Sets `self` as reactive. Reactive actors will receive events.
    /// ## `reactive`
    /// whether the actor should be reactive to events
    fn set_reactive(&self, reactive: bool);

    /// Sets the geometry request mode of `self`.
    ///
    /// The `mode` determines the order for invoking
    /// `ActorExt::get_preferred_width` and
    /// `ActorExt::get_preferred_height`
    /// ## `mode`
    /// the request mode
    fn set_request_mode(&self, mode: RequestMode);

    /// Sets the `angle` of rotation of a `Actor` on the given `axis`.
    ///
    /// This function is a convenience for setting the rotation properties
    /// `Actor:rotation-angle-x`, `Actor:rotation-angle-y`,
    /// and `Actor:rotation-angle-z`.
    ///
    /// The center of rotation is established by the `Actor:pivot-point`
    /// property.
    /// ## `axis`
    /// the axis to set the angle one
    /// ## `angle`
    /// the angle of rotation, in degrees
    fn set_rotation_angle(&self, axis: RotateAxis, angle: f64);

    /// Scales an actor with the given factors.
    ///
    /// The scale transformation is relative the the `Actor:pivot-point`.
    ///
    /// The `Actor:scale-x` and `Actor:scale-y` properties are
    /// animatable.
    /// ## `scale_x`
    /// double factor to scale actor by horizontally.
    /// ## `scale_y`
    /// double factor to scale actor by vertically.
    fn set_scale(&self, scale_x: f64, scale_y: f64);

    /// Scales an actor on the Z axis by the given `scale_z` factor.
    ///
    /// The scale transformation is relative the the `Actor:pivot-point`.
    ///
    /// The `Actor:scale-z` property is animatable.
    /// ## `scale_z`
    /// the scaling factor along the Z axis
    fn set_scale_z(&self, scale_z: f64);

    /// Sets the actor's size request in pixels. This overrides any
    /// "normal" size request the actor would have. For example
    /// a text actor might normally request the size of the text;
    /// this function would force a specific size instead.
    ///
    /// If `width` and/or `height` are -1 the actor will use its
    /// "normal" size request instead of overriding it, i.e.
    /// you can "unset" the size with -1.
    ///
    /// This function sets or unsets both the minimum and natural size.
    /// ## `width`
    /// New width of actor in pixels, or -1
    /// ## `height`
    /// New height of actor in pixels, or -1
    fn set_size(&self, width: f32, height: f32);

    /// Sets the `TextDirection` for an actor
    ///
    /// The passed text direction must not be `TextDirection::Default`
    ///
    /// If `self` implements `Container` then this function will recurse
    /// inside all the children of `self` (including the internal ones).
    ///
    /// Composite actors not implementing `Container`, or actors requiring
    /// special handling when the text direction changes, should connect to
    /// the `gobject::Object::notify` signal for the `Actor:text-direction` property
    /// ## `text_dir`
    /// the text direction for `self`
    fn set_text_direction(&self, text_dir: TextDirection);

    /// Overrides the transformations of a `Actor` with a custom
    /// matrix, which will be applied relative to the origin of the
    /// actor's allocation and to the actor's pivot point.
    ///
    /// The `Actor:transform` property is animatable.
    /// ## `transform`
    /// a `Matrix`, or `None` to
    ///  unset a custom transformation
    fn set_transform(&self, transform: Option<&Matrix>);

    /// Sets an additional translation transformation on a `Actor`,
    /// relative to the `Actor:pivot-point`.
    /// ## `translate_x`
    /// the translation along the X axis
    /// ## `translate_y`
    /// the translation along the Y axis
    /// ## `translate_z`
    /// the translation along the Z axis
    fn set_translation(&self, translate_x: f32, translate_y: f32, translate_z: f32);

    /// Forces a width on an actor, causing the actor's preferred width
    /// and height (if any) to be ignored.
    ///
    /// If `width` is -1 the actor will use its preferred width request
    /// instead of overriding it, i.e. you can "unset" the width with -1.
    ///
    /// This function sets both the minimum and natural size of the actor.
    /// ## `width`
    /// Requested new width for the actor, in pixels, or -1
    fn set_width(&self, width: f32);

    /// Sets the actor's X coordinate, relative to its parent, in pixels.
    ///
    /// Overrides any layout manager and forces a fixed position for
    /// the actor.
    ///
    /// The `Actor:x` property is animatable.
    /// ## `x`
    /// the actor's position on the X axis
    fn set_x(&self, x: f32);

    /// Sets the horizontal alignment policy of a `Actor`, in case the
    /// actor received extra horizontal space.
    ///
    /// See also the `Actor:x-align` property.
    /// ## `x_align`
    /// the horizontal alignment policy
    fn set_x_align(&self, x_align: ActorAlign);

    /// Sets whether a `Actor` should expand horizontally; this means
    /// that layout manager should allocate extra space for the actor, if
    /// possible.
    ///
    /// Setting an actor to expand will also make all its parent expand, so
    /// that it's possible to build an actor tree and only set this flag on
    /// its leaves and not on every single actor.
    /// ## `expand`
    /// whether the actor should expand horizontally
    fn set_x_expand(&self, expand: bool);

    /// Sets the actor's Y coordinate, relative to its parent, in pixels.#
    ///
    /// Overrides any layout manager and forces a fixed position for
    /// the actor.
    ///
    /// The `Actor:y` property is animatable.
    /// ## `y`
    /// the actor's position on the Y axis
    fn set_y(&self, y: f32);

    /// Sets the vertical alignment policy of a `Actor`, in case the
    /// actor received extra vertical space.
    ///
    /// See also the `Actor:y-align` property.
    /// ## `y_align`
    /// the vertical alignment policy
    fn set_y_align(&self, y_align: ActorAlign);

    /// Sets whether a `Actor` should expand horizontally; this means
    /// that layout manager should allocate extra space for the actor, if
    /// possible.
    ///
    /// Setting an actor to expand will also make all its parent expand, so
    /// that it's possible to build an actor tree and only set this flag on
    /// its leaves and not on every single actor.
    /// ## `expand`
    /// whether the actor should expand vertically
    fn set_y_expand(&self, expand: bool);

    /// Sets the actor's position on the Z axis.
    ///
    /// See `Actor:z-position`.
    /// ## `z_position`
    /// the position on the Z axis
    fn set_z_position(&self, z_position: f32);

    /// Should be called inside the implementation of the
    /// `Actor::pick` virtual function in order to check whether
    /// the actor should paint itself in pick mode or not.
    ///
    /// This function should never be called directly by applications.
    ///
    /// # Returns
    ///
    /// `true` if the actor should paint its silhouette,
    ///  `false` otherwise
    fn should_pick_paint(&self) -> bool;

    /// Flags an actor to be displayed. An actor that isn't shown will not
    /// be rendered on the stage.
    ///
    /// Actors are visible by default.
    ///
    /// If this function is called on an actor without a parent, the
    /// `Actor:show-on-set-parent` will be set to `true` as a side
    /// effect.
    fn show(&self);

    /// This function translates screen coordinates (`x`, `y`) to
    /// coordinates relative to the actor. For example, it can be used to translate
    /// screen events from global screen coordinates into actor-local coordinates.
    ///
    /// The conversion can fail, notably if the transform stack results in the
    /// actor being projected on the screen as a mere line.
    ///
    /// The conversion should not be expected to be pixel-perfect due to the
    /// nature of the operation. In general the error grows when the skewing
    /// of the actor rectangle on screen increases.
    ///
    /// This function can be computationally intensive.
    ///
    /// This function only works when the allocation is up-to-date, i.e. inside of
    /// the `ActorClass.paint`() implementation
    /// ## `x`
    /// x screen coordinate of the point to unproject
    /// ## `y`
    /// y screen coordinate of the point to unproject
    /// ## `x_out`
    /// return location for the unprojected x coordinance
    /// ## `y_out`
    /// return location for the unprojected y coordinance
    ///
    /// # Returns
    ///
    /// `true` if conversion was successful.
    fn transform_stage_point(&self, x: f32, y: f32) -> Option<(f32, f32)>;

    /// Unsets the `ActorFlags::Mapped` flag on the actor and possibly
    /// unmaps its children if they were mapped.
    ///
    /// Calling this function is not encouraged: the default `Actor`
    /// implementation of `ActorClass.unmap`() will also unmap any
    /// eventual children by default when their parent is unmapped.
    ///
    /// When overriding `ActorClass.unmap`(), it is mandatory to
    /// chain up to the parent implementation.
    ///
    /// It is important to note that the implementation of the
    /// `ActorClass.unmap`() virtual function may be called after
    /// the `ActorClass.destroy`() or the `gobject::ObjectClass.dispose`()
    /// implementation, but it is guaranteed to be called before the
    /// `gobject::ObjectClass.finalize`() implementation.
    fn unmap(&self);

    /// Unsets `flags` on `self`
    ///
    /// This function will emit notifications for the changed properties
    /// ## `flags`
    /// the flags to unset
    fn unset_flags(&self, flags: ActorFlags);

    /// Adds a `Action` to the actor
    fn set_property_actions<P: Is<Action>>(&self, actions: Option<&P>);

    /// The allocation for the actor, in pixels
    ///
    /// This is property is read-only, but you might monitor it to know when an
    /// actor moves or resizes
    fn get_property_allocation(&self) -> Option<ActorBox>;

    /// Whether the `Actor:background-color` property has been set.
    fn get_property_background_color_set(&self) -> bool;

    /// Whether the `Actor:child-transform` property is set.
    fn get_property_child_transform_set(&self) -> bool;

    /// The visible region of the actor, in actor-relative coordinates,
    /// expressed as a `Rect`.
    ///
    /// Setting this property to `None` will unset the existing clip.
    ///
    /// Setting this property will change the `Actor:has-clip`
    /// property as a side effect.
    fn get_property_clip_rect(&self) -> Option<InternalRect>;

    /// The visible region of the actor, in actor-relative coordinates,
    /// expressed as a `Rect`.
    ///
    /// Setting this property to `None` will unset the existing clip.
    ///
    /// Setting this property will change the `Actor:has-clip`
    /// property as a side effect.
    fn set_property_clip_rect(&self, clip_rect: Option<&InternalRect>);

    /// Adds a `Constraint` to the actor
    fn set_property_constraints<P: Is<Constraint>>(&self, constraints: Option<&P>);

    /// Adds `Effect` to the list of effects be applied on a `Actor`
    fn set_property_effect<P: Is<Effect>>(&self, effect: Option<&P>);

    /// The fixed X position of the actor in pixels.
    ///
    /// Writing this property sets `Actor:fixed-position-set`
    /// property as well, as a side effect
    fn get_property_fixed_x(&self) -> f32;

    /// The fixed X position of the actor in pixels.
    ///
    /// Writing this property sets `Actor:fixed-position-set`
    /// property as well, as a side effect
    fn set_property_fixed_x(&self, fixed_x: f32);

    /// The fixed Y position of the actor in pixels.
    ///
    /// Writing this property sets the `Actor:fixed-position-set`
    /// property as well, as a side effect
    fn get_property_fixed_y(&self) -> f32;

    /// The fixed Y position of the actor in pixels.
    ///
    /// Writing this property sets the `Actor:fixed-position-set`
    /// property as well, as a side effect
    fn set_property_fixed_y(&self, fixed_y: f32);

    /// Whether the actor has the `Actor:clip` property set or not
    fn get_property_has_clip(&self) -> bool;

    /// Whether the actor contains the pointer of a `InputDevice`
    /// or not.
    fn get_property_has_pointer(&self) -> bool;

    fn get_property_magnification_filter(&self) -> ScalingFilter;

    fn set_property_magnification_filter(&self, magnification_filter: ScalingFilter);

    /// Whether the actor is mapped (will be painted when the stage
    /// to which it belongs is mapped)
    fn get_property_mapped(&self) -> bool;

    /// A forced minimum height request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:min-height-set` property
    /// as well, as a side effect. This property overrides the usual height
    /// request of the actor.
    fn get_property_min_height(&self) -> f32;

    /// A forced minimum height request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:min-height-set` property
    /// as well, as a side effect. This property overrides the usual height
    /// request of the actor.
    fn set_property_min_height(&self, min_height: f32);

    /// This flag controls whether the `Actor:min-height` property
    /// is used
    fn get_property_min_height_set(&self) -> bool;

    /// This flag controls whether the `Actor:min-height` property
    /// is used
    fn set_property_min_height_set(&self, min_height_set: bool);

    /// A forced minimum width request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:min-width-set` property
    /// as well, as a side effect.
    ///
    /// This property overrides the usual width request of the actor.
    fn get_property_min_width(&self) -> f32;

    /// A forced minimum width request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:min-width-set` property
    /// as well, as a side effect.
    ///
    /// This property overrides the usual width request of the actor.
    fn set_property_min_width(&self, min_width: f32);

    /// This flag controls whether the `Actor:min-width` property
    /// is used
    fn get_property_min_width_set(&self) -> bool;

    /// This flag controls whether the `Actor:min-width` property
    /// is used
    fn set_property_min_width_set(&self, min_width_set: bool);

    fn get_property_minification_filter(&self) -> ScalingFilter;

    fn set_property_minification_filter(&self, minification_filter: ScalingFilter);

    /// A forced natural height request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:natural-height-set`
    /// property as well, as a side effect. This property overrides the
    /// usual height request of the actor
    fn get_property_natural_height(&self) -> f32;

    /// A forced natural height request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:natural-height-set`
    /// property as well, as a side effect. This property overrides the
    /// usual height request of the actor
    fn set_property_natural_height(&self, natural_height: f32);

    /// This flag controls whether the `Actor:natural-height` property
    /// is used
    fn get_property_natural_height_set(&self) -> bool;

    /// This flag controls whether the `Actor:natural-height` property
    /// is used
    fn set_property_natural_height_set(&self, natural_height_set: bool);

    /// A forced natural width request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:natural-width-set`
    /// property as well, as a side effect. This property overrides the
    /// usual width request of the actor
    fn get_property_natural_width(&self) -> f32;

    /// A forced natural width request for the actor, in pixels
    ///
    /// Writing this property sets the `Actor:natural-width-set`
    /// property as well, as a side effect. This property overrides the
    /// usual width request of the actor
    fn set_property_natural_width(&self, natural_width: f32);

    /// This flag controls whether the `Actor:natural-width` property
    /// is used
    fn get_property_natural_width_set(&self) -> bool;

    /// This flag controls whether the `Actor:natural-width` property
    /// is used
    fn set_property_natural_width_set(&self, natural_width_set: bool);

    /// Whether the actor has been realized
    fn get_property_realized(&self) -> bool;

    /// The rotation angle on the X axis.
    ///
    /// The `Actor:rotation-angle-x` property is animatable.
    fn get_property_rotation_angle_x(&self) -> f64;

    /// The rotation angle on the X axis.
    ///
    /// The `Actor:rotation-angle-x` property is animatable.
    fn set_property_rotation_angle_x(&self, rotation_angle_x: f64);

    /// The rotation angle on the Y axis
    ///
    /// The `Actor:rotation-angle-y` property is animatable.
    fn get_property_rotation_angle_y(&self) -> f64;

    /// The rotation angle on the Y axis
    ///
    /// The `Actor:rotation-angle-y` property is animatable.
    fn set_property_rotation_angle_y(&self, rotation_angle_y: f64);

    /// The rotation angle on the Z axis
    ///
    /// The `Actor:rotation-angle-z` property is animatable.
    fn get_property_rotation_angle_z(&self) -> f64;

    /// The rotation angle on the Z axis
    ///
    /// The `Actor:rotation-angle-z` property is animatable.
    fn set_property_rotation_angle_z(&self, rotation_angle_z: f64);

    /// The horizontal scale of the actor.
    ///
    /// The `Actor:scale-x` property is animatable.
    fn get_property_scale_x(&self) -> f64;

    /// The horizontal scale of the actor.
    ///
    /// The `Actor:scale-x` property is animatable.
    fn set_property_scale_x(&self, scale_x: f64);

    /// The vertical scale of the actor.
    ///
    /// The `Actor:scale-y` property is animatable.
    fn get_property_scale_y(&self) -> f64;

    /// The vertical scale of the actor.
    ///
    /// The `Actor:scale-y` property is animatable.
    fn set_property_scale_y(&self, scale_y: f64);

    /// If `true`, the actor is automatically shown when parented.
    ///
    /// Calling `ActorExt::hide` on an actor which has not been
    /// parented will set this property to `false` as a side effect.
    fn get_property_show_on_set_parent(&self) -> bool;

    /// If `true`, the actor is automatically shown when parented.
    ///
    /// Calling `ActorExt::hide` on an actor which has not been
    /// parented will set this property to `false` as a side effect.
    fn set_property_show_on_set_parent(&self, show_on_set_parent: bool);

    /// Whether the `Actor:transform` property is set.
    fn get_property_transform_set(&self) -> bool;

    /// An additional translation applied along the X axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-x` property is animatable.
    fn get_property_translation_x(&self) -> f32;

    /// An additional translation applied along the X axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-x` property is animatable.
    fn set_property_translation_x(&self, translation_x: f32);

    /// An additional translation applied along the Y axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-y` property is animatable.
    fn get_property_translation_y(&self) -> f32;

    /// An additional translation applied along the Y axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-y` property is animatable.
    fn set_property_translation_y(&self, translation_y: f32);

    /// An additional translation applied along the Z axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-z` property is animatable.
    fn get_property_translation_z(&self) -> f32;

    /// An additional translation applied along the Z axis, relative
    /// to the actor's `Actor:pivot-point`.
    ///
    /// The `Actor:translation-z` property is animatable.
    fn set_property_translation_z(&self, translation_z: f32);

    /// Whether the actor is set to be visible or not
    ///
    /// See also `Actor:mapped`
    fn get_property_visible(&self) -> bool;

    /// Whether the actor is set to be visible or not
    ///
    /// See also `Actor:mapped`
    fn set_property_visible(&self, visible: bool);

    /// The ::allocation-changed signal is emitted when the
    /// `Actor:allocation` property changes. Usually, application
    /// code should just use the notifications for the :allocation property
    /// but if you want to track the allocation flags as well, for instance
    /// to know whether the absolute origin of `actor` changed, then you might
    /// want use this signal instead.
    /// ## `box_`
    /// a `ActorBox` with the new allocation
    /// ## `flags`
    /// `AllocationFlags` for the allocation
    fn connect_allocation_changed<F: Fn(&Self, &ActorBox, AllocationFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::button-press-event signal is emitted each time a mouse button
    /// is pressed on `actor`.
    /// ## `event`
    /// a `ButtonEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_button_press_event<F: Fn(&Self, &ButtonEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::button-release-event signal is emitted each time a mouse button
    /// is released on `actor`.
    /// ## `event`
    /// a `ButtonEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_button_release_event<F: Fn(&Self, &ButtonEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::captured-event signal is emitted when an event is captured.
    /// This signal will be emitted starting from the top-level
    /// container (the `Stage`) to the actor which received the event
    /// going down the hierarchy. This signal can be used to intercept every
    /// event before the specialized events (like
    /// Actor::button-press-event or ::key-released-event) are
    /// emitted.
    /// ## `event`
    /// a `Event`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_captured_event<F: Fn(&Self, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::destroy signal notifies that all references held on the
    /// actor which emitted it should be released.
    ///
    /// The ::destroy signal should be used by all holders of a reference
    /// on `actor`.
    ///
    /// This signal might result in the finalization of the `Actor`
    /// if all references are released.
    ///
    /// Composite actors and actors implementing the `Container`
    /// interface should override the default implementation of the
    /// class handler of this signal and call `ActorExt::destroy` on
    /// their children. When overriding the default class handler, it is
    /// required to chain up to the parent's implementation.
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::enter-event signal is emitted when the pointer enters the `actor`
    /// ## `event`
    /// a `CrossingEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_enter_event<F: Fn(&Self, &CrossingEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::event signal is emitted each time an event is received
    /// by the `actor`. This signal will be emitted on every actor,
    /// following the hierarchy chain, until it reaches the top-level
    /// container (the `Stage`).
    /// ## `event`
    /// a `Event`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::hide signal is emitted when an actor is no longer rendered
    /// on the stage.
    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::key-focus-in signal is emitted when `actor` receives key focus.
    fn connect_key_focus_in<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::key-focus-out signal is emitted when `actor` loses key focus.
    fn connect_key_focus_out<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::key-press-event signal is emitted each time a keyboard button
    /// is pressed while `actor` has key focus (see `StageExt::set_key_focus`).
    /// ## `event`
    /// a `KeyEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_key_press_event<F: Fn(&Self, &KeyEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::key-release-event signal is emitted each time a keyboard button
    /// is released while `actor` has key focus (see
    /// `StageExt::set_key_focus`).
    /// ## `event`
    /// a `KeyEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_key_release_event<F: Fn(&Self, &KeyEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::leave-event signal is emitted when the pointer leaves the `actor`.
    /// ## `event`
    /// a `CrossingEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_leave_event<F: Fn(&Self, &CrossingEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::motion-event signal is emitted each time the mouse pointer is
    /// moved over `actor`.
    /// ## `event`
    /// a `MotionEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_motion_event<F: Fn(&Self, &MotionEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// This signal is emitted when the parent of the actor changes.
    /// ## `old_parent`
    /// the previous parent of the actor, or `None`
    fn connect_parent_set<F: Fn(&Self, Option<&Actor>) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::queue_redraw signal is emitted when `ActorExt::queue_redraw`
    /// is called on `origin`.
    ///
    /// The default implementation for `Actor` chains up to the
    /// parent actor and queues a redraw on the parent, thus "bubbling"
    /// the redraw queue up through the actor graph. The default
    /// implementation for `Stage` queues a `StageExt::ensure_redraw`
    /// in a main loop idle handler.
    ///
    /// Note that the `origin` actor may be the stage, or a container; it
    /// does not have to be a leaf node in the actor graph.
    ///
    /// Toolkits embedding a `Stage` which require a redraw and
    /// relayout cycle can stop the emission of this signal using the
    /// GSignal API, redraw the UI and then call `StageExt::ensure_redraw`
    /// themselves, like:
    ///
    ///
    /// ```C
    ///   static void
    ///   on_redraw_complete (gpointer data)
    ///   {
    ///     Stage *stage = data;
    ///
    ///     // execute the drawing pipeline
    ///     stage_ensure_redraw (stage);
    ///   }
    ///
    ///   static void
    ///   on_stage_queue_redraw (Stage *stage)
    ///   {
    ///     // this prevents the default handler to run
    ///     g_signal_stop_emission_by_name (stage, "queue-redraw");
    ///
    ///     // queue a redraw with the host toolkit and call
    ///     // a function when the redraw has been completed
    ///     queue_a_redraw (G_CALLBACK (on_redraw_complete), stage);
    ///   }
    /// ```
    ///
    /// Note: This signal is emitted before the paint
    /// pipeline is executed. If you want to know when the pipeline has
    /// been completed you should use `threads_add_repaint_func`
    /// or `threads_add_repaint_func_full`.
    /// ## `origin`
    /// the actor which initiated the redraw request
    fn connect_queue_redraw<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::queue_layout signal is emitted when `ActorExt::queue_relayout`
    /// is called on an actor.
    ///
    /// The default implementation for `Actor` chains up to the
    /// parent actor and queues a relayout on the parent, thus "bubbling"
    /// the relayout queue up through the actor graph.
    ///
    /// The main purpose of this signal is to allow relayout to be propagated
    /// properly in the presence of `Clone` actors. Applications will
    /// not normally need to connect to this signal.
    fn connect_queue_relayout<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::scroll-event signal is emitted each time the mouse is
    /// scrolled on `actor`
    /// ## `event`
    /// a `ScrollEvent`
    ///
    /// # Returns
    ///
    /// `true` if the event has been handled by the actor,
    ///  or `false` to continue the emission.
    fn connect_scroll_event<F: Fn(&Self, &ScrollEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::show signal is emitted when an actor is visible and
    /// rendered on the stage.
    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::touch-event signal is emitted each time a touch
    /// begin/end/update/cancel event.
    /// ## `event`
    /// a `Event`
    ///
    /// # Returns
    ///
    /// `EVENT_STOP` if the event has been handled by
    ///  the actor, or `EVENT_PROPAGATE` to continue the emission.
    fn connect_touch_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::transition-stopped signal is emitted once a transition
    /// is stopped; a transition is stopped once it reached its total
    /// duration (including eventual repeats), it has been stopped
    /// using `TimelineExt::stop`, or it has been removed from the
    /// transitions applied on `actor`, using `ActorExt::remove_transition`.
    /// ## `name`
    /// the name of the transition
    /// ## `is_finished`
    /// whether the transition was finished, or stopped
    fn connect_transition_stopped<F: Fn(&Self, &str, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    /// The ::transitions-completed signal is emitted once all transitions
    /// involving `actor` are complete.
    fn connect_transitions_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_allocation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_background_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_background_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_child_transform_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_child_transform_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_clip_rect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_clip_to_allocation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_constraints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_box_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_gravity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_content_repeat_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_effect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fixed_position_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_fixed_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_fixed_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_pointer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_layout_manager_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_magnification_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_bottom_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_margin_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_right_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_margin_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_height_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_min_width_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_minification_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_natural_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_natural_height_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_natural_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_natural_width_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_offscreen_redirect_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_opacity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pivot_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pivot_point_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reactive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_request_mode_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_rotation_angle_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_rotation_angle_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_rotation_angle_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_scale_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scale_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_on_set_parent_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_direction_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_transform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transform_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_translation_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_translation_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_translation_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_x_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_y_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_z_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: Is<Actor>> ActorExt for O {
    fn add_action<P: Is<Action>>(&self, action: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_action(
        //         self.as_ref().to_glib_none().0,
        //         action.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_action_with_name<P: Is<Action>>(&self, name: &str, action: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_action_with_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //         action.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_child<P: Is<Actor>>(&self, child: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_child(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_constraint<P: Is<Constraint>>(&self, constraint: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_constraint(
        //         self.as_ref().to_glib_none().0,
        //         constraint.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_constraint_with_name<P: Is<Constraint>>(&self, name: &str, constraint: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_constraint_with_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //         constraint.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_effect<P: Is<Effect>>(&self, effect: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_effect(
        //         self.as_ref().to_glib_none().0,
        //         effect.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_effect_with_name<P: Is<Effect>>(&self, name: &str, effect: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_effect_with_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //         effect.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn add_transition<P: Is<Transition>>(&self, name: &str, transition: &P) {
        // unsafe {
        //     ffi::clutter_actor_add_transition(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //         transition.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn allocate(&self, box_: &ActorBox, flags: AllocationFlags) {
        // unsafe {
        //     ffi::clutter_actor_allocate(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none().0,
        //         flags.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn allocate_align_fill(
        &self,
        box_: &ActorBox,
        x_align: f64,
        y_align: f64,
        x_fill: bool,
        y_fill: bool,
        flags: AllocationFlags,
    ) {
        // unsafe {
        //     ffi::clutter_actor_allocate_align_fill(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none().0,
        //         x_align,
        //         y_align,
        //         x_fill.to_glib(),
        //         y_fill.to_glib(),
        //         flags.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn allocate_available_size(
        &self,
        x: f32,
        y: f32,
        available_width: f32,
        available_height: f32,
        flags: AllocationFlags,
    ) {
        // unsafe {
        //     ffi::clutter_actor_allocate_available_size(
        //         self.as_ref().to_glib_none().0,
        //         x,
        //         y,
        //         available_width,
        //         available_height,
        //         flags.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn allocate_preferred_size(&self, flags: AllocationFlags) {
        // unsafe {
        //     ffi::clutter_actor_allocate_preferred_size(
        //         self.as_ref().to_glib_none().0,
        //         flags.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn apply_relative_transform_to_point<P: Is<Actor>>(
        &self,
        ancestor: Option<&P>,
        point: &Vertex,
    ) -> Vertex {
        // unsafe {
        //     let mut vertex = Vertex::uninitialized();
        //     ffi::clutter_actor_apply_relative_transform_to_point(
        //         self.as_ref().to_glib_none().0,
        //         ancestor.map(|p| p.as_ref()).to_glib_none().0,
        //         point.to_glib_none().0,
        //         vertex.to_glib_none_mut().0,
        //     );
        //     vertex
        // }
        unimplemented!()
    }

    fn apply_transform_to_point(&self, point: &Vertex) -> Vertex {
        // unsafe {
        //     let mut vertex = Vertex::uninitialized();
        //     ffi::clutter_actor_apply_transform_to_point(
        //         self.as_ref().to_glib_none().0,
        //         point.to_glib_none().0,
        //         vertex.to_glib_none_mut().0,
        //     );
        //     vertex
        // }
        unimplemented!()
    }

    // fn bind_model<P: Is<gio::ListModel>, Q: Fn(&glib::Object) -> Actor + 'static>(
    //     &self,
    //     model: Option<&P>,
    //     create_child_func: Q,
    // ) {
    //     // let create_child_func_data: Box_<Q> = Box_::new(create_child_func);
    //     // unsafe extern "C" fn create_child_func_func<
    //     //     P: Is<gio::ListModel>,
    //     //     Q: Fn(&glib::Object) -> Actor + 'static,
    //     // >(
    //     //     item: *mut gobject_sys::GObject,
    //     //     user_data: glib_sys::gpointer,
    //     // ) -> *mut ffi::ClutterActor {
    //     //     let item = from_glib_borrow(item);
    //     //     let callback: &Q = &*(user_data as *mut _);
    //     //     let res = (*callback)(&item);
    //     //     res.to_glib_full()
    //     // }
    //     // let create_child_func = Some(create_child_func_func::<P, Q> as _);
    //     // unsafe extern "C" fn notify_func<
    //     //     P: Is<gio::ListModel>,
    //     //     Q: Fn(&glib::Object) -> Actor + 'static,
    //     // >(
    //     //     data: glib_sys::gpointer,
    //     // ) {
    //     //     let _callback: Box_<Q> = Box_::from_raw(data as *mut _);
    //     // }
    //     // let destroy_call4 = Some(notify_func::<P, Q> as _);
    //     // let super_callback0: Box_<Q> = create_child_func_data;
    //     // unsafe {
    //     //     ffi::clutter_actor_bind_model(
    //     //         self.as_ref().to_glib_none().0,
    //     //         model.map(|p| p.as_ref()).to_glib_none().0,
    //     //         create_child_func,
    //     //         Box_::into_raw(super_callback0) as *mut _,
    //     //         destroy_call4,
    //     //     );
    //     // }
    //     unimplemented!()
    // }

    //fn bind_model_with_properties<P: Is<gio::ListModel>>(&self, model: &P, child_type: glib::types::Type, first_model_property: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { clutter_sys:clutter_actor_bind_model_with_properties() }
    //}

    fn clear_actions(&self) {
        // unsafe {
        //     ffi::clutter_actor_clear_actions(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn clear_constraints(&self) {
        // unsafe {
        //     ffi::clutter_actor_clear_constraints(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn clear_effects(&self) {
        // unsafe {
        //     ffi::clutter_actor_clear_effects(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn contains<P: Is<Actor>>(&self, descendant: &P) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_contains(
        //         self.as_ref().to_glib_none().0,
        //         descendant.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn continue_paint(&self) {
        // unsafe {
        //     ffi::clutter_actor_continue_paint(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn create_pango_context(&self) -> Option<pango::Context> {
        // unsafe {
        //     from_glib_full(ffi::clutter_actor_create_pango_context(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn create_pango_layout(&self, text: Option<&str>) -> Option<pango::Layout> {
        // unsafe {
        //     from_glib_full(ffi::clutter_actor_create_pango_layout(
        //         self.as_ref().to_glib_none().0,
        //         text.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn destroy(&self) {
        // unsafe {
        //     ffi::clutter_actor_destroy(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn destroy_all_children(&self) {
        // unsafe {
        //     ffi::clutter_actor_destroy_all_children(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn event(&self, event: &Event, capture: bool) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_event(
        //         self.as_ref().to_glib_none().0,
        //         event.to_glib_none().0,
        //         capture.to_glib(),
        //     ))
        // }
        unimplemented!()
    }

    //fn get_abs_allocation_vertices(&self, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4) {
    //    unsafe { clutter_sys:clutter_actor_get_abs_allocation_vertices() }
    //}

    fn get_action(&self, name: &str) -> Option<Action> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_action(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_actions(&self) -> Vec<Action> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_actor_get_actions(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_allocation_box(&self) -> ActorBox {
        // unsafe {
        //     let mut box_ = ActorBox::uninitialized();
        //     ffi::clutter_actor_get_allocation_box(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none_mut().0,
        //     );
        //     box_
        // }
        unimplemented!()
    }

    //fn get_allocation_vertices<P: Is<Actor>>(&self, ancestor: Option<&P>, verts: /*Unimplemented*/FixedArray TypeId { ns_id: 1, id: 16 }; 4) {
    //    unsafe { clutter_sys:clutter_actor_get_allocation_vertices() }
    //}

    fn get_background_color(&self) -> Color {
        // unsafe {
        //     let mut color = InternalColor::uninitialized();
        //     ffi::clutter_actor_get_background_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none_mut().0,
        //     );
        //     color
        // }
        unimplemented!()
    }

    fn get_child_at_index(&self, index_: i32) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_child_at_index(
        //         self.as_ref().to_glib_none().0,
        //         index_,
        //     ))
        // }
        unimplemented!()
    }

    fn get_child_transform(&self) -> Matrix {
        // unsafe {
        //     let mut transform = Matrix::uninitialized();
        //     ffi::clutter_actor_get_child_transform(
        //         self.as_ref().to_glib_none().0,
        //         transform.to_glib_none_mut().0,
        //     );
        //     transform
        // }
        unimplemented!()
    }

    fn get_children(&self) -> Vec<Actor> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_actor_get_children(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_clip(&self) -> (f32, f32, f32, f32) {
        // unsafe {
        //     let mut xoff = mem::MaybeUninit::uninit();
        //     let mut yoff = mem::MaybeUninit::uninit();
        //     let mut width = mem::MaybeUninit::uninit();
        //     let mut height = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_clip(
        //         self.as_ref().to_glib_none().0,
        //         xoff.as_mut_ptr(),
        //         yoff.as_mut_ptr(),
        //         width.as_mut_ptr(),
        //         height.as_mut_ptr(),
        //     );
        //     let xoff = xoff.assume_init();
        //     let yoff = yoff.assume_init();
        //     let width = width.assume_init();
        //     let height = height.assume_init();
        //     (xoff, yoff, width, height)
        // }
        unimplemented!()
    }

    fn get_clip_to_allocation(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_clip_to_allocation(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_constraint(&self, name: &str) -> Option<Constraint> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_constraint(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_constraints(&self) -> Vec<Constraint> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_actor_get_constraints(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_content(&self) -> Option<Content> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_content(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_content_box(&self) -> ActorBox {
        // unsafe {
        //     let mut box_ = ActorBox::uninitialized();
        //     ffi::clutter_actor_get_content_box(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none_mut().0,
        //     );
        //     box_
        // }
        unimplemented!()
    }

    fn get_content_gravity(&self) -> ContentGravity {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_content_gravity(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_content_repeat(&self) -> ContentRepeat {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_content_repeat(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_content_scaling_filters(&self) -> (ScalingFilter, ScalingFilter) {
        // unsafe {
        //     let mut min_filter = mem::MaybeUninit::uninit();
        //     let mut mag_filter = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_content_scaling_filters(
        //         self.as_ref().to_glib_none().0,
        //         min_filter.as_mut_ptr(),
        //         mag_filter.as_mut_ptr(),
        //     );
        //     let min_filter = min_filter.assume_init();
        //     let mag_filter = mag_filter.assume_init();
        //     (from_glib(min_filter), from_glib(mag_filter))
        // }
        unimplemented!()
    }

    fn get_default_paint_volume(&self) -> Option<PaintVolume> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_default_paint_volume(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_easing_delay(&self) -> u32 {
        // unsafe { ffi::clutter_actor_get_easing_delay(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_easing_duration(&self) -> u32 {
        // unsafe { ffi::clutter_actor_get_easing_duration(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_easing_mode(&self) -> AnimationMode {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_easing_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_effect(&self, name: &str) -> Option<Effect> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_effect(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_effects(&self) -> Vec<Effect> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_actor_get_effects(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_first_child(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_first_child(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_fixed_position_set(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_fixed_position_set(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_flags(&self) -> ActorFlags {
        // unsafe { from_glib(ffi::clutter_actor_get_flags(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_height(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_height(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_last_child(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_last_child(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_layout_manager(&self) -> Option<LayoutManager> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_layout_manager(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_margin(&self) -> Margin {
        // unsafe {
        //     let mut margin = Margin::uninitialized();
        //     ffi::clutter_actor_get_margin(
        //         self.as_ref().to_glib_none().0,
        //         margin.to_glib_none_mut().0,
        //     );
        //     margin
        // }
        unimplemented!()
    }

    fn get_margin_bottom(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_margin_bottom(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_margin_left(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_margin_left(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_margin_right(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_margin_right(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_margin_top(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_margin_top(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_n_children(&self) -> i32 {
        // unsafe { ffi::clutter_actor_get_n_children(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_name(&self) -> Option<String> {
        // unsafe { from_glib_none(ffi::clutter_actor_get_name(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_next_sibling(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_next_sibling(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_offscreen_redirect(&self) -> OffscreenRedirect {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_offscreen_redirect(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_opacity(&self) -> u8 {
        // unsafe { ffi::clutter_actor_get_opacity(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_paint_box(&self) -> Option<ActorBox> {
        // unsafe {
        //     let mut box_ = ActorBox::uninitialized();
        //     let ret = from_glib(ffi::clutter_actor_get_paint_box(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none_mut().0,
        //     ));
        //     if ret {
        //         Some(box_)
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }

    fn get_paint_opacity(&self) -> u8 {
        // unsafe { ffi::clutter_actor_get_paint_opacity(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_paint_visibility(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_paint_visibility(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_paint_volume(&self) -> Option<PaintVolume> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_paint_volume(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_pango_context(&self) -> Option<pango::Context> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_pango_context(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_parent(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_parent(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_pivot_point(&self) -> (f32, f32) {
        // unsafe {
        //     let mut pivot_x = mem::MaybeUninit::uninit();
        //     let mut pivot_y = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_pivot_point(
        //         self.as_ref().to_glib_none().0,
        //         pivot_x.as_mut_ptr(),
        //         pivot_y.as_mut_ptr(),
        //     );
        //     let pivot_x = pivot_x.assume_init();
        //     let pivot_y = pivot_y.assume_init();
        //     (pivot_x, pivot_y)
        // }
        unimplemented!()
    }

    fn get_pivot_point_z(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_pivot_point_z(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_position(&self) -> (f32, f32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_position(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     (x, y)
        // }
        unimplemented!()
    }

    fn get_preferred_height(&self, for_width: f32) -> (f32, f32) {
        // unsafe {
        //     let mut min_height_p = mem::MaybeUninit::uninit();
        //     let mut natural_height_p = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_preferred_height(
        //         self.as_ref().to_glib_none().0,
        //         for_width,
        //         min_height_p.as_mut_ptr(),
        //         natural_height_p.as_mut_ptr(),
        //     );
        //     let min_height_p = min_height_p.assume_init();
        //     let natural_height_p = natural_height_p.assume_init();
        //     (min_height_p, natural_height_p)
        // }
        unimplemented!()
    }

    fn get_preferred_size(&self) -> (f32, f32, f32, f32) {
        // unsafe {
        //     let mut min_width_p = mem::MaybeUninit::uninit();
        //     let mut min_height_p = mem::MaybeUninit::uninit();
        //     let mut natural_width_p = mem::MaybeUninit::uninit();
        //     let mut natural_height_p = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_preferred_size(
        //         self.as_ref().to_glib_none().0,
        //         min_width_p.as_mut_ptr(),
        //         min_height_p.as_mut_ptr(),
        //         natural_width_p.as_mut_ptr(),
        //         natural_height_p.as_mut_ptr(),
        //     );
        //     let min_width_p = min_width_p.assume_init();
        //     let min_height_p = min_height_p.assume_init();
        //     let natural_width_p = natural_width_p.assume_init();
        //     let natural_height_p = natural_height_p.assume_init();
        //     (min_width_p, min_height_p, natural_width_p, natural_height_p)
        // }
        unimplemented!()
    }

    fn get_preferred_width(&self, for_height: f32) -> (f32, f32) {
        // unsafe {
        //     let mut min_width_p = mem::MaybeUninit::uninit();
        //     let mut natural_width_p = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_preferred_width(
        //         self.as_ref().to_glib_none().0,
        //         for_height,
        //         min_width_p.as_mut_ptr(),
        //         natural_width_p.as_mut_ptr(),
        //     );
        //     let min_width_p = min_width_p.assume_init();
        //     let natural_width_p = natural_width_p.assume_init();
        //     (min_width_p, natural_width_p)
        // }
        unimplemented!()
    }

    fn get_previous_sibling(&self) -> Option<Actor> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_previous_sibling(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_reactive(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_reactive(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_request_mode(&self) -> RequestMode {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_request_mode(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_rotation_angle(&self, axis: RotateAxis) -> f64 {
        // unsafe {
        //     ffi::clutter_actor_get_rotation_angle(self.as_ref().to_glib_none().0, axis.to_glib())
        // }
        unimplemented!()
    }

    fn get_scale(&self) -> (f64, f64) {
        // unsafe {
        //     let mut scale_x = mem::MaybeUninit::uninit();
        //     let mut scale_y = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_scale(
        //         self.as_ref().to_glib_none().0,
        //         scale_x.as_mut_ptr(),
        //         scale_y.as_mut_ptr(),
        //     );
        //     let scale_x = scale_x.assume_init();
        //     let scale_y = scale_y.assume_init();
        //     (scale_x, scale_y)
        // }
        unimplemented!()
    }

    fn get_scale_z(&self) -> f64 {
        // unsafe { ffi::clutter_actor_get_scale_z(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_size(&self) -> (f32, f32) {
        // unsafe {
        //     let mut width = mem::MaybeUninit::uninit();
        //     let mut height = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_size(
        //         self.as_ref().to_glib_none().0,
        //         width.as_mut_ptr(),
        //         height.as_mut_ptr(),
        //     );
        //     let width = width.assume_init();
        //     let height = height.assume_init();
        //     (width, height)
        // }
        unimplemented!()
    }

    fn get_stage(&self) -> Option<Stage> {
        // unsafe { from_glib_none(ffi::clutter_actor_get_stage(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn get_text_direction(&self) -> TextDirection {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_text_direction(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_transform(&self) -> Matrix {
        // unsafe {
        //     let mut transform = Matrix::uninitialized();
        //     ffi::clutter_actor_get_transform(
        //         self.as_ref().to_glib_none().0,
        //         transform.to_glib_none_mut().0,
        //     );
        //     transform
        // }
        unimplemented!()
    }

    fn get_transformed_paint_volume<P: Is<Actor>>(
        &self,
        relative_to_ancestor: &P,
    ) -> Option<PaintVolume> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_transformed_paint_volume(
        //         self.as_ref().to_glib_none().0,
        //         relative_to_ancestor.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_transformed_position(&self) -> (f32, f32) {
        // unsafe {
        //     let mut x = mem::MaybeUninit::uninit();
        //     let mut y = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_transformed_position(
        //         self.as_ref().to_glib_none().0,
        //         x.as_mut_ptr(),
        //         y.as_mut_ptr(),
        //     );
        //     let x = x.assume_init();
        //     let y = y.assume_init();
        //     (x, y)
        // }
        unimplemented!()
    }

    fn get_transformed_size(&self) -> (f32, f32) {
        // unsafe {
        //     let mut width = mem::MaybeUninit::uninit();
        //     let mut height = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_transformed_size(
        //         self.as_ref().to_glib_none().0,
        //         width.as_mut_ptr(),
        //         height.as_mut_ptr(),
        //     );
        //     let width = width.assume_init();
        //     let height = height.assume_init();
        //     (width, height)
        // }
        unimplemented!()
    }

    fn get_transition(&self, name: &str) -> Option<Transition> {
        // unsafe {
        //     from_glib_none(ffi::clutter_actor_get_transition(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_translation(&self) -> (f32, f32, f32) {
        // unsafe {
        //     let mut translate_x = mem::MaybeUninit::uninit();
        //     let mut translate_y = mem::MaybeUninit::uninit();
        //     let mut translate_z = mem::MaybeUninit::uninit();
        //     ffi::clutter_actor_get_translation(
        //         self.as_ref().to_glib_none().0,
        //         translate_x.as_mut_ptr(),
        //         translate_y.as_mut_ptr(),
        //         translate_z.as_mut_ptr(),
        //     );
        //     let translate_x = translate_x.assume_init();
        //     let translate_y = translate_y.assume_init();
        //     let translate_z = translate_z.assume_init();
        //     (translate_x, translate_y, translate_z)
        // }
        unimplemented!()
    }

    fn get_width(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_width(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_x(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_x(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_x_align(&self) -> ActorAlign {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_x_align(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_x_expand(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_x_expand(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_y(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_y(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn get_y_align(&self) -> ActorAlign {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_y_align(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_y_expand(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_get_y_expand(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn get_z_position(&self) -> f32 {
        // unsafe { ffi::clutter_actor_get_z_position(self.as_ref().to_glib_none().0) }
        unimplemented!()
    }

    fn grab_key_focus(&self) {
        // unsafe {
        //     ffi::clutter_actor_grab_key_focus(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn has_actions(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_actions(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_allocation(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_allocation(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_clip(&self) -> bool {
        // unsafe { from_glib(ffi::clutter_actor_has_clip(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn has_constraints(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_constraints(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_effects(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_effects(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_key_focus(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_key_focus(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_overlaps(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_overlaps(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn has_pointer(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_has_pointer(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn hide(&self) {
        // unsafe {
        //     ffi::clutter_actor_hide(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn insert_child_above<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>) {
        // unsafe {
        //     ffi::clutter_actor_insert_child_above(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         sibling.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn insert_child_at_index<P: Is<Actor>>(&self, child: &P, index_: i32) {
        // unsafe {
        //     ffi::clutter_actor_insert_child_at_index(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         index_,
        //     );
        // }
        unimplemented!()
    }

    fn insert_child_below<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>) {
        // unsafe {
        //     ffi::clutter_actor_insert_child_below(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         sibling.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn is_in_clone_paint(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_is_in_clone_paint(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn is_mapped(&self) -> bool {
        // unsafe { from_glib(ffi::clutter_actor_is_mapped(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn is_realized(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_is_realized(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn is_rotated(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_is_rotated(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn is_scaled(&self) -> bool {
        // unsafe { from_glib(ffi::clutter_actor_is_scaled(self.as_ref().to_glib_none().0)) }
        unimplemented!()
    }

    fn is_visible(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_is_visible(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn map(&self) {
        // unsafe {
        //     ffi::clutter_actor_map(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn move_by(&self, dx: f32, dy: f32) {
        // unsafe {
        //     ffi::clutter_actor_move_by(self.as_ref().to_glib_none().0, dx, dy);
        // }
        unimplemented!()
    }

    fn needs_expand(&self, orientation: Orientation) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_needs_expand(
        //         self.as_ref().to_glib_none().0,
        //         orientation.to_glib(),
        //     ))
        // }
        unimplemented!()
    }

    fn paint(&self) {
        // unsafe {
        //     ffi::clutter_actor_paint(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn queue_redraw(&self) {
        // unsafe {
        //     ffi::clutter_actor_queue_redraw(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn queue_redraw_with_clip(&self, clip: Option<&cairo::RectangleInt>) {
        // unsafe {
        //     ffi::clutter_actor_queue_redraw_with_clip(
        //         self.as_ref().to_glib_none().0,
        //         clip.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn queue_relayout(&self) {
        // unsafe {
        //     ffi::clutter_actor_queue_relayout(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_action<P: Is<Action>>(&self, action: &P) {
        // unsafe {
        //     ffi::clutter_actor_remove_action(
        //         self.as_ref().to_glib_none().0,
        //         action.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_action_by_name(&self, name: &str) {
        // unsafe {
        //     ffi::clutter_actor_remove_action_by_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_all_children(&self) {
        // unsafe {
        //     ffi::clutter_actor_remove_all_children(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_all_transitions(&self) {
        // unsafe {
        //     ffi::clutter_actor_remove_all_transitions(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_child<P: Is<Actor>>(&self, child: &P) {
        // unsafe {
        //     ffi::clutter_actor_remove_child(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_clip(&self) {
        // unsafe {
        //     ffi::clutter_actor_remove_clip(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn remove_constraint<P: Is<Constraint>>(&self, constraint: &P) {
        // unsafe {
        //     ffi::clutter_actor_remove_constraint(
        //         self.as_ref().to_glib_none().0,
        //         constraint.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_constraint_by_name(&self, name: &str) {
        // unsafe {
        //     ffi::clutter_actor_remove_constraint_by_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_effect<P: Is<Effect>>(&self, effect: &P) {
        // unsafe {
        //     ffi::clutter_actor_remove_effect(
        //         self.as_ref().to_glib_none().0,
        //         effect.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_effect_by_name(&self, name: &str) {
        // unsafe {
        //     ffi::clutter_actor_remove_effect_by_name(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn remove_transition(&self, name: &str) {
        // unsafe {
        //     ffi::clutter_actor_remove_transition(
        //         self.as_ref().to_glib_none().0,
        //         name.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn replace_child<P: Is<Actor>, Q: Is<Actor>>(&self, old_child: &P, new_child: &Q) {
        // unsafe {
        //     ffi::clutter_actor_replace_child(
        //         self.as_ref().to_glib_none().0,
        //         old_child.as_ref().to_glib_none().0,
        //         new_child.as_ref().to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn restore_easing_state(&self) {
        // unsafe {
        //     ffi::clutter_actor_restore_easing_state(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn save_easing_state(&self) {
        // unsafe {
        //     ffi::clutter_actor_save_easing_state(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_allocation(&self, box_: &ActorBox, flags: AllocationFlags) {
        // unsafe {
        //     ffi::clutter_actor_set_allocation(
        //         self.as_ref().to_glib_none().0,
        //         box_.to_glib_none().0,
        //         flags.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_background_color(&self, color: Option<Color>) {
        // let color = match color {
        //     Some(value) => {
        //         let RgbaColor {
        //             red,
        //             green,
        //             blue,
        //             alpha,
        //         } = value.into();
        //         Some(InternalColor::new(red, green, blue, alpha))
        //     }
        //     None => None,
        // };

        // unsafe {
        //     ffi::clutter_actor_set_background_color(
        //         self.as_ref().to_glib_none().0,
        //         color.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_child_above_sibling<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>) {
        // unsafe {
        //     ffi::clutter_actor_set_child_above_sibling(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         sibling.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_child_at_index<P: Is<Actor>>(&self, child: &P, index_: i32) {
        // unsafe {
        //     ffi::clutter_actor_set_child_at_index(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         index_,
        //     );
        // }
        unimplemented!()
    }

    fn set_child_below_sibling<P: Is<Actor>, Q: Is<Actor>>(&self, child: &P, sibling: Option<&Q>) {
        // unsafe {
        //     ffi::clutter_actor_set_child_below_sibling(
        //         self.as_ref().to_glib_none().0,
        //         child.as_ref().to_glib_none().0,
        //         sibling.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_child_transform(&self, transform: Option<&Matrix>) {
        // unsafe {
        //     ffi::clutter_actor_set_child_transform(
        //         self.as_ref().to_glib_none().0,
        //         transform.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_clip(&self, xoff: f32, yoff: f32, width: f32, height: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_clip(self.as_ref().to_glib_none().0, xoff, yoff, width, height);
        // }
        unimplemented!()
    }

    fn set_clip_to_allocation(&self, clip_set: bool) {
        // unsafe {
        //     ffi::clutter_actor_set_clip_to_allocation(
        //         self.as_ref().to_glib_none().0,
        //         clip_set.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_content<P: Is<Content>>(&self, content: Option<&P>) {
        // unsafe {
        //     ffi::clutter_actor_set_content(
        //         self.as_ref().to_glib_none().0,
        //         content.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_content_gravity(&self, gravity: ContentGravity) {
        // unsafe {
        //     ffi::clutter_actor_set_content_gravity(
        //         self.as_ref().to_glib_none().0,
        //         gravity.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_content_repeat(&self, repeat: ContentRepeat) {
        // unsafe {
        //     ffi::clutter_actor_set_content_repeat(self.as_ref().to_glib_none().0, repeat.to_glib());
        // }
        unimplemented!()
    }

    fn set_content_scaling_filters(&self, min_filter: ScalingFilter, mag_filter: ScalingFilter) {
        // unsafe {
        //     ffi::clutter_actor_set_content_scaling_filters(
        //         self.as_ref().to_glib_none().0,
        //         min_filter.to_glib(),
        //         mag_filter.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_easing_delay(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_actor_set_easing_delay(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    fn set_easing_duration(&self, msecs: u32) {
        // unsafe {
        //     ffi::clutter_actor_set_easing_duration(self.as_ref().to_glib_none().0, msecs);
        // }
        unimplemented!()
    }

    fn set_easing_mode(&self, mode: AnimationMode) {
        // unsafe {
        //     ffi::clutter_actor_set_easing_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        // }
        unimplemented!()
    }

    fn set_fixed_position_set(&self, is_set: bool) {
        // unsafe {
        //     ffi::clutter_actor_set_fixed_position_set(
        //         self.as_ref().to_glib_none().0,
        //         is_set.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_flags(&self, flags: ActorFlags) {
        // unsafe {
        //     ffi::clutter_actor_set_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        // }
        unimplemented!()
    }

    fn set_height(&self, height: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_height(self.as_ref().to_glib_none().0, height);
        // }
        unimplemented!()
    }

    fn set_layout_manager<P: Is<LayoutManager>>(&self, manager: Option<&P>) {
        // unsafe {
        //     ffi::clutter_actor_set_layout_manager(
        //         self.as_ref().to_glib_none().0,
        //         manager.map(|p| p.as_ref()).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_margin(&self, margin: &Margin) {
        // unsafe {
        //     ffi::clutter_actor_set_margin(self.as_ref().to_glib_none().0, margin.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_margin_bottom(&self, margin: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_margin_bottom(self.as_ref().to_glib_none().0, margin);
        // }
        unimplemented!()
    }

    fn set_margin_left(&self, margin: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_margin_left(self.as_ref().to_glib_none().0, margin);
        // }
        unimplemented!()
    }

    fn set_margin_right(&self, margin: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_margin_right(self.as_ref().to_glib_none().0, margin);
        // }
        unimplemented!()
    }

    fn set_margin_top(&self, margin: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_margin_top(self.as_ref().to_glib_none().0, margin);
        // }
        unimplemented!()
    }

    fn set_name(&self, name: &str) {
        // unsafe {
        //     ffi::clutter_actor_set_name(self.as_ref().to_glib_none().0, name.to_glib_none().0);
        // }
        unimplemented!()
    }

    fn set_offscreen_redirect(&self, redirect: OffscreenRedirect) {
        // unsafe {
        //     ffi::clutter_actor_set_offscreen_redirect(
        //         self.as_ref().to_glib_none().0,
        //         redirect.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_opacity(&self, opacity: u8) {
        // unsafe {
        //     ffi::clutter_actor_set_opacity(self.as_ref().to_glib_none().0, opacity);
        // }
        unimplemented!()
    }

    fn set_pivot_point(&self, pivot_x: f32, pivot_y: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_pivot_point(self.as_ref().to_glib_none().0, pivot_x, pivot_y);
        // }
        unimplemented!()
    }

    fn set_pivot_point_z(&self, pivot_z: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_pivot_point_z(self.as_ref().to_glib_none().0, pivot_z);
        // }
        unimplemented!()
    }

    fn set_position(&self, x: f32, y: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_position(self.as_ref().to_glib_none().0, x, y);
        // }
        unimplemented!()
    }

    fn set_reactive(&self, reactive: bool) {
        // unsafe {
        //     ffi::clutter_actor_set_reactive(self.as_ref().to_glib_none().0, reactive.to_glib());
        // }
        unimplemented!()
    }

    fn set_request_mode(&self, mode: RequestMode) {
        // unsafe {
        //     ffi::clutter_actor_set_request_mode(self.as_ref().to_glib_none().0, mode.to_glib());
        // }
        unimplemented!()
    }

    fn set_rotation_angle(&self, axis: RotateAxis, angle: f64) {
        // unsafe {
        //     ffi::clutter_actor_set_rotation_angle(
        //         self.as_ref().to_glib_none().0,
        //         axis.to_glib(),
        //         angle,
        //     );
        // }
        unimplemented!()
    }

    fn set_scale(&self, scale_x: f64, scale_y: f64) {
        // unsafe {
        //     ffi::clutter_actor_set_scale(self.as_ref().to_glib_none().0, scale_x, scale_y);
        // }
        unimplemented!()
    }

    fn set_scale_z(&self, scale_z: f64) {
        // unsafe {
        //     ffi::clutter_actor_set_scale_z(self.as_ref().to_glib_none().0, scale_z);
        // }
        unimplemented!()
    }

    fn set_size(&self, width: f32, height: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_size(self.as_ref().to_glib_none().0, width, height);
        // }
        unimplemented!()
    }

    fn set_text_direction(&self, text_dir: TextDirection) {
        // unsafe {
        //     ffi::clutter_actor_set_text_direction(
        //         self.as_ref().to_glib_none().0,
        //         text_dir.to_glib(),
        //     );
        // }
        unimplemented!()
    }

    fn set_transform(&self, transform: Option<&Matrix>) {
        // unsafe {
        //     ffi::clutter_actor_set_transform(
        //         self.as_ref().to_glib_none().0,
        //         transform.to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_translation(&self, translate_x: f32, translate_y: f32, translate_z: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_translation(
        //         self.as_ref().to_glib_none().0,
        //         translate_x,
        //         translate_y,
        //         translate_z,
        //     );
        // }
        unimplemented!()
    }

    fn set_width(&self, width: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_width(self.as_ref().to_glib_none().0, width);
        // }
        unimplemented!()
    }

    fn set_x(&self, x: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_x(self.as_ref().to_glib_none().0, x);
        // }
        unimplemented!()
    }

    fn set_x_align(&self, x_align: ActorAlign) {
        // unsafe {
        //     ffi::clutter_actor_set_x_align(self.as_ref().to_glib_none().0, x_align.to_glib());
        // }
        unimplemented!()
    }

    fn set_x_expand(&self, expand: bool) {
        // unsafe {
        //     ffi::clutter_actor_set_x_expand(self.as_ref().to_glib_none().0, expand.to_glib());
        // }
        unimplemented!()
    }

    fn set_y(&self, y: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_y(self.as_ref().to_glib_none().0, y);
        // }
        unimplemented!()
    }

    fn set_y_align(&self, y_align: ActorAlign) {
        // unsafe {
        //     ffi::clutter_actor_set_y_align(self.as_ref().to_glib_none().0, y_align.to_glib());
        // }
        unimplemented!()
    }

    fn set_y_expand(&self, expand: bool) {
        // unsafe {
        //     ffi::clutter_actor_set_y_expand(self.as_ref().to_glib_none().0, expand.to_glib());
        // }
        unimplemented!()
    }

    fn set_z_position(&self, z_position: f32) {
        // unsafe {
        //     ffi::clutter_actor_set_z_position(self.as_ref().to_glib_none().0, z_position);
        // }
        unimplemented!()
    }

    fn should_pick_paint(&self) -> bool {
        // unsafe {
        //     from_glib(ffi::clutter_actor_should_pick_paint(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn show(&self) {
        // unsafe {
        //     ffi::clutter_actor_show(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn transform_stage_point(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        // unsafe {
        //     let mut x_out = mem::MaybeUninit::uninit();
        //     let mut y_out = mem::MaybeUninit::uninit();
        //     let ret = from_glib(ffi::clutter_actor_transform_stage_point(
        //         self.as_ref().to_glib_none().0,
        //         x,
        //         y,
        //         x_out.as_mut_ptr(),
        //         y_out.as_mut_ptr(),
        //     ));
        //     let x_out = x_out.assume_init();
        //     let y_out = y_out.assume_init();
        //     if ret {
        //         Some((x_out, y_out))
        //     } else {
        //         None
        //     }
        // }
        unimplemented!()
    }

    fn unmap(&self) {
        // unsafe {
        //     ffi::clutter_actor_unmap(self.as_ref().to_glib_none().0);
        // }
        unimplemented!()
    }

    fn unset_flags(&self, flags: ActorFlags) {
        // unsafe {
        //     ffi::clutter_actor_unset_flags(self.as_ref().to_glib_none().0, flags.to_glib());
        // }
        unimplemented!()
    }

    fn set_property_actions<P: Is<Action>>(&self, actions: Option<&P>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"actions\0".as_ptr() as *const _,
        //         Value::from(actions).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_allocation(&self) -> Option<ActorBox> {
        // unsafe {
        //     let mut value = Value::from_type(<ActorBox as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"allocation\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `allocation` getter")
        // }
        unimplemented!()
    }

    fn get_property_background_color_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"background-color-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `background-color-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_child_transform_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"child-transform-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `child-transform-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_clip_rect(&self) -> Option<InternalRect> {
        // unsafe {
        //     let mut value = Value::from_type(<InternalRect as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"clip-rect\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `clip-rect` getter")
        // }
        unimplemented!()
    }

    fn set_property_clip_rect(&self, clip_rect: Option<&InternalRect>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"clip-rect\0".as_ptr() as *const _,
        //         Value::from(clip_rect).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_property_constraints<P: Is<Constraint>>(&self, constraints: Option<&P>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"constraints\0".as_ptr() as *const _,
        //         Value::from(constraints).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn set_property_effect<P: Is<Effect>>(&self, effect: Option<&P>) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"effect\0".as_ptr() as *const _,
        //         Value::from(effect).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_fixed_x(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fixed-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `fixed-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_fixed_x(&self, fixed_x: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fixed-x\0".as_ptr() as *const _,
        //         Value::from(&fixed_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_fixed_y(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fixed-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `fixed-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_fixed_y(&self, fixed_y: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"fixed-y\0".as_ptr() as *const _,
        //         Value::from(&fixed_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_has_clip(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"has-clip\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `has-clip` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_has_pointer(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"has-pointer\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `has-pointer` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_magnification_filter(&self) -> ScalingFilter {
        // unsafe {
        //     let mut value = Value::from_type(<ScalingFilter as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"magnification-filter\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `magnification-filter` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_magnification_filter(&self, magnification_filter: ScalingFilter) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"magnification-filter\0".as_ptr() as *const _,
        //         Value::from(&magnification_filter).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_mapped(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"mapped\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `mapped` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_min_height(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-height\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-height` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_height(&self, min_height: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-height\0".as_ptr() as *const _,
        //         Value::from(&min_height).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_min_height_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-height-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-height-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_height_set(&self, min_height_set: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-height-set\0".as_ptr() as *const _,
        //         Value::from(&min_height_set).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_min_width(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_width(&self, min_width: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-width\0".as_ptr() as *const _,
        //         Value::from(&min_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_min_width_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-width-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `min-width-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_min_width_set(&self, min_width_set: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"min-width-set\0".as_ptr() as *const _,
        //         Value::from(&min_width_set).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_minification_filter(&self) -> ScalingFilter {
        // unsafe {
        //     let mut value = Value::from_type(<ScalingFilter as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"minification-filter\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `minification-filter` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_minification_filter(&self, minification_filter: ScalingFilter) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"minification-filter\0".as_ptr() as *const _,
        //         Value::from(&minification_filter).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_natural_height(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-height\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `natural-height` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_natural_height(&self, natural_height: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-height\0".as_ptr() as *const _,
        //         Value::from(&natural_height).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_natural_height_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-height-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `natural-height-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_natural_height_set(&self, natural_height_set: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-height-set\0".as_ptr() as *const _,
        //         Value::from(&natural_height_set).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_natural_width(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-width\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `natural-width` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_natural_width(&self, natural_width: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-width\0".as_ptr() as *const _,
        //         Value::from(&natural_width).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_natural_width_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-width-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `natural-width-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_natural_width_set(&self, natural_width_set: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"natural-width-set\0".as_ptr() as *const _,
        //         Value::from(&natural_width_set).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_realized(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"realized\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `realized` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_rotation_angle_x(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<f64 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `rotation-angle-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_rotation_angle_x(&self, rotation_angle_x: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-x\0".as_ptr() as *const _,
        //         Value::from(&rotation_angle_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_rotation_angle_y(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<f64 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `rotation-angle-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_rotation_angle_y(&self, rotation_angle_y: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-y\0".as_ptr() as *const _,
        //         Value::from(&rotation_angle_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_rotation_angle_z(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<f64 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-z\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `rotation-angle-z` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_rotation_angle_z(&self, rotation_angle_z: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"rotation-angle-z\0".as_ptr() as *const _,
        //         Value::from(&rotation_angle_z).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_scale_x(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<f64 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"scale-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `scale-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_scale_x(&self, scale_x: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"scale-x\0".as_ptr() as *const _,
        //         Value::from(&scale_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_scale_y(&self) -> f64 {
        // unsafe {
        //     let mut value = Value::from_type(<f64 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"scale-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `scale-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_scale_y(&self, scale_y: f64) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"scale-y\0".as_ptr() as *const _,
        //         Value::from(&scale_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_show_on_set_parent(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"show-on-set-parent\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `show-on-set-parent` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_show_on_set_parent(&self, show_on_set_parent: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"show-on-set-parent\0".as_ptr() as *const _,
        //         Value::from(&show_on_set_parent).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_transform_set(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"transform-set\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `transform-set` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn get_property_translation_x(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-x\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `translation-x` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_translation_x(&self, translation_x: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-x\0".as_ptr() as *const _,
        //         Value::from(&translation_x).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_translation_y(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-y\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `translation-y` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_translation_y(&self, translation_y: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-y\0".as_ptr() as *const _,
        //         Value::from(&translation_y).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_translation_z(&self) -> f32 {
        // unsafe {
        //     let mut value = Value::from_type(<f32 as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-z\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `translation-z` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_translation_z(&self, translation_z: f32) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"translation-z\0".as_ptr() as *const _,
        //         Value::from(&translation_z).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn get_property_visible(&self) -> bool {
        // unsafe {
        //     let mut value = Value::from_type(<bool as StaticType>::static_type());
        //     gobject_sys::g_object_get_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"visible\0".as_ptr() as *const _,
        //         value.to_glib_none_mut().0,
        //     );
        //     value
        //         .get()
        //         .expect("Return Value for property `visible` getter")
        //         .unwrap()
        // }
        unimplemented!()
    }

    fn set_property_visible(&self, visible: bool) {
        // unsafe {
        //     gobject_sys::g_object_set_property(
        //         self.to_glib_none().0 as *mut gobject_sys::GObject,
        //         b"visible\0".as_ptr() as *const _,
        //         Value::from(&visible).to_glib_none().0,
        //     );
        // }
        unimplemented!()
    }

    fn connect_allocation_changed<F: Fn(&Self, &ActorBox, AllocationFlags) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_button_press_event<F: Fn(&Self, &ButtonEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_button_release_event<F: Fn(&Self, &ButtonEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_captured_event<F: Fn(&Self, &Event) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_enter_event<F: Fn(&Self, &CrossingEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_hide<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_key_focus_in<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_key_focus_out<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_key_press_event<F: Fn(&Self, &KeyEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_key_release_event<F: Fn(&Self, &KeyEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_leave_event<F: Fn(&Self, &CrossingEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_motion_event<F: Fn(&Self, &MotionEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_parent_set<F: Fn(&Self, Option<&Actor>) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_queue_redraw<F: Fn(&Self, &Actor) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_queue_relayout<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_scroll_event<F: Fn(&Self, &ScrollEvent) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_show<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_touch_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_transition_stopped<F: Fn(&Self, &str, bool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_transitions_completed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_actions_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_allocation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_background_color_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_background_color_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_child_transform_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_child_transform_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_clip_rect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_clip_to_allocation_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_constraints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_content_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_content_box_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_content_gravity_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_content_repeat_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_effect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_first_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_fixed_position_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_fixed_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_fixed_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_has_clip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_has_pointer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_last_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_layout_manager_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_magnification_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_margin_bottom_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_margin_left_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_margin_right_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_margin_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_min_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_min_height_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_min_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_min_width_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_minification_filter_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_natural_height_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_natural_height_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_natural_width_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_natural_width_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_offscreen_redirect_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_opacity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_pivot_point_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_pivot_point_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_reactive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_realized_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_request_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_rotation_angle_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_rotation_angle_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_rotation_angle_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_scale_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_scale_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_scale_z_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_show_on_set_parent_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_text_direction_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_transform_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_transform_set_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_translation_x_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_translation_y_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_translation_z_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_x_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_x_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_x_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_y_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_y_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_y_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }

    fn connect_property_z_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unimplemented!()
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Actor")
    }
}
