use std::fmt;

// @extends PaintNode
pub struct PipelineNode{
}

impl PipelineNode {}

impl fmt::Display for PipelineNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineNode")
    }
}
