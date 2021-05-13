use std::fmt;

// @extends Model
#[derive(Debug, Clone)]
pub struct ListModel {}

impl ListModel {}

impl fmt::Display for ListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListModel")
    }
}
