use crate::PaintNode;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct PipelineNode(Object<ffi::ClutterPipelineNode, ffi::ClutterPipelineNodeClass, PipelineNodeClass>) @extends PaintNode;

    match fn {
        get_type => || ffi::clutter_pipeline_node_get_type(),
    }
}

impl PipelineNode {}

impl fmt::Display for PipelineNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PipelineNode")
    }
}
