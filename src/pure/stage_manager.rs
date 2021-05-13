use super::{HandlerId, Stage};
use crate::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StageManager {}

impl StageManager {
    /// Returns the default `StageManager`.
    ///
    /// # Returns
    ///
    /// the default stage manager instance. The returned
    ///  object is owned by internals and you should not reference or unreference it.
    pub fn get_default() -> Option<StageManager> {
        // unsafe { from_glib_none(ffi::clutter_stage_manager_get_default()) }
        unimplemented!()
    }
}

impl Object for StageManager {}
impl Is<StageManager> for StageManager {}

impl AsRef<StageManager> for StageManager {
    fn as_ref(&self) -> &StageManager {
        self
    }
}

/// Trait containing all `StageManager` methods.
///
/// # Implementors
///
/// [`StageManager`](struct.StageManager.html)
pub trait StageManagerExt: 'static {
    /// Returns the default `Stage`.
    ///
    /// # Returns
    ///
    /// the default stage. The returned object
    ///  is owned by internals and you should never reference or unreference it
    fn get_default_stage(&self) -> Option<Stage>;

    /// Lists all currently used stages.
    ///
    /// # Returns
    ///
    /// a newly
    ///  allocated list of `Stage` objects. Use `glib::SList::free` to
    ///  deallocate it when done.
    fn list_stages(&self) -> Vec<Stage>;

    /// Lists all currently used stages.
    ///
    /// # Returns
    ///
    /// a pointer
    ///  to the internal list of `Stage` objects. The returned list
    ///  is owned by the `StageManager` and should never be modified
    ///  or freed
    fn peek_stages(&self) -> Vec<Stage>;

    /// The ::stage-added signal is emitted each time a new `Stage`
    /// has been added to the stage manager.
    /// ## `stage`
    /// the added stage
    fn connect_stage_added<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> HandlerId;

    /// The ::stage-removed signal is emitted each time a `Stage`
    /// has been removed from the stage manager.
    /// ## `stage`
    /// the removed stage
    fn connect_stage_removed<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> HandlerId;

    fn connect_property_default_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId;
}

impl<O: Is<StageManager>> StageManagerExt for O {
    fn get_default_stage(&self) -> Option<Stage> {
        // unsafe {
        //     from_glib_none(ffi::clutter_stage_manager_get_default_stage(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn list_stages(&self) -> Vec<Stage> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_container(ffi::clutter_stage_manager_list_stages(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn peek_stages(&self) -> Vec<Stage> {
        // unsafe {
        //     FromGlibPtrContainer::from_glib_none(ffi::clutter_stage_manager_peek_stages(
        //         self.as_ref().to_glib_none().0,
        //     ))
        // }
        unimplemented!()
    }

    fn connect_stage_added<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_stage_removed<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }

    fn connect_property_default_stage_notify<F: Fn(&Self) + 'static>(&self, f: F) -> HandlerId {
        unimplemented!()
    }
}

impl fmt::Display for StageManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StageManager")
    }
}
