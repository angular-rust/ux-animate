use crate::prelude::*;
// use super::ActorMeta;
use std::fmt;

// * ClutterAction:
// *
// * The #ClutterAction structure contains only private data and
// * should be accessed using the provided API.
// *

// It should be Trait
// extends ActorMeta
#[derive(Debug, Clone)]
pub struct Action {
    // parent_instance: ClutterActorMeta,
}

impl Action {
    // fn init(&self);
}

impl Object for Action {}
impl Is<Action> for Action {}

impl AsRef<Action> for Action {
    fn as_ref(&self) -> &Action {
        self
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
