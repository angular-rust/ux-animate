// use crate::Script;
// use std::fmt;

// glib_wrapper! {
//     pub struct Scriptable(Interface<ffi::ClutterScriptable>);

//     match fn {
//         get_type => || ffi::clutter_scriptable_get_type(),
//     }
// }

// pub const NONE_SCRIPTABLE: Option<&Scriptable> = None;

// /// Trait containing all `Scriptable` methods.
// ///
// /// # Implementors
// ///
// /// [`Actor`](struct.Actor.html), [`Alpha`](struct.Alpha.html), [`BehaviourEllipse`](struct.BehaviourEllipse.html), [`BehaviourRotate`](struct.BehaviourRotate.html), [`Behaviour`](struct.Behaviour.html), [`Box`](struct.Box.html), [`Clone`](struct.Clone.html), [`Group`](struct.Group.html), [`Interval`](struct.Interval.html), [`KeyframeTransition`](struct.KeyframeTransition.html), [`PropertyTransition`](struct.PropertyTransition.html), [`Rectangle`](struct.Rectangle.html), [`Scriptable`](struct.Scriptable.html), [`ScrollActor`](struct.ScrollActor.html), [`Stage`](struct.Stage.html), [`State`](struct.State.html), [`Text`](struct.Text.html), [`Texture`](struct.Texture.html), [`Timeline`](struct.Timeline.html), [`TransitionGroup`](struct.TransitionGroup.html), [`Transition`](struct.Transition.html)
// pub trait ScriptableExt: 'static {
//     /// Retrieves the id of `self` set using `Scriptable::set_id`.
//     ///
//     /// # Returns
//     ///
//     /// the id of the object. The returned string is owned by
//     ///  the scriptable object and should never be modified of freed
//     fn get_id(&self) -> Option<GString>;

//     // /// Parses the passed JSON node. The implementation must set the type
//     // /// of the passed `gobject::Value` pointer using `gobject::Value::init`.
//     // /// ## `script`
//     // /// the `Script` creating the scriptable instance
//     // /// ## `value`
//     // /// the generic value to be set
//     // /// ## `name`
//     // /// the name of the node
//     // /// ## `node`
//     // /// the JSON node to be parsed
//     // ///
//     // /// # Returns
//     // ///
//     // /// `true` if the node was successfully parsed, `false` otherwise.
//     // fn parse_custom_node<P: Is<Script>>(
//     //     &self,
//     //     script: &P,
//     //     value: &mut glib::Value,
//     //     name: &str,
//     //     node: &json::Node,
//     // ) -> bool;

//     /// Overrides the common properties setting. The underlying virtual
//     /// function should be used when implementing custom properties.
//     /// ## `script`
//     /// the `Script` creating the scriptable instance
//     /// ## `name`
//     /// the name of the property
//     /// ## `value`
//     /// the value of the property
//     fn set_custom_property<P: Is<Script>>(&self, script: &P, name: &str, value: &glib::Value);

//     /// Sets `id_` as the unique Clutter script it for this instance of
//     /// `ScriptableIface`.
//     ///
//     /// This name can be used by user interface designer applications to
//     /// define a unique name for an object constructable using the UI
//     /// definition language parsed by `Script`.
//     /// ## `id_`
//     /// the `Script` id of the object
//     fn set_id(&self, id_: &str);
// }

// impl<O: Is<Scriptable>> ScriptableExt for O {
//     // fn get_id(&self) -> Option<GString> {
//     //     unsafe {
//     //         from_glib_none(ffi::clutter_scriptable_get_id(
//     //             self.as_ref().to_glib_none().0,
//     //         ))
//     //     }
//     // }

//     // fn parse_custom_node<P: Is<Script>>(
//     //     &self,
//     //     script: &P,
//     //     value: &mut glib::Value,
//     //     name: &str,
//     //     node: &json::Node,
//     // ) -> bool {
//     //     unsafe {
//     //         from_glib(ffi::clutter_scriptable_parse_custom_node(
//     //             self.as_ref().to_glib_none().0,
//     //             script.as_ref().to_glib_none().0,
//     //             value.to_glib_none_mut().0,
//     //             name.to_glib_none().0,
//     //             node.to_glib_none().0,
//     //         ))
//     //     }
//     // }

//     fn set_custom_property<P: Is<Script>>(&self, script: &P, name: &str, value: &glib::Value) {
//         unsafe {
//             ffi::clutter_scriptable_set_custom_property(
//                 self.as_ref().to_glib_none().0,
//                 script.as_ref().to_glib_none().0,
//                 name.to_glib_none().0,
//                 value.to_glib_none().0,
//             );
//         }
//     }

//     fn set_id(&self, id_: &str) {
//         unsafe {
//             ffi::clutter_scriptable_set_id(self.as_ref().to_glib_none().0, id_.to_glib_none().0);
//         }
//     }
// }

// impl fmt::Display for Scriptable {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Scriptable")
//     }
// }
