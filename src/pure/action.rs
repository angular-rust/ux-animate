use crate::prelude::*;
use super::ActorMeta;
use std::fmt;

// Action:
//
// The #Action structure contains only private data and
// should be accessed using the provided API.
//

// It should be Trait
#[derive(Default, Debug, Clone)]
pub struct Action {
    inner: ActorMeta,
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

impl AsRef<ActorMeta> for Action {
    fn as_ref(&self) -> &ActorMeta {
        &self.inner
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action")
    }
}
