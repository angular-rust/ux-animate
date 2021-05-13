use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Model {
}

impl Model {}

impl Object for Model {}
impl Is<Model> for Model {}

impl AsRef<Model> for Model {
    fn as_ref(&self) -> &Model {
        self
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Model")
    }
}
