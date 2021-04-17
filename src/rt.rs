#![allow(clippy::missing_safety_doc)]

use std::cell::Cell;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Asserts that this is the main thread and either `clutter::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !crate::rt::is_initialized_main_thread() {
            if crate::rt::is_initialized() {
                panic!("GDK may only be used from the main thread.");
            } else {
                panic!("GDK has not been initialized. Call `clutter::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that neither `clutter::init` has been called.
macro_rules! assert_not_initialized {
    () => {
        if crate::rt::is_initialized() {
            panic!("This function has to be called before `clutter::init`.");
        }
    };
}

/// Returns `true` if GDK has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if GDK has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that GDK has been initialized and the current thread is the main one.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    } else if is_initialized() {
        panic!("Attempted to initialize GDK from two different threads.");
    }
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

pub fn init() {
    assert_not_initialized!();
    unsafe {
        ffi::clutter_init(ptr::null_mut(), ptr::null_mut());
        set_initialized();
    }
}

pub fn run() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::clutter_main();
        set_initialized();
    }
}

pub fn quit() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::clutter_main_quit();
    }
}
