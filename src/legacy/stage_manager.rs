use crate::Stage;
use glib::{
    object::{Cast, IsA},
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;
use std::{fmt, mem::transmute};

glib_wrapper! {
    pub struct StageManager(Object<ffi::ClutterStageManager, ffi::ClutterStageManagerClass, StageManagerClass>);

    match fn {
        get_type => || ffi::clutter_stage_manager_get_type(),
    }
}

impl StageManager {
    /// Returns the default `StageManager`.
    ///
    /// # Returns
    ///
    /// the default stage manager instance. The returned
    ///  object is owned by internals and you should not reference or unreference it.
    pub fn get_default() -> Option<StageManager> {
        unsafe { from_glib_none(ffi::clutter_stage_manager_get_default()) }
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
    fn connect_stage_added<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> SignalHandlerId;

    /// The ::stage-removed signal is emitted each time a `Stage`
    /// has been removed from the stage manager.
    /// ## `stage`
    /// the removed stage
    fn connect_stage_removed<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_stage_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<StageManager>> StageManagerExt for O {
    fn get_default_stage(&self) -> Option<Stage> {
        unsafe {
            from_glib_none(ffi::clutter_stage_manager_get_default_stage(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn list_stages(&self) -> Vec<Stage> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::clutter_stage_manager_list_stages(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn peek_stages(&self) -> Vec<Stage> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::clutter_stage_manager_peek_stages(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_stage_added<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stage_added_trampoline<P, F: Fn(&P, &Stage) + 'static>(
            this: *mut ffi::ClutterStageManager,
            stage: *mut ffi::ClutterStage,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StageManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &StageManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(stage),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stage-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stage_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_stage_removed<F: Fn(&Self, &Stage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stage_removed_trampoline<P, F: Fn(&P, &Stage) + 'static>(
            this: *mut ffi::ClutterStageManager,
            stage: *mut ffi::ClutterStage,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StageManager>,
        {
            let f: &F = &*(f as *const F);
            f(
                &StageManager::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(stage),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stage-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stage_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_default_stage_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_stage_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::ClutterStageManager,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<StageManager>,
        {
            let f: &F = &*(f as *const F);
            f(&StageManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-stage\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_stage_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StageManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StageManager")
    }
}
